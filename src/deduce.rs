use crate::*;

pub fn deduce_perms(x: &Equation, y: &Equation) -> Vec<Equation> {
    let flip = |x: &Equation| Equation {
        lhs: x.rhs.clone(),
        rhs: x.lhs.clone(),
        oriented: false
    };

    let mut cps = Vec::new();
    for x in [&x, &flip(x)] {
        for y in [&y, &flip(y)] {
            cps.extend(deduce(x, y));
            cps.extend(deduce(y, x));
        }
    }
    cps
}

pub fn deduce(x: &Equation, y: &Equation) -> Vec<Equation> {
    let mut cps = Vec::new();
    for p in 0..y.lhs.len() {
        cps.extend(deduce_one(x, y, p));
    }
    cps
}

// assumes variables are disjoint.
pub fn deduce_one(x: &Equation, y: &Equation, p: usize) -> Option<Equation> {
    assert!(p < y.lhs.len());

    let sig = unify(&x.lhs, pos_idx(&y.lhs, p))?;
    let sub = pos_set(&y.lhs, p, &x.rhs);
    let ll = apply_subst(&sub, &sig);
    let rr = apply_subst(&y.rhs, &sig);
    let eq = Equation { lhs: ll, rhs: rr, oriented: false };
    Some(eq)
}

// t[p]
fn pos_idx(t: &FlatTerm, p: usize) -> &FlatTerm {
    assert!(p < t.len());
    let size = t[p].size as usize;
    &t[p..(p+size)]
}

// t[p := t2]
fn pos_set(t: &FlatTerm, p: usize, t2: &FlatTerm) -> Box<FlatTerm> {
    assert!(p < t.len());
    ft_check(t);
    ft_check(t2);

    // delta = how much bigger is the output term.
    let delta: i32 = (t2[0].size as i32) - (t[p].size as i32);
    let size = t[0].size as i32 + delta;
    let default_e = Entry { sym: Sym { repr: 0 }, size: 1 };
    let mut out: Box<FlatTerm> = std::iter::repeat(default_e).take(size as usize).collect();

    // I. The part after p remains unchanged.
    let post_start = p + t[p].size as usize;
    for i in post_start..t.len() {
        out[(i as i32 + delta) as usize] = t[i];
    }

    // II. insert p := t2.
    for i in 0..t2.len() {
        out[p+i] = t2[i];
    }

    // III. compute part prior to p.
    if p != 0 {
        let mut i = p-1;
        for (j, e) in t[..p].iter().enumerate().rev() {
            let child_count = ft_children(&t[j..]).count();
            let mut esize = 1;
            for _ in 0..child_count {
                esize += out[i+(esize as usize)].size;
            }
            out[i] = Entry {
                sym: e.sym,
                size: esize,
            };
            if i == 0 { assert!(j == 0); break }
            i -= 1;
        }
    }
    out
}
