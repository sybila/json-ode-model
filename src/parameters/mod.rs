//What we need:
//And, or - create new from previous two
//Not - same
//EX:
//Copy, intersect on edge, union.
//EU:
//Copy, intersect on edge, intersect on path, union and if changed, propagate.
//AU:
//Copy, intersect on edge, subtract from uncovered edge, union all uncovered, negate, union
//and if changed, propagate.
// Negated union over edge - colors:
// C(t) = !Or_{s,c \in succ(t)} c and !C_s()
// C(t) = And_{s,c \in succ(t)} !(c and !C_s())
// C(t) = And_{s,c \in succ(t)} C_s() or !c
//NO: C_t() = And_{s,c \in succ(t)} C_s() and c
//t -> s1, t -> s2
//h1 = {1,2} -!> {3}
//h2 = {2,3} -!> {1}
//s1 = {1}, s2 = {2,3}

pub mod order_0;
pub mod order_1;
pub mod order_n;

use super::model::Vertex;
use super::model::Model;
use super::model::VariableIndex;

/// Colors represent a set of constrains imposed on the parametric space P.
/// The form of these constrains is left to the implementation, however,
/// they should follow the basic rules of boolean algebra as described
/// for each method.
///
/// Note: Space P is not related to any model! This part is handled by
/// the model checker.
///
/// From mathematical point of view, each Colors structure is a set.
/// Therefore it should implement at least a semantic partial equality.
/// This can be sometimes difficult to compute, but is is necessary
/// for model checking. It can be implemented using other
/// required methods, but it might not be the best option.
pub trait Colors : PartialEq + Sized {

    /// Return a new instance of the true constrain, that is:
    /// tt = { p | p \in P }
    fn tt() -> Self;

    /// Return a new instance if the false constrain, that is:
    /// ff = { p | p \in empty_set }
    fn ff() -> Self;

    /// Divide the parameter space into two areas. One where given
    /// equation is negative and one where it is positive.
    fn divide(model: &Model, variable_index: &VariableIndex, vertex: &Vertex) -> (Self, Self);

    fn model_bounds(model: &Model) -> Self;

    /// A logical disjunction of two parameter constrains.
    /// A or B = { p | p \in A || p \in B }
    fn or(&self, other: &Self) -> Self;

    /// A logical conjunction of two parameter constrains.
    /// A and B = { p | p \in A && p \in B }
    fn and(&self, other: &Self) -> Self;

    /// A logical negation of a parameter constrain.
    /// not A = { p | p \in P && p \not\in A}
    fn not(&self) -> Self;

    /// This method can be used to optimize/compress the constrain
    /// representation. Implementation can perform minor, useful optimizations
    /// when executing each operator, but more complex procedures should be
    /// delegated to this function call.
    fn optimize(&self) -> Self;

    fn is_empty(&self) -> bool;

    fn is_not_empty(&self) -> bool { !self.is_empty() }

}