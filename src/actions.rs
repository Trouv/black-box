use crate::components::{BoxOut, BoxState};

pub enum Action {
    Set(Val, usize),
    Add(Val, Val, usize),
    AddEq(Val, usize),
    Mult(Val, Val, usize),
    PrintInt(Val),
}

impl Action {
    pub fn evaluate<'a>(&self, state: &mut BoxState) -> Option<BoxOut> {
        match self {
            Action::Set(val, i) => {
                state[*i] = val.evaluate(state);
                None
            }
            Action::Add(a, b, i) => {
                state[*i] = a.evaluate(state) + b.evaluate(state);
                None
            }
            Action::AddEq(a, i) => Action::Add(*a, Val::G(*i), *i).evaluate(state),
            Action::Mult(a, b, i) => {
                state[*i] = a.evaluate(state) * b.evaluate(state);
                None
            }
            Action::PrintInt(val) => Some(BoxOut::Int(val.evaluate(state) as i32)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Val {
    C(f32),
    G(usize),
}

impl Val {
    fn evaluate(&self, state: &BoxState) -> f32 {
        match self {
            Val::C(val) => *val,
            Val::G(i) => state[*i],
        }
    }
}
