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

// <ai>
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Variable(c) => write!(f, "{}", c),
            Expr::Null() => write!(f, "null"),
            Expr::Negate(inner) => write!(f, "-{}", inner),
            Expr::Function(name, inner) => write!(f, "{}({})", name, inner),
            Expr::Add(exprs) => write_joined(f, exprs, " + "),
            Expr::Sub(exprs) => write_joined(f, exprs, " - "),
            Expr::Mult(exprs) => write_joined(f, exprs, " * "),
            Expr::Div(exprs) => write_joined(f, exprs, " / "),
            Expr::Pow(exprs) => write_joined(f, exprs, "^"),
        }
    }
}

fn write_joined(f: &mut fmt::Formatter<'_>, exprs: &[Expr], sep: &str) -> fmt::Result {
    write!(f, "(")?;
    for (i, e) in exprs.iter().enumerate() {
        if i > 0 {
            write!(f, "{}", sep)?;
        }
        write!(f, "{}", e)?;
    }
    write!(f, ")")
}
// </ai>