use crate::*;

pub fn deduce(x: &Equation, y: &Equation, cps: &mut Vec<Equation>) {
    deduce2(x, y, cps);
    deduce2(y, x, cps);
}

fn deduce2(x: &Equation, y: &Equation, cps: &mut Vec<Equation>) {
    let flip = |x: &Equation| Equation {
        lhs: x.rhs.clone(),
        rhs: x.lhs.clone(),
        oriented: false
    };

    // tries both x and its reverse for unoriented rules
    let both = |x: &Equation| {
        if x.oriented {
            vec![x.clone()]
        } else {
            vec![x.clone(), flip(x)]
        }
    };

    for x in both(x) {
        for y in both(y) {
            deduce3(&x, &y, cps);
        }
    }
}

fn deduce3(x: &Equation, y: &Equation, cps: &mut Vec<Equation>)  {
    for p in 0..y.lhs.len() {
        cps.extend(deduce4(x, y, p));
    }
}

// assumes variables are disjoint.
fn deduce4(x: &Equation, y: &Equation, p: Pos) -> Option<Equation> {
    assert!(p < y.lhs.len());

    let sig = unify(&x.lhs, pos_idx(&y.lhs, p))?;
    let sub = pos_set(&y.lhs, p, &x.rhs);
    let ll = apply_subst(&sub, &sig);
    let rr = apply_subst(&y.rhs, &sig);
    let eq = Equation { lhs: ll, rhs: rr, oriented: false };
    Some(eq)
}
