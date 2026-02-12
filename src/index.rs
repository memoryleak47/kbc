use crate::*;

pub struct TermIndex<T> {
    // NOTE: Twee uses an array here, indexed by Sym.
    // Twee also has different indices for Fun and Var.
    children: HashMap<Sym, TermIndex<T>>,

    here: Vec<T>,
}

impl<T> TermIndex<T> {
    pub fn new() -> Self {
        TermIndex {
            children: HashMap::default(),
            here: Vec::new(),
        }
    }

    pub fn add(&mut self, k: &FlatTerm, t: T) {
        assert!(is_canon_term(k));

        let mut index = self;
        for a in k {
            index = index.children.entry(a.sym).or_insert_with(TermIndex::new);
        }
        index.here.push(t);
    }

    pub fn find_matches(&self, k: &FlatTerm) -> Vec<(Subst, &T)> {
        if k.is_empty() {
            return self.here.iter().map(|x| (Subst::new(), x)).collect()
        }

        let mut out: Vec<(Subst, &T)> = Vec::new();

        let a = k[0].sym;
        if !a.is_var() && let Some(m) = self.children.get(&a) {
            out.extend(m.find_matches(&k[1..]));
        }

        for (s, idx) in &self.children {
            if !s.is_var() { continue }

            let size = k[0].size as usize;
            let a = &k[0..size];
            let b = &k[size..];
            for (mut subst, t) in idx.find_matches(b) {
                if let Some(aa) = subst.get(&s) {
                    if **aa != *a { continue }
                } else {
                    subst.insert(*s, ft_box(a));
                }
                out.push((subst, t));
            }
        }

        out
    }

    pub fn lookup(&self, k: &FlatTerm) -> Option<Vec<T>> {
        assert!(is_canon_term(k));

        todo!()
    }
}

#[test]
fn test_index() {
    let mut index = TermIndex::new();
    let t_in = [
        Entry { sym: Sym::mk_fn(0), size: 3 },
        Entry { sym: Sym::mk_var(0), size: 1 },
        Entry { sym: Sym::mk_var(0), size: 1 },
    ];
    index.add(&t_in, 0);
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
    let subst: Subst = [(Sym::mk_var(0), repl)].into_iter().collect();
    let matches: Vec<(Subst, &usize)> = index.find_matches(&t_out);
    let other: Vec<(Subst, &usize)> = vec![(subst, &0)];
    assert_eq!(matches, other);
}
