use std::ops::DerefMut;

use crate::{Vec2, expression::{BinaryExpression, Evaluable, Expression, UnaryExpression}, ops::{BinaryOperator, UnaryOperator}, variable::{Variable, VariableContainer}};



#[derive(Debug, Clone)]
pub(crate) struct UnaryNode {
    pub node: Box<NodeType>,
    pub op: UnaryOperator
}

impl UnaryNode {
    pub fn from_expression(expr: Expression, op: UnaryOperator) -> Self {
        Self {
            node: Box::new(NodeType::Leaf(expr)),
            op
        }
    }

    pub fn from_node(node: Node, op: UnaryOperator) -> Self {
        Self {
            node: Box::new(NodeType::Branch(node)),
            op
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BinaryNode {
    pub lhs: Box<NodeType>,
    pub rhs: Box<NodeType>,
    pub op: BinaryOperator
}

impl BinaryNode {
    pub fn from_nodes(a: Node, b: Node, op: BinaryOperator) -> Self {
        Self {
            lhs: Box::new(NodeType::Branch(a)),
            rhs: Box::new(NodeType::Branch(b)),
            op
        }
    }

    pub fn new() -> Self {
        Self {
            lhs: Box::new(NodeType::Leaf(Expression::Unary(UnaryExpression::default()))),
            rhs: Box::new(NodeType::Leaf(Expression::Unary(UnaryExpression::default()))),
            op: BinaryOperator::Add
        }
    }
}

pub(crate) struct BinaryNodeBuilder {
    lhs: NodeType,
    rhs: NodeType,
}

impl BinaryNodeBuilder {
    pub fn new() -> Self {
        Self {
            lhs: NodeType::Leaf(Expression::Unary(UnaryExpression::default())),
            rhs: NodeType::Leaf(Expression::Unary(UnaryExpression::default())),
        }
    }

    pub fn with_lhs_node(mut self, lhs: Node) -> Self {
        self.lhs = NodeType::Branch(lhs);
        self
    }

    pub fn with_lhs_leaf(mut self, lhs: Expression) -> Self {
        self.lhs = NodeType::Leaf(lhs);
        self
    }

    pub fn with_rhs_node(mut self, rhs: Node) -> Self {
        self.rhs = NodeType::Branch(rhs);
        self
    }

    pub fn with_rhs_leaf(mut self, rhs: Expression) -> Self {
        self.rhs = NodeType::Leaf(rhs);
        self
    }

    pub fn build(self, op: BinaryOperator) -> BinaryNode {
        BinaryNode {
            lhs: Box::new(self.lhs),
            rhs: Box::new(self.rhs),
            op
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Unary(UnaryNode),
    Binary(BinaryNode)
}

#[derive(Debug, Clone)]
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
