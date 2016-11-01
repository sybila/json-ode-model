use super::super::model::Vertex;
use super::super::model::VariableIndex;
use super::super::model::evaluable::Evaluable2;
use super::super::model::Model;

extern crate z3;
use self::z3::{Context, Ast, Solver};

use super::Colors;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

pub struct OrderN<'ctx>(Ast<'ctx>);

//Incredibly awful, but works...
pub static mut Z3: *const Context = 0 as *const Context;


impl<'ctx> OrderN<'ctx> {
    pub fn new(ast: Ast<'ctx>) -> OrderN<'ctx> {
        OrderN(ast)
    }
}

impl<'ctx> Debug for OrderN<'ctx> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("debug-undefined")
    }
}

impl<'ctx> PartialEq for OrderN<'ctx> {
    fn eq(&self, other: &OrderN<'ctx>) -> bool {
        if self.is_empty() {
            other.is_empty()
        } else {
            self.and(&other.not()).is_empty()
        }
    }
}

impl<'ctx> Colors for OrderN<'ctx> {
    fn tt() -> Self {
        unsafe {
            OrderN((*Z3).from_bool(true))
        }
    }

    fn ff() -> Self {
        unsafe {
            OrderN((*Z3).from_bool(false))
        }
    }

    fn divide(model: &Model, variable_index: &VariableIndex, vertex: &Vertex) -> (Self, Self) {
        let ref equation = model.equations[*variable_index];
        //TODO: This is bullshit!
        let ref thresholds = model.variables[*variable_index];
        let param_count = model.parameter_bounds.len();
        let mut sums = vec![0.0; param_count + 1];
        for summand in equation {
            let mut result = summand.multiplier;
            for var in &summand.variable_indices {
                result *= thresholds[vertex[*var]];
            }
            for &Evaluable2(ref var, ref f) in &summand.functions {
                result *= f.eval(&thresholds[vertex[*var]]);
            }
            if let Some(index) = summand.parameter_indices.first() {
                sums[*index] += result;
            } else {
                sums[param_count] += result;
            }
        }
        unsafe {
            let constant = sums[param_count];
            let precision = 1000000;
            let mut polynom = (*Z3).from_real((constant * precision as f64) as i32, precision);
            for i in 0..param_count {
                let p = (*Z3).numbered_real_const(i as u32);
                let c = (*Z3).from_real((sums[i] * precision as f64) as i32, precision);
                polynom = polynom.add(&[&p.mul(&[&c])])
            };
            let zero = (*Z3).from_real(0, 1);
            (OrderN(polynom.lt(&zero)), OrderN(polynom.gt(&zero)))
        }
    }

    fn or(&self, other: &Self) -> Self {
        OrderN(self.0.or(&[&other.0]))
    }

    fn and(&self, other: &Self) -> Self {
        OrderN(self.0.and(&[&other.0]))
    }

    fn not(&self) -> Self {
        OrderN(self.0.not())
    }

    fn optimize(&self) -> Self {
        OrderN(self.0.not().not())
    }

    fn is_empty(&self) -> bool {
        unsafe {
            let solver = Solver::new(&*Z3);
            solver.assert(&self.0);
            !solver.check()
        }
    }
    fn model_bounds(model: &Model) -> Self {
        unimplemented!()
    }
}