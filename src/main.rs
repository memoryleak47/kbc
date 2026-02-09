mod minqueue;
pub use minqueue::*;

mod flatterm;
pub use flatterm::*;

mod order;
pub use order::*;

mod parse;
pub use parse::*;

pub use std::collections::{HashMap, BTreeMap};

type DiscrMap = HashMap<Sym, DiscrNode>;

enum DiscrNode {
    Branch(DiscrMap),
    Leaf(Vec<EqId>),
}

pub struct Equation {
    lhs: Box<FlatTerm>,
    rhs: Box<FlatTerm>,
    oriented: bool,
}



type RuleId = usize;
type Pos = usize;
type Score = usize;

struct Passive {
    lhs: RuleId,
    rhs: RuleId,
    pos: Pos,
}

pub type EqId = usize; // index into equations.

pub struct State {
    // invariant: Contains every lhs (and rhs of unoriented rules)
    discr: DiscrMap,

    // invariant: every CP from elements of `equations` shall be in passive.
    // indices into this shall be stable.
    equations: Vec<Equation>,

    passive: MinPrioQueue<Score, Passive>,
}

fn main() {
}
