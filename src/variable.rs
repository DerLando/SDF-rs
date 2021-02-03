use std::{ops::{Add, Mul, Neg, Sub}};

use crate::vec2::Vec2;


#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Variable {
    VecConst(Vec2),
    NumConst(f32),
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

    pub fn max(&self, other: &Variable) -> Variable {
        match self {
            Variable::NumConst(n) => match other {
                Variable::NumConst(o) => Variable::NumConst(n.max(*o)),
                Variable::VecConst(_) | Variable::Variable(_) => *self
            },
            Variable::VecConst(v) | Variable::Variable(v) => match other {
                Variable::NumConst(n) => Variable::VecConst(Vec2{x: v.x.max(*n as i32), y: v.y.max(*n as i32)}),
                Variable::VecConst(vo) | Variable::Variable(vo) => Variable::VecConst(v.max(vo))
            }
        }
    }

    pub fn min(&self, other: &Variable) -> Variable {
        match self {
            Variable::NumConst(n) => match other {
                Variable::NumConst(o) => Variable::NumConst(n.min(*o)),
                Variable::VecConst(_) | Variable::Variable(_) => *self
            },
            Variable::VecConst(v) | Variable::Variable(v) => match other {
                Variable::NumConst(n) => Variable::VecConst(Vec2{x: v.x.min(*n as i32), y: v.y.min(*n as i32)}),
                Variable::VecConst(vo) | Variable::Variable(vo) => Variable::VecConst(v.min(vo))
            }
        }
    }
}

pub(crate) trait VariableContainer {
    fn replace_variable(&mut self, var: Vec2);
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

impl Mul<Variable> for Variable {
    type Output = Variable;

    fn mul(self, rhs: Variable) -> Self::Output {
        match self {
            Variable::NumConst(n_l) => match rhs {
                Variable::NumConst(n_r) => Variable::NumConst(n_l * n_r),
                Variable::VecConst(v) | Variable::Variable(v) => Variable::VecConst(v * n_l)
            },
            Variable::VecConst(v_l) | Variable::Variable(v_l) => match rhs {
                Variable::NumConst(n) => Variable::VecConst(v_l * n),
                Variable::VecConst(v_r) | Variable::Variable(v_r) => Variable::NumConst((v_l * v_r) as f32)
            }
        }
    }
}

impl From<f32> for Variable {
    fn from(arg: f32) -> Self {
        Variable::NumConst(arg)
    }
}

impl From<Vec2> for Variable {
    fn from(arg: Vec2) -> Self {
        Variable::VecConst(arg)
    }
}