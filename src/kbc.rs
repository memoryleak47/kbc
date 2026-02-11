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

    pub fn add_active(&mut self, mut e: Equation) {
        if e.lhs == e.rhs { return }
        rename_canon(&mut e);
        if self.active.contains(&e) { return }

        let mut cps = Vec::new();
        {
            let mut e2 = e.clone();
            make_odd(&mut e2.lhs);
            make_odd(&mut e2.rhs);

            for mut a in self.active.iter().cloned() {
                make_even(&mut a.lhs);
                make_even(&mut a.rhs);
                cps.extend(deduce_perms(&a, &e2));
            }
            for cp in cps {
                self.enqueue(cp);
            }
        }

        let i = self.active.len();
        println!("{i}: {:?} = {:?}", e.lhs, e.rhs);
        self.active.push(e);
    }
}
