use crate::*;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Equation {
    pub lhs: Box<FlatTerm>,
    pub rhs: Box<FlatTerm>,
    pub oriented: bool,
}

pub type RuleId = usize;
pub type Score = u32;

pub type EqId = usize; // index into equations.

pub struct State {
    // invariant: Contains every lhs (and rhs of unoriented rules)
    pub index: TermIndex<EqId>,

    // invariant: every CP from elements of `equations` shall be in passive.
    // indices into this shall be stable.
    pub active: Vec<Equation>,

    pub passive: MinPrioQueue<Score, Equation>,
}

// NOTE: Twee uses this to encode passives as its more memory efficient. Maybe later.
#[allow(unused)]
struct Passive {
    pub lhs: RuleId,
    pub rhs: RuleId,
    pub pos: Pos,
}
