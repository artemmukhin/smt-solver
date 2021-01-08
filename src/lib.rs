use std::iter::Chain;
use std::fmt::{Display, Formatter};
use std::fmt;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Term {
    pub name: String,
    pub arguments: Vec<Box<Term>>,
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // non-empty arguments
        if let Some((last, elements)) = self.arguments.split_last() {
            write!(f, "{}(", self.name)?;
            for arg in elements {
                arg.fmt(f)?;
                write!(f, ",")?;
            }
            last.fmt(f)?;
            write!(f, ")")
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Term {
    // TODO: Should parse "f(g(a, b))" to `Symbol { "f", [ Symbol { "g", ["a", "b"] } ] }`
    #[allow(dead_code)]
    fn from(_raw: String) -> Term { unimplemented!() }

    pub fn var(name: &str) -> Term {
        Term { name: name.to_string(), arguments: vec![] }
    }

    pub fn fun(name: &str, args: Vec<Term>) -> Term {
        Term { name: name.to_string(), arguments: args.into_iter().map(|arg| Box::new(arg)).collect() }
    }

    pub fn subterms(&self) -> SubTerms {
        SubTerms { stack: vec![self] }
    }
}

pub struct SubTerms<'a> { stack: Vec<&'a Term> }

impl<'a> Iterator for SubTerms<'a> {
    type Item = &'a Term;

    fn next(&mut self) -> Option<&'a Term> {
        if self.stack.len() == 0 {
            None
        } else {
            let cur: Option<&Term> = self.stack.pop();
            for term in cur.iter() {
                for t in term.arguments.iter() { self.stack.push(&**t) }
            }
            cur
        }
    }
}

#[derive(Debug)]
pub enum Kind { Equal, NotEqual }

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Equal => write!(f, "="),
            Kind::NotEqual => write!(f, "!=")
        }
    }
}

/// Equality relation
#[derive(Debug)]
pub struct Relation {
    pub left: Term,
    pub right: Term,
    pub kind: Kind,
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.kind, self.right)
    }
}

impl Relation {
    pub fn subterms(&self) -> Chain<SubTerms<'_>, SubTerms<'_>> {
        self.left.subterms().chain(self.right.subterms())
    }

    #[allow(dead_code)]
    pub fn parse(raw: String) -> Relation {
        let mut kind: Kind = Kind::Equal;
        let mut eq_index = 0;

        for char in raw.chars().enumerate() {
            match char {
                (i, '=') => {
                    kind = Kind::Equal;
                    eq_index = i
                }
                (i, '!') if raw.chars().nth(i + 1).expect("Invalid symbol") == '=' => {
                    kind = Kind::NotEqual;
                    eq_index = i;
                }
                _ => {}
            }
        }

        let left_raw = &raw[eq_index..];
        let right_raw = &raw[..eq_index];

        Relation {
            left: Term::from(left_raw.to_string()),
            right: Term::from(right_raw.to_string()),
            kind,
        }
    }
}
