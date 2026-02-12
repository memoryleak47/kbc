use crate::*;

pub fn simplify_eq(mut e: Equation, st: &State) -> Equation {
    simplify_term(&mut e.lhs, st);
    simplify_term(&mut e.rhs, st);
    e
}

// all simplification rules should not need more memory, and thus fit into the container.
pub fn simplify_term(t: &mut FlatTerm, st: &State) {
    'outer: loop {
        for pos in 0..t.len() {
            if simplify_term2(t, pos, st) {
               continue 'outer;
            }
        }
        break
    }
}

// returns whether progress was made.
fn simplify_term2(t: &mut FlatTerm, pos: Pos, st: &State) -> bool {
    let sub = pos_idx(t, pos);
    for (subst, m) in st.index.find_matches(sub) {
        todo!()
    }
    false
}
