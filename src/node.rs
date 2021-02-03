use std::ops::DerefMut;

use crate::{Vec2, expression::{BinaryExpression, Evaluable, Expression, UnaryExpression}, ops::{BinaryOperator, UnaryOperator}, variable::{Variable, VariableContainer}};



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
