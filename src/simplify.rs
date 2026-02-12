use crate::*;

pub fn simplify_eq(mut e: Equation, st: &State) -> Equation {
    simplify_term(&mut e.lhs, st);
    simplify_term(&mut e.rhs, st);
    e
}

// NOTE: all simplification rules should not need more memory, and thus fit into the container.
// We might want a variant of "pos_set" that is more efficient for that.
pub fn simplify_term(t: &mut Box<FlatTerm>, st: &State) {
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
fn simplify_term2(t: &mut Box<FlatTerm>, pos: Pos, st: &State) -> bool {
    assert!(pos < t.len());

    let sub = pos_idx(t, pos);
    for (subst, eq_id) in st.index.find_matches(sub) {
        let rhs = &st.active[*eq_id].rhs;
        let rhs = apply_subst(rhs, &subst);
        *t = pos_set(t, pos, &rhs);

        // We just apply a single rule for now.
        return true
    }
    false
}
