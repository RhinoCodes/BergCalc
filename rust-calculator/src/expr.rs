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
    Function(String, Box<Expr>),
}

impl Expr {
    pub fn is_evaluable(&self) -> bool {
        !self.contains_variable()
    }

    fn contains_variable(&self) -> bool {
        match self {
            Expr::Number(_) => false,
            Expr::Null() => false,
            Expr::Variable(_) => true,
            Expr::Add(exprs)
            | Expr::Mult(exprs)
            | Expr::Sub(exprs)
            | Expr::Div(exprs)
            | Expr::Pow(exprs) => exprs.iter().any(|e| e.contains_variable()),
            Expr::Negate(inner) => inner.contains_variable(),
            Expr::Function(_, inner) => inner.contains_variable(),
        }
    }
}
