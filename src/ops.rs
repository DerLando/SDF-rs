

use crate::{tree::Tree};

#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Max,
    Min,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UnaryOperator {
    Length,
    NoOp,
    Abs,
}
