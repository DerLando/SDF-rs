use crate::{expression::{BinaryExpression, Expression, UnaryExpression}, ops::{BinaryOperator, UnaryOperator}, tree::{BinaryNode, Node, NodeType, Tree, UnaryNode}, variable::Variable, vec2::Vec2};



pub fn circle(center: &Vec2, radius: f32) -> Tree {
    let center_sub = 
        BinaryExpression {
            lhs: Variable::default_variable(),
            rhs: Variable::VecConst(*center),
            op: BinaryOperator::Sub
        };

    let dist_from_center = UnaryNode {
        node: Box::new(NodeType::Leaf(Expression::Binary(center_sub))),
        op: UnaryOperator::Length
    };

    let radius_neg = 
        UnaryExpression {
            var: Variable::NumConst(-radius),
            op: UnaryOperator::NoOp
        };

    let radius_sub = BinaryNode {
        lhs: Box::new(NodeType::Branch(Node::Unary(dist_from_center))),
        rhs: Box::new(NodeType::Leaf(Expression::Unary(radius_neg))),
        op: BinaryOperator::Add
    };

    Tree {
        root: Node::Binary(radius_sub)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_circle() {
        let center = (0, -1).into();
        let mut circle = circle(&center, 10.0);

        assert_eq!(-10.0, circle.sign_at((0, -1)));
        assert_eq!(0.0, circle.sign_at((10, -1)));
        assert_eq!(10.0, circle.sign_at((20, -1)));
    }
}