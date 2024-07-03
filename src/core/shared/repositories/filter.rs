#[derive(Clone)]
pub enum Filter {
    Expr(Expr),
    None
}

#[derive(Clone)]
pub enum Expr {
    ExprStr(ExprGeneric<String>)
}

#[derive(Clone)]
pub struct ExprGeneric<T> {
    pub field: String,
    pub operation: Operation,
    pub head: T
}

#[derive(Clone)]
pub enum Operation {
    EqualsTo
}
