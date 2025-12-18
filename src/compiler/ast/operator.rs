#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Operator {
    Add,
    Minus,
    Negative,
    Multi,
    Divide,
}
