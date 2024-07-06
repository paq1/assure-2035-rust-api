#[derive(Clone, Debug)]
pub enum Filter {
    Expr(Expr),
    None
}

#[derive(Clone, Debug)]
pub enum Expr {
    ExprStr(ExprGeneric<String>)
}

#[derive(Clone, Debug)]
pub struct ExprGeneric<T> {
    pub field: String,
    pub operation: Operation,
    pub head: T
}

#[derive(Clone, Debug)]
pub enum Operation {
    EqualsTo
}
