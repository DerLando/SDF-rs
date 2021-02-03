

use crate::{tree::Tree};

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Length,
    NoOp,
}
