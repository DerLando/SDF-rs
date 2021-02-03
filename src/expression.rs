use crate::{ops::Operator, variable::Variable, vec2::Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Expression {
    pub lhs: Variable,
    pub rhs: Variable,
    pub op: Operator
}

impl Expression {
    pub fn new(lhs: Variable, rhs: Variable, op: Operator) -> Self {
        Self {
            lhs,
            rhs,
            op
        }
    }

    pub fn evaluate(&self) -> Variable {
        println!("Evaluating {:?}", self);
        match self.op {
            Operator::Add => self.lhs + self.rhs,
            Operator::Subtract => self.lhs - self.rhs,
            Operator::Length => match self.lhs {
                Variable::Vector(v) => Variable::Constant(v.length()),
                Variable::Constant(c) => Variable::Constant(c),
                _ => unreachable!()
            }
        }
    }

    pub fn can_evaluate(&self) -> bool {
        !((self.lhs == Variable::Variable) | (self.rhs == Variable::Variable))
    }

    pub fn insert_variable(&mut self, var: Vec2) {
        if self.lhs == Variable::Variable {self.lhs = Variable::Vector(var)}
        if self.rhs == Variable::Variable {self.rhs = Variable::Vector(var)}
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::new(Variable::Constant(0), Variable::Constant(0), Operator::Add)
    }
}