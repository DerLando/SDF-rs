use std::ops::DerefMut;

use crate::{expression::Expression, ops::Operator, variable::Variable, vec2::Vec2};

/// TODO: How to define constant variables?
#[derive(Debug, Clone)]
pub struct Tree {
    pub root: Node
}

impl Tree {
    /// TODO: How to actually feed down the vec?
    pub fn sign_at(tree: &Tree, position: impl Into<Vec2>) -> Variable {
        
        // explicitly clone the tree, as we will replace it's variable occurences
        let mut clone = tree.clone();

        // propage the given variable
        clone.propagate_variable(position.into());

        // evaluate the clones' root node
        clone.root.evaluate()
    }

    fn propagate_variable(&mut self, variable: Vec2) {
        self.root.propagate_variable(variable.into())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub lhs: Box<NodeType>,
    pub rhs: Box<NodeType>,
    pub op: Operator
}

impl Node {
    pub fn evaluate(&self) -> Variable {
        let mut expr = Expression::default();
        expr.op = self.op;
        
        match &*self.lhs {
            NodeType::Branch(node) => expr.lhs = node.evaluate(),
            NodeType::Leaf(leaf) => expr.lhs = leaf.evaluate()
        }
        match &*self.rhs {
            NodeType::Branch(node) => expr.rhs = node.evaluate(),
            NodeType::Leaf(leaf) => expr.rhs = leaf.evaluate()
        }

        expr.evaluate()
    }

    pub fn propagate_variable(&mut self, variable: Vec2) {
        match self.lhs.deref_mut() {
            NodeType::Leaf(expr) => expr.insert_variable(variable),
            NodeType::Branch(node) => node.propagate_variable(variable)
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Leaf(Expression),
    Branch(Node)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_tree() {
        let tree = Tree {
            root: Node {
                lhs: Box::new(NodeType::Leaf(Expression::new(1.into(), Variable::Variable, Operator::Add))),
                rhs: Box::new(NodeType::Leaf(Expression::default())),
                op: Operator::Add
            }
        };

        assert_eq!(Variable::Vector((2, 3).into()), Tree::sign_at(&tree, (1, 2)));
        assert_eq!(Variable::Vector((-3, 1).into()), Tree::sign_at(&tree, (-4, 0)));
    }
}