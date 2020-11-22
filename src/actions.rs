use crate::components::{BoxOut, BoxResult, BoxState};

pub enum Action {
    Set(f32, usize),
    Add(usize, usize, usize),
    AddEq(f32, usize),
    Mult(usize, usize, usize),
    PrintInt(usize),
}

fn compose(f: Action, g: Action) -> Box<dyn Fn(BoxState) -> BoxResult> {
    Box::new(move |s| {
        let (s, g_out) = g.evaluate()(s);
        let (s, f_out) = f.evaluate()(s);
        match (f_out, g_out) {
            (Some(out), _) => (s, Some(out)),
            (_, opout) => (s, opout),
        }
    })
}

impl Action {
    pub fn evaluate<'a>(&'a self) -> Box<dyn Fn(BoxState) -> BoxResult + 'a> {
        match self {
            Action::Set(val, i) => Box::new(move |s| {
                let mut s = s.clone();
                s[*i] = *val;
                (s, None)
            }),
            Action::Add(a, b, i) => Box::new(move |s| {
                let mut s = s.clone();
                s[*i] = s[*a] + s[*b];
                (s, None)
            }),
            Action::AddEq(val, i) => compose(Action::Add(*i, 7, *i), Action::Set(*val, 7)),
            Action::Mult(a, b, i) => Box::new(move |s| {
                let mut s = s.clone();
                s[*i] = s[*a] * s[*b];
                (s, None)
            }),
            Action::PrintInt(i) => Box::new(move |s| (s, Some(BoxOut::Int(s[*i] as i32)))),
        }
    }
}
