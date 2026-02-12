use crate::*;

pub type Pos = usize;

// t[p]
pub fn pos_idx(t: &FlatTerm, p: Pos) -> &FlatTerm {
    assert!(p < t.len());
    let size = t[p].size as usize;
    &t[p..(p+size)]
}

// t[p := t2]
pub fn pos_set(t: &FlatTerm, p: Pos, t2: &FlatTerm) -> Box<FlatTerm> {
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
    let mut i = p;
    for (j, e) in t[..p].iter().enumerate().rev() {
        i -= 1;
        let child_count = ft_children(&t[j..]).count();
        let mut esize = 1;
        for _ in 0..child_count {
            esize += out[i+(esize as usize)].size;
        }
        out[i] = Entry {
            sym: e.sym,
            size: esize,
        };
    }
    assert!(i == 0);

    out
}
