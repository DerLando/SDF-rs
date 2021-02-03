# SDF-RS

For now a simple learning project, to learn about signed-distance-fields.

To keep the scope of this manageable, for now I will only implement 2d sdf.

# SDF Tree

The idea is to have a tree-like structure representing formulas in memory, similar to how libfive does it.
This tree gets fed a variable **V** and traverses down it's expression nodes,
replacing all occurences of the **V** variable with it's value and evaluating the nodes.

f.e.

 - circle: |V - C| - r, where *C* is the Center vector of the circle and *r* it's radius constant

We can either copy the expression Tree for every evaluation and replace all occurances of **V** with the given values.
Or we push down the value the whole tree and evaluate all nodes. For this we need two-way tree traversal.
Since copy is easy to implement, let's go with this first.

Combining sdfs is super easy here, just make a new tree, insert both trees as lhs and rhs of it's root node und store the op that should be applied.

There is some awkwardness in representing operators that don't abide to the *lhs-rhs* structure of expressions. The *abs* or *length* operators need to store useless *junk-data* to be representable in this model. A variable amount of *Variables* for Expressions and Nodes could eleviate that problem.

# SDF Enum

We could represend the whole SDF as nested enum. The *Operator* enum could take from 0 to 1 *Variables*. But how do we nest this down then? :thinking:. AH! A *Variable* plus an *Operator* can implicitly be converted to an expression. The *Variable* is the left-hand-side and the operator is both rhs and op. We could more elegantly represent f.e. the *Length* operator, or the *negate* operator, as those take no Arguments when implicitly executed on the variable before it. The *Tree* would then be a vector of ops.

f.e.

 - circle: |V - C| - r => [Op::NoOp(Variable::Variable), Op::Sub(Variable::Vector(C)), Op::Length, Op::Sub(Variable::Constant(r))]

But how do we combine trees here? We would need to store a collection of those trees and evaluate them in order. This seems way more messy than the *Node-tree-approach*.

Also an expression chain could only be evaluated linearly, while a *tree-like* structure can be traversed in parallel.

# Hybrid Enum Tree

Taking the best of both worlds. We give store a *TreeNode* enum in a *Node* which is either a an *ExpressionChain* like described in **SDF Enum** or another *Node*. Nodes store a *TreeOp* telling them how to evaluate the internal *TreeNode*. *TreeOp* can hold an *ExpressionChain*, but can also be parameterless like *abs* or *length*.

# SDF Language

Create a programming language to store sdfs as ASTs. The library will need this at some point anyways to give a convenient API surface. At the start the language won't need any parsing support. It *needs* to support the following to create sdfs:
 - Variables -> Not *really* though, there is ever only **one** variable **V**, which gets replaced by the caller when evaluating. Everything alse can be stored as constants.
 - Functions
 - Expressions

# Notes

correct terminology for Operations with multiple args:
 - **Unary** for single Argument *f.e. abs, length, negate*
 - **Binary** for two Arguments *f.e. add, sub, mul, blend, max*

So there are also two types of expressions:
 - **UnaryExpression**
 - **BinaryExpression**