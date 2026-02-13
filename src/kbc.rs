use crate::*;

pub fn kbc(eqs: Vec<Equation>, goals: Vec<Goal>) {
    let mut st = State::new();

    // only ground goals allowed for now.
    for g in &goals {
        assert!(is_ground(&g.lhs));
        assert!(is_ground(&g.rhs));
    }
    st.goals = goals;

    for e in eqs {
        st.enqueue(e);
    }

    while let Some((_, x)) = st.passive.pop() {
        st.add_active(x);

        if st.check_goals() {
            println!("goal reached!");
            return;
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            passive: MinPrioQueue::new(),
            active: Vec::new(),
            index: TermIndex::new(),
            goals: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, eq: Equation) {
        // We simplify in enqueue to get the score right.
        let eq = simplify_eq(eq, self);
        if eq.lhs == eq.rhs { return }
        // NOTE: We could do some "is `eq` already in the queue? / active?" checking here

        let score = eq.lhs[0].size + eq.rhs[0].size;
        self.passive.push(score, eq);
    }

    pub fn add_active(&mut self, e: Equation) {
        // We simplify in add_active (in addition to enqueue) as many new rules might have been added by now.
        let mut e = simplify_eq(e, self);

        if e.lhs == e.rhs { return }
        make_canon(&mut e);
        if self.active.contains(&e) { return }

        {
            let mut e2 = e.clone();
            make_odd(&mut e2.lhs);
            make_odd(&mut e2.rhs);

            let mut cps = Vec::new();
            for mut a in self.active.iter().cloned() {
                make_even(&mut a.lhs);
                make_even(&mut a.rhs);
                deduce(&a, &e2, &mut cps);
            }
            for cp in cps {
                self.enqueue(cp);
            }
        }

        let i = self.active.len();
        println!("{i}: {}", eq_string(&e));
        if e.oriented {
            // for now we only add oriented equations to the discr tree, for simplicities sake.
            self.index.add(&e.lhs, i);
        }
        self.active.push(e);
    }

    pub fn check_goals(&mut self) -> bool {
        let mut goals = Vec::new();
        for mut g in std::mem::take(&mut self.goals) {
            simplify_term(&mut g.lhs, self);
            simplify_term(&mut g.rhs, self);
            if g.lhs == g.rhs { return true }
            goals.push(g);
        }
        self.goals = goals;
        false
    }
}

fn make_canon(e: &mut Equation) {
    if gt(&e.lhs, &e.rhs) {
        e.oriented = true;
        rename_canon(e);
        return
    }

    if gt(&e.rhs, &e.lhs) {
        std::mem::swap(&mut e.lhs, &mut e.rhs);
        e.oriented = true;
        rename_canon(e);
        return
    }

    e.oriented = false;

    let mut e2 = e.clone();
    std::mem::swap(&mut e2.lhs, &mut e2.rhs);

    rename_canon(e);
    rename_canon(&mut e2);

    if &e2 < e { std::mem::swap(e, &mut e2); }
}

fn eq_string(e: &Equation) -> String {
    let op = match e.oriented {
        false => "=",
        true => "->"
    };
    format!("{} {op} {}", ft_string(&e.lhs), ft_string(&e.rhs))
}
