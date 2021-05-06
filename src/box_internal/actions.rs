//! Provides objects for defining the behavior of a Box's Buttons.
use crate::box_internal::components::{BoxOut, BoxState};
use serde::{Deserialize, Serialize};

/// Enum that provides various commands that read/write to a BoxState when evaluated.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Action {
    /// Evaluate the Val and store the result at the usize index.
    Set(Val, usize),
    /// Add the two Vals and store the result at the usize index.
    Add(Val, Val, usize),
    /// Add the Val and the current value at the usize index, then store the result at that index.
    AddEq(Val, usize),
    /// Multiply the two Vals and store the result at the usize index.
    Mult(Val, Val, usize),
    /// Take the first Val mod the second Val, and store the result in the usize index.
    Mod(Val, Val, usize),
    /// Set the value at the usize index to 1.0 if the two Vals are equal, 0.0 otherwise.
    Equals(Val, Val, usize),
    /// If the first Val is nonzero, set the value at the usize index to the second Val, otherwise
    /// set it to the third Val.
    IfElse(Val, Val, Val, usize),
    /// Evaluate every Action in the Vec in order, and return the last returned BoxOut (if there
    /// are any).
    Do(Vec<Action>),
    /// If the first Val is nonzero, evaluate the first Vec<Action>, otherwise evaluate the second.
    IfElseDo(Val, Vec<Action>, Vec<Action>),
    /// Return the Val as a BoxOut::Int
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
                state[*i] = if (a.evaluate(state) - b.evaluate(state)).abs() < 0.00001 {
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

/// Enum that provides abstraction over whether a value should be provided as is (C(onstant)) or
/// should be looked up in a BoxState (G(et)) when evaluated.
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Val {
    /// Constant, evaluates to the internal value.
    C(f32),
    /// Get, evaluates to a value stored in a BoxState given the internal value as an index.
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
