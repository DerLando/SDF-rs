use crate::{expression::{BinaryExpression, Evaluable, Expression, UnaryExpression}, node::{BinaryNode, BinaryNodeBuilder, Node}, ops::{BinaryOperator, UnaryOperator}, variable::{Variable, VariableContainer}, vec2::Vec2};

pub struct Tree {
    pub(crate) root: Node
}

impl Tree {
    fn combine(a: Tree, b: Tree, op: BinaryOperator) -> Self {
        let root = BinaryNode::from_nodes(a.root, b.root, op);

        Self {
            root: Node::Binary(root)
        }
    }

    pub fn sign_at(&mut self, position: impl Into<Vec2>) -> f32 {
    
        // replace all variables inside the tree with the given position
        self.root.replace_variable(position.into());

        // evaluate root and return
        match self.root.evaluate() {
            Variable::NumConst(n) => n,
            Variable::VecConst(v) | Variable::Variable(v) => panic!("Sign was vec2 instead {:?}", v)
        }
    }

    pub fn union(a: Tree, b: Tree) -> Tree {
        Self::combine(a, b, BinaryOperator::Min)
    }

    pub fn intersection(a: Tree, b: Tree) -> Tree {
        Self::combine(a, b, BinaryOperator::Max)
    }

    pub fn blend(a: Tree, b: Tree, factor: f32) -> Tree {
        let blend_dist = BinaryExpression {
            lhs: Variable::NumConst(1.0),
            rhs: Variable::NumConst(factor),
            op: BinaryOperator::Sub
        };

        let a_blend = 
            BinaryNodeBuilder::new()
                .with_lhs_leaf(Expression::Binary(blend_dist))
                .with_rhs_node(a.root)
                .build(BinaryOperator::Mul);

        let factor_noop = UnaryExpression {
            var: Variable::NumConst(factor),
            op: UnaryOperator::NoOp
        };

        let b_blend =
            BinaryNodeBuilder::new()
                .with_lhs_leaf(Expression::Unary(factor_noop))
                .with_rhs_node(b.root)
                .build(BinaryOperator::Mul);

        let combined_blend = BinaryNode::from_nodes(
            Node::Binary(a_blend),
            Node::Binary(b_blend),
            BinaryOperator::Add
        ); 

        Tree {
            root: Node::Binary(combined_blend)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::node::{NodeType, UnaryNode};

    use super::*;

    #[test]
    fn simple_tree() {
        let mut tree = Tree {
            root: Node::Unary(UnaryNode{
                node: Box::new(NodeType::Leaf(Expression::Binary(BinaryExpression{lhs: Variable::default_variable(), rhs: Variable::NumConst(1.0), op:BinaryOperator::Add}))),
                op: UnaryOperator::Length
            })
        };

        assert_eq!(2.0, tree.sign_at((1, -1)));
        assert_eq!(3.0, tree.sign_at((2, -1)));
    }
}