use crate::{ops::{BinaryOperator, UnaryOperator}, variable::Variable, vec2::Vec2};

#[derive(Debug)]
pub struct UnaryExpression {
    pub var: Variable,
    pub op: UnaryOperator
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Variable,
    pub rhs: Variable,
    pub op: BinaryOperator
}

#[derive(Debug)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression)
}

pub trait Evaluable {
    fn evaluate(&self) -> Variable;
}

pub trait VariableContainer {
    fn replace_variable(&mut self, var: Vec2);
}

impl Evaluable for UnaryExpression {
    fn evaluate(&self) -> Variable {
        match self.op {
            UnaryOperator::Length => match self.var {
                Variable::NumConst(n) => self.var,
                Variable::VecConst(v) | Variable::Variable(v) => Variable::NumConst(v.length()),
            },
            UnaryOperator::NoOp => self.var
        }
    }
}

impl VariableContainer for UnaryExpression {
    fn replace_variable(&mut self, var: Vec2) {
        if !self.var.is_variable() {return;}

        self.var = Variable::Variable(var)
    }
}

impl Default for UnaryExpression {
    fn default() -> Self {
        Self {
            var: Variable::NumConst(0),
            op: UnaryOperator::NoOp
        }
    }
}

impl Evaluable for BinaryExpression {
    fn evaluate(&self) -> Variable {
        match self.op {
            BinaryOperator::Add => self.lhs + self.rhs,
            BinaryOperator::Subtract => self.lhs - self.rhs
        }
    }
}

impl VariableContainer for BinaryExpression {
    fn replace_variable(&mut self, var: Vec2) {
        if self.lhs.is_variable() {
            self.lhs = Variable::Variable(var);
        }

        if self.rhs.is_variable() {
            self.rhs = Variable::Variable(var);
        }
    }
}

impl Default for BinaryExpression {
    fn default() -> Self {
        Self {
            lhs: Variable::NumConst(0),
            rhs: Variable::NumConst(0),
            op: BinaryOperator::Add
        }
    }
}

impl Evaluable for Expression {
    fn evaluate(&self) -> Variable {
        match self {
            Expression::Unary(e) => e.evaluate(),
            Expression::Binary(e) => e.evaluate()
        }
    }
}

impl VariableContainer for Expression {
    fn replace_variable(&mut self, var: Vec2) {
        match self {
            Expression::Unary(e) => e.replace_variable(var),
            Expression::Binary(e) => e.replace_variable(var)
        }
    }
}