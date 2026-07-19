use crate::expr::Expr;
use core::fmt;
use core::fmt::{Debug};
use alloc::vec;       // brings in the vec! macro
use alloc::vec::Vec;  // brings in the Vec type
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;

#[derive(Debug, Clone)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<StringOrVec>),
}

impl StringOrVec {
    fn to_vec(&self) -> Option<&Vec<StringOrVec>> {
        match self {
            StringOrVec::Multiple(v) => Some(v),
            StringOrVec::Single(_) => None,
        }
    }
}

#[derive(Debug)]
struct Pair {
    open: usize,
    close: usize,
    children: Vec<Pair>,
}

impl Pair {
    fn new(open: usize, close: usize) -> Self {
        Pair {
            open,
            close,
            children: Vec::new(),
        }
    }

    fn contains(&self, other: &Pair) -> bool {
        self.open < other.open && self.close > other.close
    }

    fn add_child(&mut self, child: Pair) {
        let s = child.close - self.open - 1;
        let mut new_pair = Pair::new(child.open - self.open - 1, s);
        new_pair.children = child.children;
        self.children.push(new_pair);
    }
}

//<ai>
impl fmt::Display for StringOrVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StringOrVec::Single(s) => write!(f, "{}", s),
            StringOrVec::Multiple(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}
//</ai>
fn recursive_paren(reconstructed: &Vec<StringOrVec>, pair: &Pair) -> Vec<StringOrVec> {
    let mut new_reconstructed: Vec<StringOrVec> = reconstructed.clone();
    let mut pair_list = new_reconstructed[pair.open + 1..pair.close].to_vec();
    //if pair.close != -1 {
    new_reconstructed.drain(pair.open..pair.close + 1);
    /*} else {
        new_reconstructed.drain(pair.open..pair.close);
        new_reconstructed.drain(new_reconstructed.len() - 1..new_reconstructed.len());
    }*/
    for child in pair.children.iter() {
        let pairlist = recursive_paren(&pair_list, &child);
        pair_list = pairlist.clone();
    }
    new_reconstructed.insert(pair.open, StringOrVec::Multiple(pair_list.to_vec()));
    new_reconstructed
}

fn precendence(op: char) -> u8 {
    match op {
        '^' => 3,
        '~' | '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

fn push_operand(operands: &mut Vec<Expr>, operand: Expr) {
    /*if operators.last() == Some(&'~') {
        operators.pop();
        operands.push(Expr::Negate(Box::new(operand)));
    } else {*/
    operands.push(operand);
    //}
}

pub fn tree(calc: &StringOrVec) -> Expr {
    let parsed = calc.clone();
    if let Some(v) = parsed.to_vec() {
        let operators = ["*", "+", "-", "/", "^"];
        let mut operators_found: Vec<char> = Vec::new();
        let mut operands: Vec<Expr> = Vec::new();
        let mut expecting_operand = true;
        let mut is_function = false;
        let mut function_name = "";
        let mut i = 0;
        for item in v.iter() {
            match item {
                StringOrVec::Single(s) => {
                    if operators.contains(&s.as_str()) {
                        let op = s.chars().next().unwrap();
                        if op == '-' && expecting_operand {
                            operators_found.push('~');
                        } else {
                            operators_found.push(op);
                            expecting_operand = true;
                        }
                    } else if let Ok(num) = s.parse::<f64>() {
                        push_operand(&mut operands, Expr::Number(num));
                        expecting_operand = false;
                    } else if s.chars().count() == 1 && !s.chars().next().unwrap().is_numeric() {
                        expecting_operand = false;
                        push_operand(&mut operands, Expr::Variable(s.chars().next().unwrap()));
                    } else if s.chars().all(|c| c.is_alphabetic()) {
                        let functions = ["sin", "cos", "tan", "ln", "log", "csc", "sec", "cot", "exp", "abs", "ddx"];
                        if functions.contains(&s.as_str()) && i != v.len() {
                            is_function = true;
                            function_name = s;
                        } else {
                            for char in s.chars() {
                                push_operand(&mut operands, Expr::Variable(char));
                            }
                        }
                        
                    }
                }
                StringOrVec::Multiple(items) => {
                    expecting_operand = false;
                    if is_function {
                        push_operand(
                            &mut operands,
                            Expr::Function(
                                function_name.to_string(),
                                Box::from(tree(&StringOrVec::Multiple(items.clone()))),
                            ),
                        );
                        is_function = false;
                    } else {
                        push_operand(&mut operands, tree(&StringOrVec::Multiple(items.clone())));
                    }
                }
            }
            i += 1;
        }
        for &tier in &[3u8, 2u8, 1u8] {
            let mut i = 0;
            while i < operators_found.len() {
                let op = operators_found[i];
                if precendence(op) == tier {
                    if op != '~' {
                        let mut adj = 0;
                        for j in 0..i {
                            if operators_found[j] == '~' {
                                adj += 1;
                            }
                        }
                        let left = operands.remove(i - adj);
                        let right = operands.remove(i - adj);

                        let new_expr = match op {
                            '+' => Expr::Add(vec![left, right]),
                            '-' => Expr::Sub(vec![left, right]),
                            '*' => Expr::Mult(vec![left, right]),
                            '/' => Expr::Div(vec![left, right]),
                            '^' => {
                                if operators_found.len() != i + 1 && operators_found[i + 1] == '~' {
                                    operators_found.remove(i + 1);
                                    if left == Expr::Variable('e') {
                                        Expr::Function(
                                            "exp".to_string(),
                                            Box::from(Expr::Negate(Box::from(right))),
                                        )
                                    } else if right.is_evaluable() {
                                        Expr::Pow(vec![left, Expr::Negate(Box::from(right))])
                                    } else {
                                        Expr::Function("exp".to_string(), Box::from(Expr::Negate(Box::from(Expr::Mult(vec![
                                            Expr::Function("ln".to_string(), Box::from(left)),
                                            right
                                        ])))))
                                    }
                                } else {
                                    if left == Expr::Variable('e') {
                                        Expr::Function("exp".to_string(), Box::from(right))
                                    } else if right.is_evaluable() {
                                        Expr::Pow(vec![left, right])
                                    } else {
                                        Expr::Function("exp".to_string(), Box::from(Expr::Mult(vec![
                                            Expr::Function("ln".to_string(), Box::from(left)),
                                            right
                                        ])))
                                    }
                                }
                            }
                            _ => panic!("Unknown operator: {}", op),
                        };
                        operands.insert(i - adj, new_expr);
                        operators_found.remove(i);
                    } else {
                        let o = operands.remove(i);
                        operands.insert(i, Expr::Negate(Box::from(o)));
                        operators_found.remove(i);
                    }
                } else {
                    i += 1;
                }
            }
        }
        operands.into_iter().next().unwrap_or(Expr::Null())
    } else {
        Expr::Null()
    }
}

pub fn parse(calc: &str) -> StringOrVec {
    let calc = String::from(calc).replace("pi", "π");
    let operators = ["*", "+", "-", "/", "^", "(", ")"];
    let mut reconstructed: Vec<String> = vec!["".to_string()];

    for c in calc.chars() {
        if !operators.contains(&c.to_string().as_str()) {
            if !c.is_alphabetic() {
                if let Some(last) = reconstructed.last_mut() {
                    if c != ' ' {
                        last.push(c);
                    }
                }
            } else {
                if let Some(last) = reconstructed.last_mut() {
                    if last.chars().all(|c| c.is_alphabetic()) {
                        last.push(c);
                    } else {
                        reconstructed.push(c.to_string());
                    }
                }
            }
        } else {
            reconstructed.push(c.to_string());
            reconstructed.push("".to_string());
        }
    }

    let mut term = 0;
    loop {
        if term >= reconstructed.len() {
            break;
        }
        if reconstructed[term] == "".to_string() || reconstructed[term] == " ".to_string() {
            reconstructed.remove(term);
        } else {
            term += 1;
        }
    }
    let op = ["*", "+", "-", "/", "^"];
    let mut term = 0usize;
    loop {
        if term >= reconstructed.len() - 1 {
            break;
        }
        if !op.contains(&reconstructed[term].as_str())
            && !reconstructed[term].chars().all(|c| c.is_alphabetic())
        {
            if !op.contains(&reconstructed[term + 1].as_str()) {
                if &reconstructed[term] != "(" && &reconstructed[term + 1] != ")" {
                    if term >= 1 && reconstructed[term - 1] == "^" {
                        reconstructed.insert(term + 2, ")".to_string());
                        reconstructed.insert(term + 1, "*".to_string());
                        reconstructed.insert(term, "(".to_string());
                        term += 3;
                    } else {
                        reconstructed.insert(term + 1, "*".to_string());
                    }
                    term += 1;
                }
            }
        }
        term += 1;
    }
    let mut pairs: Vec<usize> = Vec::new();
    let mut completed_pairs: Vec<Pair> = Vec::new();
    for term in 0..reconstructed.len() {
        if reconstructed[term] == "(" {
            pairs.push(term);
        } else if reconstructed[term] == ")" {
            if let Some(open_index) = pairs.pop() {
                completed_pairs.push(Pair::new(open_index, term));
            }
        }
    }

    term = 0;
    while completed_pairs.len() > 0 && term < completed_pairs.len() - 1 {
        if completed_pairs[term + 1].contains(&completed_pairs[term]) {
            let child = completed_pairs.remove(term);
            completed_pairs[term].add_child(child);
            term = 0;
        } else {
            term += 1;
        }
        if !term < completed_pairs.len() - 1 {
            break;
        }
    }
    // TODO: throw error if there are pairs left
    completed_pairs.reverse();
    let mut new_reconstructed: Vec<StringOrVec> = Vec::new();
    for i in 0..reconstructed.len() {
        new_reconstructed.push(StringOrVec::Single(reconstructed[i].clone()));
    }
    for pair in completed_pairs.iter_mut() {
        new_reconstructed = recursive_paren(&new_reconstructed, &pair);
    }
    StringOrVec::Multiple(new_reconstructed)
}
