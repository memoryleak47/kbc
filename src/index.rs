use crate::*;

// NOTE: Twee uses an array here, indexed by Sym.
pub type TermIndex = HashMap<Sym, IndexNode>;

pub enum IndexNode {
    Branch(TermIndex),
    Leaf(Vec<EqId>),
}

pub fn index_add(index: &mut TermIndex, k: &FlatTerm, v: EqId) {
    todo!()
}

pub type Match = (Subst, EqId);

pub fn index_match(index: &TermIndex, k: &FlatTerm) -> Vec<Match> {
    todo!()
}
