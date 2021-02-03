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

# SDF Enum

We could represend the whole SDF as nested enum. The *Operator* enum could take from 0 to 1 *Variables*. But how do we nest this down then? :thinking:. AH! A *Variable* plus an *Operator* can implicitly be converted to an expression. The *Variable* is the left-hand-side and the operator is both rhs and op. We could more elegantly represent f.e. the *Length* operator, or the *negate* operator, as those take no Arguments when implicitly executed on the variable before it. The *Tree* would then be a vector of ops

f.e.

 - circle: |V - C| - r => [Op::NoOp(Variable::Variable), Op::Sub(Variable::Vector(C)), Op::Length, Op::Sub(Variable::Constant(r))]