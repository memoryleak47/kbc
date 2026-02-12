use crate::*;

pub fn simplify_eq(mut e: Equation, st: &State) -> Equation {
    e.lhs = simplify_term(e.lhs, st);
    e.rhs = simplify_term(e.rhs, st);
    e
}

pub fn simplify_term(t: Box<FlatTerm>, st: &State) -> Box<FlatTerm> {
    // TODO
    t
}
