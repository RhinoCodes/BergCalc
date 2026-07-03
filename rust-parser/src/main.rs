use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Number(f64),
    Add(Vec<Expr>),
    Mult(Vec<Expr>),
    Sub(Vec<Expr>),
    Div(Vec<Expr>),
    Pow(Vec<Expr>),
    Variable(char),
    Null(),
}

#[derive(Debug, Clone)]
enum StringOrVec {
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

    fn length(&self) -> usize {
        self.close - self.open
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
    println!("pair: {:?}", new_reconstructed);
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

fn tree(calc: &StringOrVec) -> Expr {
    let mut parsed = calc.clone();
    let order = vec!["^", "*/", "+-"];
    let index = 0;
    if let Some(v) = parsed.to_vec() {
        println!("v: {:?}", v);

        let operators = ["*", "+", "-", "/", "^"];
        let mut operator = ' ';
        let mut term = Expr::Null();
        for item in v.iter() {
            let mut t = Expr::Null();
            match item {
                StringOrVec::Single(s) => {
                    if operators.contains(&s.as_str()) {
                        operator = s.chars().next().unwrap();
                    } else {
                        if let Ok(num) = s.parse::<f64>() {
                            t = Expr::Number(num);
                        }
                    } /*else if s.len() == 1 && s.chars().next().unwrap().is_alphabetic() {
                    return Expr::Variable(s.chars().next().unwrap());
                    }*/
                }
                StringOrVec::Multiple(items) => {
                    t = tree(&StringOrVec::Multiple(items.clone()));
                }
            }
            if operator != ' ' && t != Expr::Null() {
                match operator {
                    '^' => term = Expr::Pow(vec![term, t]),
                    '*' => term = Expr::Mult(vec![term, t]),
                    '/' => term = Expr::Div(vec![term, t]),
                    '+' => term = Expr::Add(vec![term, t]),
                    '-' => term = Expr::Sub(vec![term, t]),
                    _ => panic!("Unhandled operator: {}", operator),
                }
                operator = ' ';
            } else if t != Expr::Null() {
                term = t;
            }
        }
        term
    } else {
        Expr::Null()
    }
}

fn parse(calc: &str) -> StringOrVec {
    let calc = String::from(calc);
    let operators = ["*", "+", "-", "/", "^", "(", ")"];
    let mut reconstructed: Vec<String> = vec!["".to_string()];
    for c in calc.chars() {
        if !operators.contains(&c.to_string().as_str()) {
            if let Some(last) = reconstructed.last_mut() {
                last.push(c);
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
        if reconstructed[term] == "".to_string() {
            reconstructed.remove(term);
        } else {
            term += 1;
        }
    }

    let mut pairs: Vec<usize> = Vec::new();
    let mut completedPairs: Vec<Pair> = Vec::new();
    for term in 0..reconstructed.len() {
        if reconstructed[term] == "(" {
            pairs.push(term);
        } else if reconstructed[term] == ")" {
            if let Some(openIndex) = pairs.pop() {
                completedPairs.push(Pair::new(openIndex, term));
            }
        }
    }

    let mut term = 0;
    while completedPairs.len() > 0 && term < completedPairs.len() - 1 {
        if completedPairs[term + 1].contains(&completedPairs[term]) {
            let child = completedPairs.remove(term);
            completedPairs[term].add_child(child);
            term = 0;
        } else {
            term += 1;
        }
        if !term < completedPairs.len() - 1 {
            break;
        }
    }
    // TODO: throw error if there are pairs left
    completedPairs.reverse();
    let mut new_reconstructed: Vec<StringOrVec> = Vec::new();
    for i in 0..reconstructed.len() {
        new_reconstructed.push(StringOrVec::Single(reconstructed[i].clone()));
    }
    for pair in completedPairs.iter_mut() {
        new_reconstructed = recursive_paren(&new_reconstructed, &pair);
    }

    StringOrVec::Multiple(new_reconstructed)
}

fn main() {
    let calc = "2+2*3";
    let result = parse(calc);
    println!("{}", result);
    println!("{:?}", tree(&result));
}
