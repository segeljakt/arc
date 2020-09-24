use {
    crate::{prelude::*, error::*, info::*, symbols::*},
    codespan::Span,
    ena::unify::{InPlace, UnifyKey, UnifyValue},
};

pub type UnificationTable = ena::unify::UnificationTable<InPlace<TypeVar>>;

pub struct Typer {
    table: UnificationTable,
}

/// Suppose we have the following arc-script code:
/// ---------------------------------------------
/// fun max(a: i32, b: i32) -> i32
///   c = a > b
///   if c then a else b
///
/// a = 5
/// b = 7
/// ---------------------------------------------
/// With type variables (some omitted):
/// ---------------------------------------------
/// fun max(a: 1→i32, b: 2→i32) -> 3→i32
///   c = a > b: 3→?
///   if c then a else b: 4→?
///
/// a = 5: 5→?
/// b = 7: 6→?
/// max(a,b): 7→?
/// ---------------------------------------------
/// With type constraints:
/// ---------------------------------------------
/// fun max(a: 1→i32, b: 2→i32) -> 3→i32
///   c = a > b: 3→bool
///   if c then a else b: 4→?
///
/// a = 5: 5→i32
/// b = 7: 6→i32
/// max(a,b): 7→(Fun(8→i32, 9→i32) -> 10→i32)
/// ---------------------------------------------
///
/// unify_var_val(7, Fun(8→i32, 9→i32) -> 10→8)
/// unify_var_val(7, Fun(1→i32, 2→i32) -> 3→i32)
/// unify_values(
///     Fun(8→i32, 9→i32) -> 10→8,
///     Fun(1→i32, 2→i32) -> 3→i32
/// )
///
/// unify_values(
///     Fun(8, 9) -> 10,
///     Fun(1, 2) -> 3
/// )
///
/// unify_var_value(1, Fun(2,3))
///   1 → Fun(2,3)

impl Typer {
    pub fn new() -> Typer {
        let table = UnificationTable::new();
        Typer { table }
    }
}

impl Typer {
    /// Unifies two type variables `a` and `b`.
    fn unify_var_var(
        &mut self,
        a: TypeVar,
        b: TypeVar,
        span: Span,
        errors: &mut Vec<CompilerError>,
    ) {
        let snapshot = self.table.snapshot();
        match self.table.unify_var_var(a, b) {
            Ok(()) => self.table.commit(snapshot),
            Err((lhs, rhs)) => {
                errors.push(CompilerError::TypeMismatch { lhs, rhs, span });
                self.table.rollback_to(snapshot)
            }
        }
    }

    /// Unifies a type variable `a` with a type `b`.
    fn unify_var_val<T>(&mut self, a: TypeVar, b: T, span: Span, errors: &mut Vec<CompilerError>)
    where
        T: Into<Type>,
    {
        let snapshot = self.table.snapshot();
        match self.table.unify_var_value(a, b.into()) {
            Ok(()) => self.table.commit(snapshot),
            Err((lhs, rhs)) => {
                errors.push(CompilerError::TypeMismatch { lhs, rhs, span });
                self.table.rollback_to(snapshot)
            }
        }
    }

    /// Returns a fresh type variable which is unified with the given type `ty`.
    pub fn intern<T: Into<Type>>(&mut self, ty: T) -> TypeVar {
        self.table.new_key(ty.into())
    }

    /// Returns a fresh type variable.
    pub fn fresh(&mut self) -> TypeVar {
        self.table.new_key(Type::new())
    }

    /// Returns the type which `tv` is unified with.
    pub fn lookup(&mut self, tv: TypeVar) -> Type {
        self.table.probe_value(tv)
    }
}

impl UnifyKey for TypeVar {
    type Value = Type;

    fn index(&self) -> u32 {
        let TypeVar(id) = *self;
        id
    }

    fn from_index(id: u32) -> TypeVar {
        TypeVar(id)
    }

    fn tag() -> &'static str {
        "Type"
    }
}

impl UnifyValue for Type {
    type Error = (Self, Self);

    /// Unifies two monotypes, i.e., types which contain no type variables
    /// Some logic is duplicated between mono- and poly-type unification which is
    /// maybe not ideal. The reason for why unify_values cannot do poly-type
    /// unification is it's not possible to access the Typer from within the function,
    /// so we cannot do recursive lookups for values of type variables. For this reason,
    /// all unification needs to be "top-level".
    fn unify_values(ty1: &Self, ty2: &Self) -> Result<Self, Self::Error> {
        if let (Array(_, sh1), Array(_, sh2)) = (&ty1.kind, &ty2.kind) {
            match (&sh1.dims, &sh2.dims) {
                (d1, _) if d1.iter().all(|dim| dim.is_val()) => Ok(ty1.clone()),
                (_, d2) if d2.iter().all(|dim| dim.is_val()) => Ok(ty2.clone()),
                _ => Err((ty1.clone(), ty2.clone())),
            }
        } else {
            match (&ty1.kind, &ty2.kind) {
                (Unknown, _) | (TypeErr, _) => Ok(ty2.clone()),
                (_, Unknown) | (_, TypeErr) => Ok(ty1.clone()),
                (a, b) => {
                    if a == b {
                        Ok(ty1.clone())
                    } else {
                        Err((ty1.clone(), ty2.clone()))
                    }
                }
            }
        }
    }
}

impl Typer {
    /// Unifies two polytypes, i.e., types which may contain type variables
    fn unify(&mut self, tv1: TypeVar, tv2: TypeVar, span: Span, errors: &mut Vec<CompilerError>) {
        let ty1 = self.lookup(tv1);
        let ty2 = self.lookup(tv2);
        match (&ty1.kind, &ty2.kind) {
            (Unknown, Unknown) => self.unify_var_var(tv1, tv2, span, errors),
            (Unknown, _) => self.unify_var_val(tv1, ty2, span, errors),
            (_, Unknown) => self.unify_var_val(tv2, ty1, span, errors),
            (Array(tv1, sh1), Array(tv2, sh2)) if sh1.dims.len() == sh2.dims.len() => {
                self.unify(*tv1, *tv2, span, errors);
            }
            (Fun(args1, ret1), Fun(args2, ret2)) if args1.len() == args2.len() => {
                for (arg1, arg2) in args1.into_iter().zip(args2.into_iter()) {
                    self.unify(*arg1, *arg2, span, errors);
                }
                self.unify(*ret1, *ret2, span, errors);
            }
            // This seems a bit out of place, but it is needed to ensure that monotypes unify
            _ => self.unify_var_var(tv1, tv2, span, errors),
        }
    }
}

impl Script<'_> {
    /// Infers the types of all type variables in a Script.
    pub fn infer(&mut self) {
        let Info {
            table,
            typer,
            errors,
            ..
        } = &mut self.info;
        let typer = typer.get_mut();
        self.ast
            .for_each_expr(|expr| expr.constrain(typer, errors, table));
        self.ast
            .fundefs
            .iter_mut()
            .for_each(|(id, fundef)| (id, fundef).constrain(typer, errors, table));
    }
}

trait Constrain {
    fn constrain(
        &mut self,
        typer: &mut Typer,
        errors: &mut Vec<CompilerError>,
        table: &SymbolTable,
    );
}

impl Constrain for (&Ident, &mut FunDef) {
    /// Constrains the types of a function based on its signature and body.
    fn constrain(
        &mut self,
        typer: &mut Typer,
        errors: &mut Vec<CompilerError>,
        table: &SymbolTable,
    ) {
        let (id, fundef) = self;
        let tv = table.get_decl(&id).tv;
        let ty = typer.lookup(tv);
        if let Fun(_, ret_tv) = ty.kind {
            typer.unify(fundef.body.tv, ret_tv, fundef.body.span, errors)
        }
    }
}

impl Constrain for Expr {
    /// Constrains an expression based on its subexpressions.
    fn constrain(
        &mut self,
        typer: &mut Typer,
        errors: &mut Vec<CompilerError>,
        table: &SymbolTable,
    ) {
        let span = self.span;
        match &self.kind {
            Let(id, v) => {
                let tv = table.get_decl(id).tv;
                typer.unify(v.tv, tv, span, errors);
                typer.unify_var_val(self.tv, Scalar(Unit), span, errors);
            }
            Var(id) => {
                let tv = table.get_decl(id).tv;
                typer.unify(self.tv, tv, span, errors);
            }
            Lit(l) => {
                let kind = match l {
                    LitI8(_) => I8,
                    LitI16(_) => I16,
                    LitI32(_) => I32,
                    LitI64(_) => I64,
                    LitF32(_) => F32,
                    LitF64(_) => F64,
                    LitBool(_) => Bool,
                    LitUnit => Unit,
                    LitTime(_) => todo!(),
                    LitErr => return,
                };
                typer.unify_var_val(self.tv, Scalar(kind), span, errors);
            }
            ConsArray(args) => {
                let elem_tv = typer.fresh();
                let dim = Dim::from(DimVal(args.len() as i32));
                args.iter()
                    .for_each(|e| typer.unify(elem_tv, e.tv, span, errors));
                let shape = Shape::from(vec![dim]);
                typer.unify_var_val(self.tv, Array(elem_tv, shape), span, errors);
            }
            ConsStruct(fields) => {
                let fields = fields
                    .iter()
                    .map(|(sym, e)| (*sym, e.tv))
                    .collect::<Vec<_>>();
                typer.unify_var_val(self.tv, Struct(fields), span, errors);
            }
            ConsTuple(args) => {
                let tvs = args.iter().map(|arg| arg.tv).collect();
                typer.unify_var_val(self.tv, Tuple(tvs), span, errors);
            }
            BinOp(l, kind, r) => match kind {
                Add | Div | Mul | Sub => {
                    typer.unify(l.tv, r.tv, span, errors);
                    typer.unify(self.tv, r.tv, span, errors)
                }
                Equ | Neq | Gt | Lt | Geq | Leq => {
                    typer.unify(l.tv, r.tv, span, errors);
                    typer.unify_var_val(self.tv, Scalar(Bool), span, errors)
                }
                Or | And => {
                    typer.unify(self.tv, l.tv, self.span, errors);
                    typer.unify(self.tv, r.tv, self.span, errors);
                    typer.unify_var_val(self.tv, Scalar(Bool), self.span, errors)
                }
                Pipe => todo!(),
                Seq => typer.unify(self.tv, r.tv, span, errors),
                BinOpErr => return,
            },
            UnOp(kind, e) => {
                match kind {
                    Not => {
                        typer.unify(self.tv, e.tv, span, errors);
                        typer.unify_var_val(e.tv, Scalar(Bool), span, errors);
                    }
                    Neg => typer.unify(self.tv, e.tv, span, errors),
                    Cast(tv) => typer.unify(e.tv, *tv, span, errors),
                    Project(_) => return,
                    Access(_) => return,
                    Call(args) => {
                        let params = args.iter().map(|arg| arg.tv).collect();
                        let tv2 = typer.intern(Fun(params, self.tv));
                        typer.unify(e.tv, tv2, span, errors);
                    }
                    UnOpErr => return,
                }
            }
            If(c, t, e) => {
                typer.unify_var_val(c.tv, Scalar(Bool), span, errors);
                typer.unify(t.tv, e.tv, span, errors);
                typer.unify(t.tv, self.tv, span, errors);
            }
            Closure(params, body) => {
                let params = params
                    .iter()
                    .map(|param| table.get_decl(param).tv)
                    .collect();
                let tv = typer.intern(Fun(params, body.tv));
                typer.unify(self.tv, tv, span, errors)
            }
            Match(_, _) => {}
            Sink(_) => todo!(),
            Source(_) => todo!(),
            Loop(_, _) => todo!(),
            ExprErr => return,
        }
    }
}
