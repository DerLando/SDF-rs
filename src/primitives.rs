use crate::{Tree, expression::{BinaryExpression, Expression, UnaryExpression}, node::{BinaryNode, BinaryNodeBuilder, Node, NodeType, UnaryNode}, ops::{BinaryOperator, UnaryOperator}, variable::Variable, vec2::Vec2};



pub fn circle(center: &Vec2, radius: f32) -> Tree {
    let center_sub = 
        BinaryExpression {
            lhs: Variable::default_variable(),
            rhs: Variable::VecConst(*center),
            op: BinaryOperator::Sub
        };

    let dist_from_center = 
        UnaryNode::from_expression(
        Expression::Binary(center_sub),
        UnaryOperator::Length
        );

    let radius_neg = 
        UnaryExpression {
            var: Variable::NumConst(-radius),
            op: UnaryOperator::NoOp
        };

    let radius_sub =
        BinaryNodeBuilder::new()
            .with_lhs_node(Node::Unary(dist_from_center))
            .with_rhs_leaf(Expression::Unary(radius_neg))
            .build(BinaryOperator::Add)
            ;

    Tree {
        root: Node::Binary(radius_sub)
    }
}

pub fn rectangle(width: u32, height: u32) -> Tree {
    // R is Vec2(width, height)
    // P is point to sample at
    // d = length(max(abs(P) - R, 0)) -> single quadrant not interior
    //
    // q = abs(P) - R
    // d = length(max(q,0)) + min(max(q.x, q.y), 0)

    let abs_var = UnaryExpression {
        var: Variable::default_variable(),
        op: UnaryOperator::Abs
    };

    let w_h: UnaryExpression = Vec2::new(width as i32, height as i32).into();

    // distance vector, will be re-used
    let d = BinaryNodeBuilder::new()
        .with_lhs_leaf(Expression::Unary(abs_var))
        .with_rhs_leaf(Expression::Unary(w_h))
        .build(BinaryOperator::Sub);

    let max = BinaryNodeBuilder::new()
        .with_lhs_node(Node::Binary(d.clone()))
        .with_rhs_leaf(Expression::Unary(0.0.into()))
        .build(BinaryOperator::Max);

    let length = UnaryNode::from_node(Node::Binary(max), UnaryOperator::Length);

    let d_x = UnaryNode::from_node(Node::Binary(d.clone()), UnaryOperator::X);
    let d_y = UnaryNode::from_node(Node::Binary(d.clone()), UnaryOperator::Y);

    let max_x_y = BinaryNodeBuilder::new()
        .with_lhs_node(Node::Unary(d_x))
        .with_rhs_node(Node::Unary(d_y))
        .build(BinaryOperator::Max);

    let min_0 = BinaryNodeBuilder::new()
        .with_lhs_node(Node::Binary(max_x_y))
        .with_rhs_leaf(Expression::Unary(0.0.into()))
        .build(BinaryOperator::Min);

    let distance = BinaryNodeBuilder::new()
        .with_lhs_node(Node::Unary(length))
        .with_rhs_node(Node::Binary(min_0))
        .build(BinaryOperator::Add);

    Tree {
        root: (Node::Binary(distance))
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