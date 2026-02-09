use crate::*;

// s > t
pub fn gt(s: &FlatTerm, t: &FlatTerm) -> bool {
    let vars_s = get_vars(s);
    let vars_t = get_vars(t);
    for (x, ct) in &vars_t {
        let cs = vars_s.get(x).unwrap_or(&0);
        // if t contains a variable more than s, then we have to return false.
        if ct > cs { return false }
    }

    let ws = s[0].size as usize;
    let wt = t[0].size as usize;
    if ws > wt { return true }
    if ws < wt { return false }

    assert_eq!(ws, wt);

    let fs = s[0].sym;
    let ft = t[0].sym;
    if fs.is_var() | ft.is_var() { return false }

    if fs > ft { return true }
    if fs < ft { return false }

    assert_eq!(fs, ft);

    let mut is = 1;
    let mut it = 1;
    while is < ws && it < wt {
        let is_size = s[is].size as usize;
        let it_size = s[it].size as usize;
        let sub_s = &s[is..(is+is_size)];
        let sub_t = &s[it..(it+it_size)];
        if gt(sub_s, sub_t) { return true }

        if sub_s == sub_t {
            is += is_size;
            it += it_size;
        } else { return false }
    }

    assert!(s == t);

    false
}

pub fn get_vars(t: &FlatTerm) -> BTreeMap<Sym, usize> {
    let mut out = BTreeMap::new();
    acc_vars(&t, &mut out);
    out
}

pub fn get_vars_eq(eq: &Equation) -> BTreeMap<Sym, usize> {
    let mut out = BTreeMap::new();
    acc_vars(&eq.lhs, &mut out);
    acc_vars(&eq.rhs, &mut out);
    out
}

fn acc_vars(t: &FlatTerm, acc: &mut BTreeMap<Sym, usize>) {
    for Entry { sym, .. } in t {
        if sym.is_var() {
            *acc.entry(*sym).or_default() += 1;
        }
    }
}

// TODO:
#[cfg(test)]
mod tst {
    use crate::*;

    fn kbo_assert(x: &str) {
        for op in ["~", "==", "<", ">"] {
            if x.contains(op) { // unrelated
                let [l, r] = *x.split(op).collect::<Vec<_>>() else { panic!() };
                let l = FlatTerm::parse(l).unwrap();
                let r = FlatTerm::parse(r).unwrap();

                let l_gt_r = gt(&l, &r);
                let r_gt_l = gt(&r, &l);

                match op {
                    "~" => { assert!(!l_gt_r); assert!(!r_gt_l); },
                    "==" => { assert!(l == r); assert!(!l_gt_r); assert!(!r_gt_l); },
                    "<" => { assert!(r_gt_l); assert!(!l_gt_r); },
                    ">" => { assert!(!r_gt_l); assert!(l_gt_r); },
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn refl() {
        kbo_assert("c == c");
        kbo_assert("X == X");
        kbo_assert("f(X) == f(X)");
        kbo_assert("f(X, Y) == f(X, Y)");
    }

    #[test]
    fn incompat_vars() {
        kbo_assert("f(X) ~ Y");
        kbo_assert("X ~ Y");
        kbo_assert("f(c) ~ X");
        kbo_assert("c ~ X");
    }

    #[test]
    fn weight_chk() {
        kbo_assert("f(X) > X");
        kbo_assert("f(c) > c");
        kbo_assert("f(X) > c");
    }

    #[test]
    fn lex_chk() {
        kbo_assert("c < d");
        kbo_assert("c < c2");
        kbo_assert("c2 < d");
        kbo_assert("f(X, Y) ~ f(Y, X)");
        kbo_assert("g(X) > f(a)");
    }

    #[test]
    fn assoc() {
        kbo_assert("f(f(X, Y), Z) > f(X, f(Y, Z))");
    }
}
