use crate::compiler::hir::infer::unify::Unify;
use crate::compiler::hir::utils::SortFields;
use crate::compiler::hir::{
    self, BinOpKind, Dim, DimKind, Expr, ExprKind, Extern, Fun, Item, ItemKind, LitKind, Param,
    ParamKind, Path, ScalarKind, Shape, State, Task, Type, TypeKind, UnOpKind, HIR,
};
use crate::compiler::info::diags::{Diagnostic, Error, Warning};
use crate::compiler::info::types::TypeId;

use arc_script_core_shared::itertools::Itertools;
use arc_script_core_shared::map;
use arc_script_core_shared::VecMap;

use super::Context;

use arc_script_core_shared::get;

pub(crate) trait Constrain<'i> {
    fn constrain(&self, ctx: &mut Context<'i>);
}

impl Constrain<'_> for Item {
    #[rustfmt::skip]
    fn constrain(&self, ctx: &mut Context<'_>) {
        match &self.kind {
            ItemKind::Fun(item)   => item.constrain(ctx),
            ItemKind::Task(item)  => item.constrain(ctx),
            ItemKind::Alias(_)    => {}
            ItemKind::Enum(_)     => {}
            ItemKind::Extern(_)   => {}
            ItemKind::Variant(_)  => {}
        }
    }
}

impl Constrain<'_> for Fun {
    fn constrain(&self, ctx: &mut Context<'_>) {
        let tvs = self.params.iter().map(|x| x.tv).collect();
        let tv = self.body.tv;
        ctx.unify(self.tv, TypeKind::Fun(tvs, tv));
        ctx.unify(self.rtv, tv);
        for p in &self.params {
            if let ParamKind::Var(x) = &p.kind {
                ctx.env.insert(*x, *p);
            }
        }
        self.body.constrain(ctx);
    }
}

impl Constrain<'_> for Task {
    /// On the outside, a task is nothing more than a function which returns a function.
    fn constrain(&self, ctx: &mut Context<'_>) {
        // Key-type of this task
        let key_tv = ctx.info.types.fresh();
        // Constrain parameters
        self.params.iter().for_each(|p| {
            if let ParamKind::Var(x) = &p.kind {
                ctx.env.insert(*x, *p);
            }
        });
        // Constrain input/output streams
        let (key_tvs, istream_tvs) = self.ihub.constrain(ctx);
        key_tvs.iter().for_each(|t| ctx.unify(*t, key_tv));
        let (_, ostream_tvs) = self.ohub.constrain(ctx);
        // Constrain state-variables
        self.states.iter().for_each(|s| {
            s.init.constrain(ctx);
            ctx.unify(s.param.tv, s.init.tv);
            if let ParamKind::Var(x) = &s.param.kind {
                ctx.env.insert(*x, s.param);
            }
        });
        // Constrain startup statements
        self.startups.iter().for_each(|s| s.expr.constrain(ctx));
        // Constrain event handler
        if let Some(on) = &self.on {
            if let ParamKind::Var(x) = &on.param.kind {
                ctx.env.insert(*x, on.param);
            }
            ctx.unify(on.param.tv, self.ihub.internal_tv);
            on.body.constrain(ctx);
        }
        // Constrain timer and timeout handler
        if let (Some(timer), Some(timeout)) = (&self.timer, &self.timeout) {
            if let ParamKind::Var(x) = &timeout.param.kind {
                ctx.env.insert(*x, timeout.param);
            }
            let val_name = ctx.info.names.common.val.into();
            let key_name = ctx.info.names.common.key.into();
            let val_tv = timer.tv;
            let tv = ctx
                .info
                .types
                .intern(vec![(key_name, key_tv), (val_name, val_tv)]);
            ctx.unify(tv, timeout.param.tv);
            timeout.body.constrain(ctx);
        }
        self.items.iter().for_each(|item| {
            let item = ctx.hir.defs.get(item).unwrap();
            item.constrain(ctx)
        });
        let tvs = self.params.iter().map(|x| x.tv).collect();
        let otv = match ostream_tvs.len() {
            0 => ctx.info.types.intern(ScalarKind::Unit),
            1 => ostream_tvs[0],
            _ => ctx.info.types.intern(TypeKind::Tuple(ostream_tvs)),
        };
        let ttv = ctx.info.types.intern(TypeKind::Fun(istream_tvs, otv));
        ctx.unify(self.tv, TypeKind::Fun(tvs, ttv));
    }
}

impl hir::Hub {
    /// Constrains the internal port types of a hub and returns the key-types
    /// and stream-types
    fn constrain(&self, ctx: &mut Context<'_>) -> (Vec<TypeId>, Vec<TypeId>) {
        let val_name = ctx.info.names.common.val.into();
        let key_name = ctx.info.names.common.key.into();
        match &self.kind {
            hir::HubKind::Tagged(x) => {
                ctx.unify(self.internal_tv, TypeKind::Nominal(*x));
                let item = ctx.hir.defs.get(x).unwrap();
                let item = get!(&item.kind, hir::ItemKind::Enum(x));
                item.variants
                    .iter()
                    .map(|x| {
                        let item = ctx.hir.defs.get(x).unwrap();
                        let stream_tv = get!(&item.kind, hir::ItemKind::Variant(x)).tv;
                        let key_tv = ctx.info.types.fresh();
                        let val_tv = ctx.info.types.fresh();
                        let event_tv = ctx
                            .info
                            .types
                            .intern(vec![(key_name, key_tv), (val_name, val_tv)]);
                        ctx.unify(stream_tv, hir::TypeKind::Stream(event_tv));
                        (key_tv, stream_tv)
                    })
                    .unzip()
            }
            hir::HubKind::Single(stream_tv) => {
                let key_tv = ctx.info.types.fresh();
                let val_tv = ctx.info.types.fresh();
                let event_tv = ctx
                    .info
                    .types
                    .intern(vec![(key_name, key_tv), (val_name, val_tv)]);
                ctx.unify(self.internal_tv, event_tv);
                ctx.unify(*stream_tv, hir::TypeKind::Stream(event_tv));
                (vec![key_tv], vec![*stream_tv])
            }
        }
    }
}

impl Constrain<'_> for State {
    fn constrain(&self, ctx: &mut Context<'_>) {}
}

impl Constrain<'_> for Expr {
    /// Constrains an expression based on its subexpressions.
    fn constrain(&self, ctx: &mut Context<'_>) {
        use BinOpKind::*;
        use ScalarKind::*;
        let loc = ctx.loc;
        ctx.loc = self.loc;
        match &self.kind {
            ExprKind::Let(p, e0, e1) => {
                if let ParamKind::Var(x) = p.kind {
                    ctx.env.insert(x, *p);
                    ctx.unify(p.tv, e0.tv);
                    ctx.unify(self.tv, e1.tv);
                    e0.constrain(ctx);
                    e1.constrain(ctx);
                }
            }
            ExprKind::Var(x, _) => ctx.unify(self.tv, ctx.env.get(x).unwrap().tv),
            #[rustfmt::skip]
            ExprKind::Item(x) => {
                match &ctx.hir.defs.get(x).unwrap().kind {
                    ItemKind::Fun(item)    => ctx.unify(self.tv, item.tv),
                    ItemKind::Task(item)   => ctx.unify(self.tv, item.tv),
                    ItemKind::Extern(item) => ctx.unify(self.tv, item.tv),
                    ItemKind::Alias(_)     => unreachable!(),
                    ItemKind::Enum(_)      => unreachable!(),
                    ItemKind::Variant(_)   => unreachable!(),
                }
            },
            #[rustfmt::skip]
            ExprKind::Lit(kind) => {
                let kind = match kind {
                    LitKind::I8(_)   => I8,
                    LitKind::I16(_)  => I16,
                    LitKind::I32(_)  => I32,
                    LitKind::I64(_)  => I64,
                    LitKind::U8(_)   => U8,
                    LitKind::U16(_)  => U16,
                    LitKind::U32(_)  => U32,
                    LitKind::U64(_)  => U64,
                    LitKind::Bf16(_) => Bf16,
                    LitKind::F16(_)  => F16,
                    LitKind::F32(_)  => F32,
                    LitKind::F64(_)  => F64,
                    LitKind::Bool(_) => Bool,
                    LitKind::Unit    => Unit,
                    LitKind::DateTime(_) => DateTime,
                    LitKind::Duration(_) => Duration,
                    LitKind::Char(_) => Char,
                    LitKind::Str(_)  => Str,
                    LitKind::Err     => return,
                };
                ctx.unify(self.tv, kind);
            }
            ExprKind::Array(es) => {
                let elem_tv = ctx.info.types.fresh();
                let dim = Dim::new(DimKind::Val(es.len() as i32));
                es.iter().for_each(|e| ctx.unify(elem_tv, e.tv));
                let shape = Shape::new(vec![dim]);
                ctx.unify(self.tv, TypeKind::Array(elem_tv, shape));
                es.constrain(ctx);
            }
            // NOTE: We sort fields-types by field name.
            ExprKind::Struct(fs) => {
                fs.constrain(ctx);
                let fs = fs
                    .iter()
                    .map(|(x, e)| (*x, e.tv))
                    .collect::<VecMap<_, _>>()
                    .sort_fields(ctx.info);
                ctx.unify(self.tv, TypeKind::Struct(fs));
            }
            ExprKind::Enwrap(x0, e) => {
                e.constrain(ctx);
                let item = &ctx.hir.defs.get(x0).unwrap().kind;
                let item = get!(item, ItemKind::Variant(x));
                let x1 = ctx.info.paths.resolve(x0.id).pred.unwrap().into();
                ctx.unify(e.tv, item.tv);
                ctx.unify(self.tv, TypeKind::Nominal(x1));
            }
            ExprKind::Unwrap(x0, e) => {
                e.constrain(ctx);
                let item = ctx.hir.defs.get(x0).unwrap();
                let item = get!(&item.kind, ItemKind::Variant(x));
                let x1 = ctx.info.paths.resolve(x0.id).pred.unwrap().into();
                ctx.unify(e.tv, TypeKind::Nominal(x1));
                ctx.unify(self.tv, item.tv);
            }
            ExprKind::Is(x0, e) => {
                e.constrain(ctx);
                let x1 = ctx.info.paths.resolve(x0.id).pred.unwrap().into();
                ctx.unify(e.tv, TypeKind::Nominal(x1));
                ctx.unify(self.tv, Bool);
            }
            ExprKind::Tuple(es) => {
                let tvs = es.iter().map(|arg| arg.tv).collect();
                ctx.unify(self.tv, TypeKind::Tuple(tvs));
                es.constrain(ctx);
            }
            ExprKind::BinOp(e0, op, e1) => {
                e0.constrain(ctx);
                e1.constrain(ctx);
                match &op.kind {
                    Add | Div | Mul | Sub | Mod => {
                        ctx.unify(e0.tv, e1.tv);
                        ctx.unify(self.tv, e1.tv);
                    }
                    Pow => {
                        ctx.unify(self.tv, e0.tv);
                        if let TypeKind::Scalar(kind) = ctx.info.types.resolve(e0.tv).kind {
                            match kind {
                                I8 | I16 | I32 | I64 => ctx.unify(e1.tv, I32),
                                F32 => ctx.unify(e1.tv, F32),
                                F64 => ctx.unify(e1.tv, F64),
                                _ => {}
                            }
                        }
                        if let TypeKind::Scalar(kind) = ctx.info.types.resolve(e1.tv).kind {
                            match kind {
                                F32 => ctx.unify(e0.tv, F32),
                                F64 => ctx.unify(e0.tv, F64),
                                _ => {}
                            }
                        }
                    }
                    Equ | Neq | Gt | Lt | Geq | Leq => {
                        ctx.unify(e0.tv, e1.tv);
                        ctx.unify(self.tv, Bool);
                    }
                    Or | And | Xor => {
                        ctx.unify(self.tv, e0.tv);
                        ctx.unify(self.tv, e1.tv);
                        ctx.unify(self.tv, Bool);
                    }
                    Band | Bor | Bxor => {
                        ctx.unify(e0.tv, e1.tv);
                        ctx.unify(self.tv, e1.tv);
                    }
                    Pipe => unreachable!(),
                    Mut => {
                        ctx.unify(self.tv, Unit);
                        ctx.unify(e0.tv, e1.tv);
                    }
                    Seq => ctx.unify(self.tv, e1.tv),
                    In => ctx.unify(self.tv, Bool),
                    NotIn => ctx.unify(self.tv, Bool),
                    BinOpKind::Err => {}
                    After => unreachable!(),
                    By => unreachable!(),
                    Err => {}
                }
            }
            ExprKind::UnOp(op, e0) => match &op.kind {
                UnOpKind::Boxed => {
                    e0.constrain(ctx);
                    ctx.unify(self.tv, TypeKind::Boxed(e0.tv))
                }
                UnOpKind::Not => {
                    e0.constrain(ctx);
                    ctx.unify(self.tv, e0.tv);
                    ctx.unify(e0.tv, Bool);
                }
                UnOpKind::Neg => ctx.unify(self.tv, e0.tv),
                _ => unreachable!(),
            },
            ExprKind::Call(e, es) => {
                e.constrain(ctx);
                es.constrain(ctx);
                let tvs = es.iter().map(|e| e.tv).collect();
                ctx.unify(e.tv, TypeKind::Fun(tvs, self.tv));
            }
            ExprKind::Select(e0, es) => match es.as_slice() {
                [e1] => {
                    e0.constrain(ctx);
                    e1.constrain(ctx);
                    ctx.unify(e0.tv, TypeKind::Map(e1.tv, self.tv));
                }
                _ => crate::todo!("Probably support arrow-tables?"),
            },
            ExprKind::Project(e, i) => {
                e.constrain(ctx);
                if let TypeKind::Tuple(tvs) = ctx.info.types.resolve(e.tv).kind {
                    if let Some(tv) = tvs.get(i.id) {
                        ctx.unify(self.tv, *tv);
                    } else {
                        ctx.info
                            .diags
                            .intern(Error::OutOfBoundsProject { loc: self.loc })
                    }
                }
            }
            ExprKind::Access(e, x) => {
                e.constrain(ctx);
                if let TypeKind::Struct(fs) = ctx.info.types.resolve(e.tv).kind {
                    if let Some(tv) = fs.get(x) {
                        ctx.unify(self.tv, *tv);
                    } else {
                        ctx.info
                            .diags
                            .intern(Error::FieldNotFound { loc: self.loc })
                    }
                }
            }
            ExprKind::Emit(e0) => {
                let key_name = ctx.info.names.common.key.into();
                let val_name = ctx.info.names.common.val.into();
                let val_tv = ctx.info.types.fresh();
                let key_tv = ctx.info.types.fresh();
                let tv0 = ctx
                    .info
                    .types
                    .intern(vec![(key_name, key_tv), (val_name, val_tv)]);
                ctx.unify(self.tv, Unit);
                e0.constrain(ctx);
            }
            ExprKind::Trigger(e0) => {
                let key_name = ctx.info.names.common.key.into();
                let val_name = ctx.info.names.common.val.into();
                let dur_name = ctx.info.names.common.dur.into();
                let key_tv = ctx.info.types.fresh();
                let val_tv = ctx.info.types.fresh();
                let dur_tv = ctx.info.types.intern(Duration);
                let tv0 = ctx
                    .info
                    .types
                    .intern(vec![(key_name, key_tv), (val_name, val_tv)]);
                let tv1 = ctx
                    .info
                    .types
                    .intern(vec![(dur_name, dur_tv), (val_name, tv0)]);
                ctx.unify(self.tv, Unit);
                e0.constrain(ctx);
            }
            ExprKind::Log(e) => {
                ctx.unify(self.tv, Unit);
                e.constrain(ctx);
            }
            ExprKind::If(e0, e1, e2) => {
                ctx.unify(e0.tv, Bool);
                ctx.unify(e1.tv, e2.tv);
                ctx.unify(e1.tv, self.tv);
                e0.constrain(ctx);
                e1.constrain(ctx);
                e2.constrain(ctx);
            }
            ExprKind::Loop(_) => crate::todo!(),
            ExprKind::Break => ctx.unify(self.tv, Unit),
            ExprKind::Return(_) => ctx.unify(self.tv, Unit),
            ExprKind::Empty => {}
            ExprKind::Todo => {}
            ExprKind::Add(e0, e1) => {
                e0.constrain(ctx);
                e1.constrain(ctx);
                ctx.unify(e0.tv, hir::TypeKind::Set(e1.tv));
                ctx.unify(self.tv, Unit);
            }
            ExprKind::Del(e0, e1) => {
                e0.constrain(ctx);
                e1.constrain(ctx);
                match ctx.info.types.resolve(e0.tv).kind {
                    hir::TypeKind::Map(tv, _) => ctx.unify(tv, e1.tv),
                    hir::TypeKind::Set(tv) => ctx.unify(tv, e1.tv),
                    hir::TypeKind::Unknown => unknown_err(ctx),
                    _ => ctx
                        .info
                        .diags
                        .intern(Error::ExpectedSelectableType { loc: ctx.loc }),
                }
                ctx.unify(self.tv, Unit);
            }
            ExprKind::Err => {}
        }
        ctx.loc = loc;
    }
}

fn unknown_err(ctx: &mut Context<'_>) {
    ctx.info
        .diags
        .intern(Error::TypeMustBeKnownAtThisPoint { loc: ctx.loc });
}

impl Constrain<'_> for Vec<Expr> {
    fn constrain(&self, ctx: &mut Context<'_>) {
        self.iter().for_each(|e| e.constrain(ctx))
    }
}

impl<T: Eq> Constrain<'_> for VecMap<T, Expr> {
    fn constrain(&self, ctx: &mut Context<'_>) {
        self.values().for_each(|e| e.constrain(ctx))
    }
}
