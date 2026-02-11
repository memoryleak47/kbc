use crate::*;

pub fn kbc(eqs: Vec<Equation>) {
    let mut st = State::new();
    for e in eqs {
        st.enqueue(e);
    }

    while let Some((_, x)) = st.passive.pop() {
        st.add_active(x);
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            passive: MinPrioQueue::new(),
            active: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, eq: Equation) {
        let score = eq.lhs[0].size + eq.rhs[0].size;
        self.passive.push(score, eq);
    }

    pub fn add_active(&mut self, e: Equation) {
        let mut cps = Vec::new();
        for a in &self.active {
            cps.extend(deduce_perms(a, &e));
        }
        for cp in cps {
            self.enqueue(cp);
        }

        self.active.push(e);
    }
}
