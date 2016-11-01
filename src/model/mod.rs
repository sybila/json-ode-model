pub mod evaluable;

use types::Interval;
use super::parameters::order_1::Order1;
use super::parameters::order_1::Clause;
use self::evaluable::Evaluable2;

pub type StateID = usize;
pub type Threshold = f64;
pub type TimeFlow = bool;   //true = forward, false = backward
pub type Direction = bool;  //true = up, false = down
pub type StateCoordinate = usize;
pub type VariableIndex = usize;
pub type ParameterIndex = usize;
pub type ThresholdIndex = usize;
pub type Vertex = Vec<ThresholdIndex>;
pub type State = Vec<StateCoordinate>;

//A struct that combines variable index and direction.
//These two values uniquely identify a face of a hypercube.
#[derive(PartialEq, Eq, Hash)]
pub struct Face(pub VariableIndex, pub Direction);

impl Face {
    /*fn as_positive(&self) -> Face {
        Face(self.0, true)
    }
    fn as_negative(&self) -> Face {
        Face(self.0, false)
    }*/
}

#[derive(PartialEq, Eq, Hash)]
pub struct Facet(pub StateID, pub Face);

impl Facet {
    pub fn direction(&self) -> Direction {
        let &Facet(_, Face(_, ref dir)) = self; *dir
    }
    pub fn variable_index(&self) -> VariableIndex {
        let &Facet(_, Face(ref i, _)) = self; *i
    }
}

// A "compiled" model stripped of all unnecessary information
#[derive(Clone)]
pub struct Model {
    pub parameter_bounds: Vec<Interval>,
    pub variables: Vec<Vec<Threshold>>,
    pub equations: Vec<Vec<Summand2>>,
    dimension_state_counts: Vec<usize>,
    dimension_multipliers: Vec<usize>
}

#[derive(Clone)]
pub struct Summand2 {
    pub multiplier: f64,
    pub variable_indices: Vec<VariableIndex>,
    pub parameter_indices: Vec<ParameterIndex>,
    pub functions: Vec<Evaluable2>
}

impl Model {

    pub fn new(
        parameter_bounds: Vec<Interval>,
        variables: Vec<Vec<Threshold>>,
        equations: Vec<Vec<Summand2>>
    ) -> Model {
        let dimension_state_counts = variables.iter().map(|t| t.len() - 1).collect::<Vec<usize>>();
        let mut dimension_multipliers = vec![1];
        for c in &dimension_state_counts {
            let next = *dimension_multipliers.last().unwrap() * c;
            dimension_multipliers.push(next);
        }
        Model {
            parameter_bounds: parameter_bounds,
            variables: variables,
            equations: equations,
            dimension_state_counts: dimension_state_counts,
            dimension_multipliers: dimension_multipliers
        }
    }

    //Returns derivation, denominator and possibly parameter index.
    //Assumes there is only one parameter in the equation.
    pub fn eval(&self, variable: &VariableIndex, at: &Vertex)
        -> (f64, f64, Option<ParameterIndex>)
    {
        self.equations[*variable].iter().fold((0.0, 0.0, None), |(dv, denom, param), summand| {
            let sum = summand.multiplier *
            summand.variable_indices.iter().fold(1.0, |sum, i| {
                //evaluate variables
                sum * self.variables[*i][at[*i]]
            }) * summand.functions.iter().fold(1.0, |sum, &Evaluable2(i, ref f) | {
                //evaluate functions
                sum * f.eval(&self.variables[i][at[i]])
            });
            if let Some(parameter_index) = summand.parameter_indices.first() {
                (dv, denom + sum, Some(*parameter_index))
            } else {
                (dv + sum, denom, param)
            }
        })
    }

    pub fn extract_threshold(&self, state: &StateID, face: &Face) -> ThresholdIndex {
        let &Face(dim, dir) = face;
        (state / self.dimension_multipliers[dim]) % self.dimension_state_counts[dim]
            + if dir { 1 } else { 0 }
    }

    pub fn extract_coordinate(&self, state: &StateID, d: &VariableIndex) -> StateCoordinate {
        (state / self.dimension_multipliers[*d]) % self.dimension_state_counts[*d]
    }

    pub fn upper_neighbour(&self, state: &StateID, d: &VariableIndex) -> Option<StateID> {
        let coordinate = (state / self.dimension_multipliers[*d]) % self.dimension_state_counts[*d];
        if coordinate == self.dimension_state_counts[*d] - 1 { None } else {
            Some(state + self.dimension_multipliers[*d])
        }
    }

    pub fn lower_neighbour(&self, state: &StateID, d: &VariableIndex) -> Option<StateID> {
        let coordinate = (state / self.dimension_multipliers[*d]) % self.dimension_state_counts[*d];
        if coordinate == 0 { None } else {
            Some(state - self.dimension_multipliers[*d])
        }
    }

    pub fn encode_state(&self, state: &State) -> StateID {
        state.iter().enumerate().fold(0, |acc, (i, e)| {
            acc + self.dimension_multipliers[i] * e
        })
    }

    pub fn full_order_1_colors(&self) -> Order1 {
        Order1(vec![Clause(self.parameter_bounds.clone())])
    }

    pub fn expand_state(&self, state: &StateID) -> Vec<(f64, f64)> {
        (0..self.variables.len()).map(|i| {
            let c = self.extract_coordinate(state, &i);
            (self.variables[i][c], self.variables[i][c+1])
        }).collect()
    }

}