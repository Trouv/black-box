use crate::box_internal::components::{BoxOut, BoxState};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Action {
    Set(Val, usize),
    Add(Val, Val, usize),
    AddEq(Val, usize),
    Mult(Val, Val, usize),
    Mod(Val, Val, usize),
    Equals(Val, Val, usize),
    IfElse(Val, Val, Val, usize),
    Do(Vec<Action>),
    IfElseDo(Val, Vec<Action>, Vec<Action>),
    PrintInt(Val),
}

impl Action {
    pub fn evaluate(&self, state: &mut BoxState) -> Option<BoxOut> {
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
            Action::Mod(a, b, i) => {
                state[*i] = ((a.evaluate(state) % b.evaluate(state)) + b.evaluate(state))
                    % b.evaluate(state);
                None
            }
            Action::Equals(a, b, i) => {
                state[*i] = if a.evaluate(state) == b.evaluate(state) {
                    1.
                } else {
                    0.
                };
                None
            }
            Action::IfElse(a, b, c, i) => {
                state[*i] = if a.evaluate(state) != 0. {
                    b.evaluate(state)
                } else {
                    c.evaluate(state)
                };
                None
            }
            Action::Do(dos) => {
                let mut res = None;
                for action in dos {
                    if let Some(r) = action.evaluate(state) {
                        res = Some(r);
                    }
                }
                res
            }
            Action::IfElseDo(a, if_dos, else_dos) => {
                if a.evaluate(state) != 0. {
                    Action::Do(if_dos.clone()).evaluate(state)
                } else {
                    Action::Do(else_dos.clone()).evaluate(state)
                }
            }
            Action::PrintInt(val) => Some(BoxOut::Int(val.evaluate(state) as i32)),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
