use std::{ops::{Add, Neg, Sub}, unreachable};

use crate::vec2::Vec2;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variable {
    Vector(Vec2),
    Constant(i32),
    Variable
}

impl Add<Variable> for Variable {
    type Output = Variable;

    fn add(self, rhs: Variable) -> Self::Output {
        match self {
            Variable::Constant(l) => match rhs {
                Variable::Constant(r) => Variable::Constant(l + r),
                Variable::Vector(r) => Variable::Vector(r + l),
                _ => unreachable!()
            },
            Variable::Vector(l) => match rhs {
                Variable::Constant(r) => Variable::Vector(l + r),
                Variable::Vector(r) => Variable::Vector(l + r),
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }
}

impl Neg for Variable {
    type Output = Variable;

    fn neg(self) -> Self::Output {
        match self {
            Variable::Constant(n) => Variable::Constant(-n),
            Variable::Vector(n) => Variable::Vector(-n),
            _ => unreachable!()
        }
    }
}

impl Sub<Variable> for Variable {
    type Output = Variable;

    fn sub(self, rhs: Variable) -> Self::Output {
        self + -rhs
    }
}

impl From<i32> for Variable {
    fn from(arg: i32) -> Self {
        Variable::Constant(arg)
    }
}

impl From<Vec2> for Variable {
    fn from(arg: Vec2) -> Self {
        Variable::Vector(arg)
    }
}