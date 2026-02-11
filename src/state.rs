use crate::*;

// NOTE: Twee uses an array here, indexed by Sym.
type DiscrMap = HashMap<Sym, DiscrNode>;

enum DiscrNode {
    Branch(DiscrMap),
    Leaf(Vec<EqId>),
}

#[derive(PartialEq, Eq)]
pub struct Equation {
    pub lhs: Box<FlatTerm>,
    pub rhs: Box<FlatTerm>,
    pub oriented: bool,
}

type RuleId = usize;
type Pos = usize;
type Score = u32;

struct Passive {
    pub lhs: RuleId,
    pub rhs: RuleId,
    pub pos: Pos,
}

pub type EqId = usize; // index into equations.

pub struct State {
    // invariant: Contains every lhs (and rhs of unoriented rules)
    // discr: DiscrMap,

    // invariant: every CP from elements of `equations` shall be in passive.
    // indices into this shall be stable.
    pub active: Vec<Equation>,

    pub passive: MinPrioQueue<Score, Equation>,
}
