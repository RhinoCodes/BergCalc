#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Add(Vec<Expr>),
    Mult(Vec<Expr>),
    Sub(Vec<Expr>),
    Div(Vec<Expr>),
    Pow(Vec<Expr>),
    Variable(char),
    Null(),
    Negate(Box<Expr>),
    Function(String, Box<Expr>)
}