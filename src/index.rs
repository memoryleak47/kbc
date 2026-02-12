use crate::*;

// NOTE: Twee uses an array here, indexed by Sym.
pub type TermIndex = HashMap<Sym, IndexNode>;

pub enum IndexNode {
    Branch(TermIndex),
    Leaf(Vec<EqId>),
}

