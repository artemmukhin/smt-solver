use smt_solver;
use smt_solver::{Relation, Term, RelationKind};

fn main() {
    // `f(x, y) = g(x)`
    let relation = Relation {
        left: Term { 
            name: "f".to_string(),
            arguments: vec![Term::var("x"), Term::var("y")]
        },
        right: Term {
            name: "g".to_string(),
            arguments: vec![Term::var("x")]
        },
        kind: RelationKind::Equal
    };
    
    println!("{:#?}", relation);
}
