use crate::components::{BoxOut, BoxResult, BoxState};

pub enum Action {
    Set(f32, usize),
    Add(usize, usize, usize),
    AddEq(f32, usize),
    Mult(usize, usize, usize),
    PrintInt(usize),
}

fn compose(
    f: Box<dyn Fn(BoxState) -> BoxResult>,
    g: Box<dyn Fn(BoxState) -> BoxResult>,
) -> Box<dyn Fn(BoxState) -> BoxResult> {
    Box::new(move |s| {
        let (s, g_out) = g(s);
        let (s, f_out) = f(s);
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
            Action::AddEq(val, i) => compose(
                Action::Add(0, 7, *i).evaluate(),
                Action::Set(*val, 7).evaluate(),
            ),
            Action::Mult(a, b, i) => Box::new(move |s| {
                let mut s = s.clone();
                s[*i] = s[*a] * s[*b];
                (s, None)
            }),
            Action::PrintInt(i) => Box::new(move |s| (s, Some(BoxOut::Int(s[*i] as i32)))),
        }
    }
}
