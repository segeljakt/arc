use crate::{ast::*, utils::*};
use ExprKind::*;

type Context = Vec<(Ident, Expr)>;

trait Ssa {
    fn flatten(self) -> (Context, Self);
}

impl Expr {
    /// Reverse-folds a vec of SSA values into an AST of let-expressions
    pub fn into_ssa(self) -> Expr {
        let (ctx, var) = self.flatten();
        ctx.into_iter().rev().fold(var, |acc, (id, expr)| Expr {
            ty: acc.ty.clone(),
            span: acc.span,
            kind: Let(id, acc.ty.clone(), Box::new(expr), Box::new(acc)),
        })
    }
}
impl Ssa for Expr {
    /// Turns an expression into a flat list of assignments and a variable
    fn flatten(self) -> (Context, Expr) {
        let Expr { kind, ty, span } = self;
        let (mut ctx, kind) = match kind {
            BinOp(l, op, r) => {
                let ((lc, l), (rc, r)) = (l.flatten(), r.flatten());
                (merge(lc, rc), BinOp(Box::new(l), op, Box::new(r)))
            }
            UnOp(op, e) => {
                let (ec, e) = e.flatten();
                (ec, UnOp(op, Box::new(e)))
            }
            If(c, t, e) => {
                let ((cc, c), t, e) = (c.flatten(), t.into_ssa(), e.into_ssa());
                (cc, If(Box::new(c), Box::new(t), Box::new(e)))
            }
            FunCall(id, a) => {
                let (ac, a) = a.flatten();
                (ac, FunCall(id, a))
            }
            Let(id, _, v, b) => {
                let (mut vc, v) = match v.is_imm() {
                    true => (vec![], *v),
                    false => v.flatten(),
                };
                let (bc, b) = b.flatten();
                vc.push((id, v));
                return (merge(vc, bc), b);
            }
            ConsArray(e) => {
                let (ec, e) = e.flatten();
                (ec, ConsArray(e))
            }
            ConsStruct(a) => {
                let (ac, a) = a.flatten();
                (ac, ConsStruct(a))
            }
            kind @ Var(_) => return (vec![], Expr { kind, ty, span }),
            kind => (vec![], kind),
        };
        let id = Ident::gen();
        let var = Self::new(Var(id.clone()), ty.clone(), span);
        let expr = Self { kind, ty, span };
        ctx.push((id, expr));
        (ctx, var)
    }
}

impl Expr {
    /// Returns true if an expression has no subexpressions
    fn is_imm(&self) -> bool {
        match &self.kind {
            ExprKind::Var(_) => true,
            ExprKind::Lit(_) => true,
            _ => false,
        }
    }
}

impl Ssa for Vec<Expr> {
    fn flatten(self) -> (Context, Self) {
        let mut ctx = vec![];
        let exprs = self
            .into_iter()
            .map(|e| {
                let (mut ec, e) = e.flatten();
                ctx.append(&mut ec);
                e
            })
            .collect();
        (ctx, exprs)
    }
}

impl Ssa for Vec<(Ident, Expr)> {
    fn flatten(self) -> (Context, Self) {
        let mut ctx = vec![];
        let exprs = self
            .into_iter()
            .map(|(id, e)| {
                let (mut ec, e) = e.flatten();
                ctx.append(&mut ec);
                (id, e)
            })
            .collect();
        (ctx, exprs)
    }
}
