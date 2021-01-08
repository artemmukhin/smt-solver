#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Term {
    pub name: String,
    pub arguments: Vec<Term>,
}

impl Term {
    // TODO: Should parse "f(g(a, b))" to `Symbol { "f", [ Symbol { "g", ["a", "b"] } ] }`
    #[allow(dead_code)]
    fn from(_raw: String) -> Term { unimplemented!() }
    
    pub fn var(name: &str) -> Term {
        Term { name: name.to_string(), arguments: vec![] }
    }
}

#[derive(Debug)]
pub enum RelationKind { Equal, NotEqual }

/// Equality relation
#[derive(Debug)]
pub struct Relation {
    pub left: Term,
    pub right: Term,
    pub kind: RelationKind,
}

impl Relation {
    #[allow(dead_code)]
    pub fn parse(raw: String) -> Relation {
        let mut kind: RelationKind = RelationKind::Equal;
        let mut eq_index = 0;

        for char in raw.chars().enumerate() {
            match char {
                (i, '=') => {
                    kind = RelationKind::Equal;
                    eq_index = i
                }
                (i, '!') if raw.chars().nth(i + 1).expect("Invalid symbol") == '=' => {
                    kind = RelationKind::NotEqual;
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
