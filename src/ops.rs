

use crate::{tree::Tree};

#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryOperator {
    Add,
    Subtract,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UnaryOperator {
    Length,
    NoOp,
}
