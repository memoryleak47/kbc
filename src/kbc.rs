use crate::*;

pub fn kbc(eqs: Vec<Equation>) {
    for eq in &eqs {
        println!("{:?} = {:?}", &eq.lhs, &eq.rhs);
    }
    todo!()
}
