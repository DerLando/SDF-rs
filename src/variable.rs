use std::{ops::{Add, Neg, Sub}};

use crate::vec2::Vec2;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variable {
    VecConst(Vec2),
    NumConst(i32),
    Variable(Vec2)
}

impl Variable {
    pub fn is_variable(&self) -> bool {
        match self {
            Variable::Variable(_) => true,
            _ => false
        }
    }

    pub fn default_variable() -> Variable {
        Variable::Variable((0, 0).into())
    }
}

impl Add<Variable> for Variable {
    type Output = Variable;

    fn add(self, rhs: Variable) -> Self::Output {
        match self {
            Variable::NumConst(l) => match rhs {
                Variable::NumConst(r) => Variable::NumConst(l + r),
                Variable::VecConst(r) | Variable::Variable(r) => Variable::VecConst(r + l),
            },
            Variable::VecConst(l) | Variable::Variable(l) => match rhs {
                Variable::NumConst(r) => Variable::VecConst(l + r),
                Variable::VecConst(r) | Variable::Variable(r) => Variable::VecConst(l + r),
            },
        }
    }
}

impl Neg for Variable {
    type Output = Variable;

    fn neg(self) -> Self::Output {
        match self {
            Variable::NumConst(n) => Variable::NumConst(-n),
            Variable::VecConst(v) | Variable::Variable(v) => Variable::VecConst(-v),
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
        Variable::NumConst(arg)
    }
}

impl From<Vec2> for Variable {
    fn from(arg: Vec2) -> Self {
        Variable::VecConst(arg)
    }
}