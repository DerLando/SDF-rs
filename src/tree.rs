use std::ops::DerefMut;

use crate::{expression::{BinaryExpression, Evaluable, Expression, UnaryExpression}, ops::{BinaryOperator, UnaryOperator}, variable::{Variable, VariableContainer}, vec2::Vec2};

pub struct Tree {
    pub(crate) root: Node
}

impl Tree {
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
        Tree{
            root: Node::Binary(
                BinaryNode {
                    lhs: Box::new(NodeType::Branch(a.root)),
                    rhs: Box::new(NodeType::Branch(b.root)),
                    op: BinaryOperator::Min
                }
            )
        }
    }

    pub fn intersection(a: Tree, b: Tree) -> Tree {
        Tree {
            root: Node::Binary(
                BinaryNode {
                    lhs: Box::new(NodeType::Branch(a.root)),
                    rhs: Box::new(NodeType::Branch(b.root)),
                    op: BinaryOperator::Max
                }
            )
        }
    }

    pub fn blend(a: Tree, b: Tree, factor: f32) -> Tree {
        let blend_dist = BinaryExpression {
            lhs: Variable::NumConst(1.0),
            rhs: Variable::NumConst(factor),
            op: BinaryOperator::Sub
        };

        let a_blend = BinaryNode {
            lhs: Box::new(
                NodeType::Leaf(
                    Expression::Binary(blend_dist)
                )
            ),
            rhs: Box::new(NodeType::Branch(a.root)),
            op: BinaryOperator::Mul
        };

        let b_blend = BinaryNode {
            lhs: Box::new(
                NodeType::Leaf(
                    Expression::Unary(
                        UnaryExpression {
                            var: Variable::NumConst(factor),
                            op: UnaryOperator::NoOp
                        }))),
            rhs: Box::new(
                NodeType::Branch(
                    b.root
                )
            ),
            op: BinaryOperator::Mul
        };

        let combined_blend = BinaryNode {
            lhs: Box::new(NodeType::Branch(Node::Binary(a_blend))),
            rhs: Box::new(NodeType::Branch(Node::Binary(b_blend))),
            op: BinaryOperator::Add
        };

        Tree {
            root: Node::Binary(combined_blend)
        }
    }
}

#[derive(Debug)]
pub(crate) struct UnaryNode {
    pub node: Box<NodeType>,
    pub op: UnaryOperator
}

#[derive(Debug)]
pub(crate) struct BinaryNode {
    pub lhs: Box<NodeType>,
    pub rhs: Box<NodeType>,
    pub op: BinaryOperator
}

#[derive(Debug)]
pub(crate) enum Node {
    Unary(UnaryNode),
    Binary(BinaryNode)
}

#[derive(Debug)]
pub(crate) enum NodeType {
    Leaf(Expression),
    Branch(Node)
}

impl Evaluable for Node {
    fn evaluate(&self) -> Variable {
        match self {
            Node::Unary(un) => {
                let mut expr = UnaryExpression::default();
                expr.op = un.op;

                match &*un.node {
                    NodeType::Branch(node) => expr.var = node.evaluate(),
                    NodeType::Leaf(leaf) => expr.var = leaf.evaluate()
                }
                expr.evaluate()
            },
            Node::Binary(bn) => {
                let mut expr = BinaryExpression::default();
                expr.op = bn.op;

                match &*bn.lhs {
                    NodeType::Branch(node) => expr.lhs = node.evaluate(),
                    NodeType::Leaf(leaf) => expr.lhs = leaf.evaluate()
                }
                match &*bn.rhs {
                    NodeType::Branch(node) => expr.rhs = node.evaluate(),
                    NodeType::Leaf(leaf) => expr.rhs = leaf.evaluate()
                }
                expr.evaluate()
            }
        }
    }
}

impl VariableContainer for Node {
    fn replace_variable(&mut self, var: Vec2) {
        match self {
            Node::Unary(un) => match un.node.deref_mut() {
                NodeType::Branch(node) => node.replace_variable(var),
                NodeType::Leaf(leaf) => leaf.replace_variable(var)
            },
            Node::Binary(bn) => {
                match bn.lhs.deref_mut() {
                    NodeType::Branch(node) => node.replace_variable(var),
                    NodeType::Leaf(leaf) => leaf.replace_variable(var)
                }
                match bn.rhs.deref_mut() {
                    NodeType::Branch(node) => node.replace_variable(var),
                    NodeType::Leaf(leaf) => leaf.replace_variable(var)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
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