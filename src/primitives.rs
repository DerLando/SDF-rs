use crate::{expression::Expression, ops::{Operator}, tree::{Node, NodeType, Tree}, variable::Variable, vec2::Vec2};



pub fn circle(center: &Vec2, radius: u32) -> Tree {
    let center_sub = Expression::new(Variable::Variable, Variable::Vector(*center), crate::ops::Operator::Subtract);
    let dist_from_center = Node {
        lhs: Box::new(NodeType::Leaf(center_sub)),
        rhs: Box::new(NodeType::Leaf(Expression::default())),
        op: Operator::Length
    };
    let radius_sub = Node {
        lhs: Box::new(NodeType::Branch(dist_from_center)),
        rhs: Box::new(NodeType::Leaf(Expression::new(0.into(), (radius as i32).into(), Operator::Add))),
        op: Operator::Subtract
    };

    Tree {
        root: radius_sub
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_circle() {
        let center = (0, -1).into();
        let circle = circle(&center, 10);

        assert_eq!(Variable::Constant(-10), Tree::sign_at(&circle, (0, -1)));
        assert_eq!(Variable::Constant(0), Tree::sign_at(&circle, (10, -1)));
        assert_eq!(Variable::Constant(10), Tree::sign_at(&circle, (20, -1)));
    }
}