

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
    X,
    Y
}
