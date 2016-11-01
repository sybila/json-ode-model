use std::cmp::PartialOrd;
use std::cmp::Ordering;

use super::Colors;

use super::super::model::Vertex;
use super::super::model::Model;
use super::super::model::VariableIndex;
use super::super::model::evaluable::Evaluable2;

///A variant of Colors that is represented by a formula of inequality polynomials of order 0.
///That is: phi = k < 0 | k > 0 | phi1 and phi2 | phi1 or phi2.
///This case can be immediately evaluated and hence can be represented as boolean value.
#[derive(Debug, Clone, PartialEq)]
pub struct Order0(bool);

impl Colors for Order0 {
    fn tt() -> Self { Order0(true) }

    fn ff() -> Self { Order0(false) }

    fn divide(model: &Model, variable_index: &VariableIndex, vertex: &Vertex) -> (Self, Self) {
        let ref equation = model.equations[*variable_index];
        let ref thresholds = model.variables[*variable_index];
        let mut sum = 0.0;
        for summand in equation {
            if !summand.parameter_indices.is_empty() {
                panic!["Model contains parameters!"]
            } else {
                let mut result = summand.multiplier;
                for var in &summand.variable_indices {
                    result *= thresholds[vertex[*var]];
                }
                for &Evaluable2(ref var, ref f) in &summand.functions {
                    result *= f.eval(&thresholds[vertex[*var]]);
                }
                sum += result;
            }
        }
        if sum > 0.0 {
            (Order0(false), Order0(true))
        } else if sum < 0.0 {
            (Order0(true), Order0(false))
        } else {
            (Order0(false), Order0(false))
        }
    }

    fn or(&self, other: &Self) -> Self { Order0(self.0 || other.0) }

    fn and(&self, other: &Self) -> Self { Order0(self.0 && other.0) }

    fn not(&self) -> Self { Order0(!self.0) }

    fn is_empty(&self) -> bool { !self.0 }

    fn optimize(&self) -> Self { self.clone() }

    fn model_bounds(model: &Model) -> Self {
        Order0(true)
    }
}

impl PartialOrd<Order0> for Order0 {
    fn partial_cmp(&self, other: &Order0) -> Option<Ordering> {
        Some(if self.0 == other.0 {
            Ordering::Equal
        } else if !self.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}