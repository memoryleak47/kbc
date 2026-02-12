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

    pub fn index_match(&self, k: &FlatTerm) -> Vec<(Subst, &T)> {
        assert!(is_canon_term(k));

        if k.is_empty() {
            return self.here.iter().map(|x| (Subst::new(), x)).collect()
        }

        // should this also happen if a.is_var()?
        let a = k[0].sym;
        if !a.is_var() && let Some(m) = self.children.get(&a) {
            // TODO fix substs.
            return m.index_match(&k[1..]);
        }

        for (s, idx) in &self.children {
            if !s.is_var() { continue }

            // TODO ...
        }

        todo!()
    }

    pub fn index_lookup(&self, k: &FlatTerm) -> Option<Vec<T>> {
        assert!(is_canon_term(k));

        todo!()
    }
}
