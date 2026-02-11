use crate::*;

pub type Subst = HashMap<Sym, Box<FlatTerm>>;

pub fn apply_subst(t: &FlatTerm, subst: &Subst) -> Box<FlatTerm> {
    let size = post_subst_size(t, subst);
    let default_e = Entry { sym: Sym { repr: 0 }, size: 1 };
    let mut out: Box<FlatTerm> = std::iter::repeat(default_e).take(size).collect();

    let mut i = size - 1;
    'outer: for (j, e) in t.iter().enumerate().rev() {
        if e.sym.is_var() && let Some(tt) = subst.get(&e.sym) {
            for a in tt.iter().rev() {
                out[i] = *a;
                if i == 0 { assert!(j == 0); break 'outer; }
                i -= 1;
            }
        } else {
            let child_count = ft_children(&t[j..]).count();
            let mut esize = 1;
            for _ in 0..child_count {
                esize += out[i+(esize as usize)].size;
            }
            out[i] = Entry {
                sym: e.sym,
                size: esize,
            };
            if i == 0 { assert!(j == 0); break } // TODO is this right?
            i -= 1;
        }
    }
    out
}

fn post_subst_size(t: &FlatTerm, subst: &Subst) -> usize {
    let mut s = 0;
    for Entry { sym, .. } in t {
        if sym.is_var() && let Some(tt) = subst.get(sym) {
            s += tt[0].size as usize;
        } else {
            s += 1;
        }
    }
    s
}

#[test]
fn apply_subst_t1() {
    let t_in = [
        Entry { sym: Sym::mk_fn(0), size: 3 },
        Entry { sym: Sym::mk_var(3), size: 1 },
        Entry { sym: Sym::mk_var(3), size: 1 },
    ];
    let t_out = [
        Entry { sym: Sym::mk_fn(0), size: 5 },
        Entry { sym: Sym::mk_fn(1), size: 2 },
        Entry { sym: Sym::mk_fn(2), size: 1 },
        Entry { sym: Sym::mk_fn(1), size: 2 },
        Entry { sym: Sym::mk_fn(2), size: 1 },
    ];
    let repl: Box<FlatTerm> = Box::new([
        Entry { sym: Sym::mk_fn(1), size: 2 },
        Entry { sym: Sym::mk_fn(2), size: 1 },
    ]);
    let subst = [(Sym::mk_var(3), repl)].into_iter().collect();
    let t_out2 = apply_subst(&t_in, &subst);
    assert_eq!(&t_out, &*t_out2);
}
