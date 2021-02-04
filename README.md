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

So there are also multiple types of expressions:
 - **UnaryExpression**
 - **BinaryExpression**

Thinking about how to implement tertiary and functions of even higher arity.
For most operators we should be able to store the numbers in the enum variant, keeping the nodes cleaner.
F.e. this would reduce the arity of *clamp* from 3 to 1.

# TODO

 - [ ] Need to support tertiary expressions, too *(f.e. clamp)*
 - [ ] Probably can remove expressions all together, as their functionality is mirrored by nodes
 - [ ] Change grom Vec2 to Vec3, the implementation is generic enough that it's not hard
 - [ ] Allow internal vec representation to go up to vec4, to keep it in line with shader impls
 - [ ] Change all internal numbers to f32
 - [ ] Re-think if supporting Vec3s as variables is going to work.

The mathematical model is still wierd at some points. Some primitives want the length operator, but some ops like sin don't make any sense when applied to vec2s. Libfive only represents numbers, so it doesn't have any vector specific functions like `length`.
For now let's just implement all vector ops that aren't too clear how they should work as component-wise.
This means sin(v) -> (sin(v.x), sin(v.y)).
We already come to a dead and when looking at operators like `Atan2` or `Cross`. One is only useable on numbers, while the other only ever makes sense on Vecs. On the other hand `Add` can even operate on a mix of numbers and vecs. In reality we have 3 different possible nodes:
 - Algebraic -> just numbers
 - Spatial -> just vecs (of any dimension)
 - Mixed -> both a number and a vec (only possible in binary nodes)

So clearly, the system of using enums to describe operators is falling apart. Could we switch to a trait-based approach?
We could then composite operators (and even nodes) from their algebraic types. The downside of this is passing around trait-objects everywhere and I don't know what the implications of that are memory-wise.
A basic implementation of this could be like so:
```rust

/// All possible vec dimensions
enum Vec {
    Vec2,
    Vec3,
    Vec4
}

/// A variable in a function
enum Variable {
    VecConst(Vec),
    NumConst(f32),
    /// This has to be replaced before evaluating
    Replaceable(Vec3)
}

/// After performing an operation, there can't be a replaceable variable left,
/// As it will have been consumed
enum EvaluatedVariable {
    VecConst(Vec),
    NumConst(f32)
}

/// The basic operator trait all operators have to implement
trait Operator {
    /// operate on the values stored in this and return a variable
    fn operate(&self) -> EvaluatedVariable;
}

/// Operators that can only ever be used on numbers
trait Algebraic: Operator;

/// Operators that can only ever be used on Vecs
trait Spatial: Operator;

/// Operators that can be used on a mix of both algebraic and spatial
trait Mixed: Operator;

struct Add {
    lhs: dyn Mixed,
    rhs: dyn Mixed
}

impl Mixed for Add {
    fn operate(&self) -> EvaluatedVariable {
        // How do we know the return type for the num here?
        // probably have to wrap in a tuple and implement from...
        self.lhs.operate() + self.rhs.operate()
    }
}

struct Atan2 {
    lhs: dyn Algebraic,
    rhs: dyn Algebraic
}

impl Algebraic for Atan2 {
    fn operate(&self) -> EvaluatedVariable {
        // can we override operate in algebraic to have the return type f32?
        Math::Atan2(self.lhs.operate().as_num(), self.rhs.operate().as_num()).into()
    }
}

struct CrossProduct {
    lhs: dyn Spatial,
    rhs: dyn Spatial
}

impl Spatial for CrossProduct {
    fn operate(&self) -> EvaluatedVariable {
        Vec::CrossProduct(&self.lhs.operate().as_vec(), &self.rhs.operate().as_vec()).into()
    }
}

```

## List of operators to implement

Operators will be ordered by their arity, we should be able to implement all higher-arity operators, as unary or binary enum variants. This might have negative side-effects for ffi later on.
For now all *classical* number ops, will be implemented *component-wise* for vecs.

### Unary Ops

First the *pure* unary ops, which require no additional to compute other than the given *value*

 - [x] NoOp -> *No operation is done, value is passed along on evaluation*
 - [x] X -> *x property of Vec3 is returned, if used on numbers it behaves as NoOp*
 - [x] Y -> *same as `X` but for y property*
 - [ ] Z -> *same as `X` but for z property*
 - [ ] Square -> *trivial `pow(2)` for numbers*
 - [ ] Sqrt -> *`.sqrt()` for numbers*
 - [ ] Neg -> *negates numbers, vecs get negated component-wise*
 - [ ] Sin -> *`.sin()` for numbers*
 - [ ] Cos ->
 - [ ] Tan ->
 - [ ] ASin ->
 - [ ] ACos ->
 - [ ] ATan ->
 - [ ] Abs ->
 - [ ] Log -> *Log 10 of value*
 - [x] Length -> *Length of vec3, length of a number is equal to it's absolute value*
 - [ ] Unitize -> *Divide the value through it's length*
 - [ ] All swizzling ops -> *xy, xz, ..., xzy, yzx, ..., wzxy, ywwz, ..., wwww*

Other *un-pure* operators, which have an arity higher than 1, but we can store the additional information needed to compute them inside their enum variant. The condition for this is, that the additional information can be reprsented through f32s, which is why f.e. `Add`, or `Cross` don't make this list. If implemented like this, those operators can **not** have variables for their additional information, which makes them less flexible as the `BinaryOperators`.

 - [ ] Clamp(min, max) -> *Clamps the value between min and max*

### Binary Ops

Here it becomes even more complicated, many binary ops only make sense when used purely on numbers, or purely on vec3s.
Maybe we need a system to divide them more clearly.

 - [x] Add
 - [x] Sub
 - [x] Mul
 - [ ] Div
 - [x] Min
 - [x] Max
 - [ ] Atan2 -> This only makes sense for numbers, so we need to decide how we sample-down vecs here.
 - [ ] Pow -> Also makes no sense for vecs...
 - [ ] NthRoot -> Same, same