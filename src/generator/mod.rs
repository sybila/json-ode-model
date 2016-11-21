extern crate iterator_to_hash_map;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use super::model::*;
use super::parameters::Colors;
use super::parameters::order_1::Order1;
use super::formula::Proposition;
use super::formula::Proposition::*;
use super::formula::CompareOp;
use super::checker::CheckerContext;

use self::Progress::*;

pub type StateSet = HashMap<StateID, Order1>;
pub type StateSet2<C: Colors> = HashMap<StateID, C>;

pub fn initial_states<C: Colors + Debug>(model: &Model, proposition: &Proposition) -> StateSet2<C> {
   // println!["Initial states!"];
    match proposition {
        &False => HashMap::new(),
        &True => enumerate_states(model, &0, &0, &(model.variables[0].len() - 2)),
        &Equation(ref d, ref op, ref t) => {
            match op {
                &CompareOp::LT => enumerate_states(model, d, &0, &(t - 1)),
                &CompareOp::GT => enumerate_states(model, d, t, &(model.variables[*d].len() - 2))
            }
        }
    }
}

fn enumerate_states<C: Colors + Debug>(model: &Model, d: &VariableIndex, from: &StateCoordinate, to: &StateCoordinate)
    -> StateSet2<C> {
    if to < from {
        //println!["Empty!"];
        HashMap::new()
    } else {
        //let from: StateCoordinate = *from;
        //let to: StateCoordinate = *to - 1;
        let mut results = HashMap::new();
        //"stack" with progress
        let mut progress: Vec<i32> = vec![(-1); model.variables.len()];
        let mut stack_pointer: usize = 0;
        let mut state: State = vec![0; model.variables.len()];
        loop {
            if stack_pointer == model.variables.len() {
                let state_id = model.encode_state(&state);
                //println!["{:?} - {:?}", state, state_id];
                results.insert(state_id, C::tt());

                stack_pointer -= 1;
                while progress[stack_pointer] < 0 {
                    if let Some(sp) = stack_pointer.checked_sub(1) {
                        stack_pointer = sp;
                    } else {
                        //println!["Enumerated: {:?}", results];
                        return results;
                    }
                }
            } else {
                if progress[stack_pointer] < 0 {
                    progress[stack_pointer] = if stack_pointer == *d { *to } else {
                        model.variables[stack_pointer].len() - 2
                    } as i32
                }
                state[stack_pointer] = progress[stack_pointer] as usize;
                progress[stack_pointer] -= 1;
                if stack_pointer == *d && progress[stack_pointer] < *from as i32 {
                    progress[stack_pointer] = -1;
                }

                stack_pointer += 1;
            }
        }
    }
}

pub static mut TOTAL_EDGE: i32 = 0;
pub static mut EDGE_HIT: i32 = 0;

pub fn compute_directed_edges<C: Colors + Debug + Clone>(
    ctx: &mut CheckerContext<C>, from: &StateID, time_flow: &TimeFlow
) -> StateSet2<C> {
    /*unsafe {
        TOTAL_EDGE += 1;
    }*/
    match ctx.edge_cache.entry((*from, *time_flow)) {
        Entry::Vacant(o) => {
            let mut self_loop = C::tt();
            let mut result: StateSet2<C> = HashMap::new();
            for i in 0..ctx.model.variables.len() {
                let (high_in, high_out) = facet_colors::<C>(&ctx.model, &Facet(*from, Face(i, true)));
                let (low_out, low_in) = facet_colors::<C>(&ctx.model, &Facet(*from, Face(i, false)));

                //println!["high in:{:?} out:{:?}", high_in, high_out];
                //println!["low in:{:?} out:{:?}", low_in, low_out];

                let positive_flow: C = low_in.and(&high_out).and(&low_out.or(&high_in).not());
                let negative_flow: C = low_out.and(&high_in).and(&low_in.or(&high_out).not());

                if let Some(upper) = ctx.model.upper_neighbour(from, &i) {
                    self_loop = self_loop.and(&positive_flow.not());
                    let colors = if *time_flow { high_out } else { high_in };
                    if !colors.is_empty() { result.insert(upper, colors); };
                }
                if let Some(lower) = ctx.model.lower_neighbour(from, &i) {
                    self_loop = self_loop.and(&negative_flow.not());
                    let colors = if *time_flow { low_out } else { low_in };
                    if !colors.is_empty() { result.insert(lower, colors); };
                }
            }
            if !self_loop.is_empty() {
                result.insert(*from, self_loop);
            }
            o.insert(result.clone());
            result
        }
        Entry::Occupied(o) => {
            /*unsafe {
                EDGE_HIT += 1;
            }*/
            o.get().clone()
        }
    }
}

pub fn facet_colors<C: Colors + Debug>(model: &Model, facet: &Facet) -> (C,C) {
    fold_over_facet(model, facet, (C::ff(), C::ff()), |(down, up), vertex| {
        let (negative, positive) = C::divide(model, &facet.variable_index(), vertex);
        (down.or(&negative), up.or(&positive))
    })
}

///Compute a color set for which a flow exists through a given facet in specified time direction.
/*pub fn compute_facet_colors_order1(model: &Model, facet: &Facet, time_flow: &TimeFlow) -> Order1 {
    //folding with Option<(Option<usize>, (f64, f64))>, doing an union over all vertices.
    let result = fold_over_facet(model, facet, None, |state, vertex| {
        let (derivation, denominator, parameter_index) = model.eval(&facet.variable_index(), vertex);
        if parameter_index.is_none() || denominator == 0.0 {
            //No parameter or denominator, just look at the derivation
            if (derivation > 0.0 && facet.direction() ^ *time_flow) ||
                (derivation < 0.0 && facet.direction() == *time_flow) {
                state.or_else(|| Some((parameter_index, (NEG_INFINITY, INFINITY))))
            } else { state }
        } else {
            let time_flow = if denominator > 0.0 { time_flow.clone() } else { !*time_flow };
            let split = (-derivation) / denominator;
            if facet.direction() ^ time_flow {
                state.map_or_else(
                    || Some((parameter_index, (split, INFINITY))),    //default
                    |(current_index, (low, high))| {
                        Some((current_index.or(parameter_index), (low.min(split), high)))
                    }
                )
            } else if facet.direction() == time_flow {
                state.map_or_else(
                    || Some((parameter_index, (NEG_INFINITY, split))),    //default
                    |(current_index, (low, high))| {
                        Some((current_index.or(parameter_index), (low, high.max(split))))
                    }
                )
            } else { state }
        }
    });

    if let Some((parameter_index, (lower_bound, upper_bound))) = result {
        if let Some(i) = parameter_index {
            let mut clause = model.parameter_bounds.clone();
            let Interval(min, max) = clause[i];
            clause[i] = Interval(min.max(lower_bound), max.min(upper_bound));
            Order1(vec![Clause(clause)])
        } else { Order1::tt() }
    } else { Order1::ff() }
}*/

//Internal structure used in fold_over_facet
#[derive(Debug, PartialEq, Eq, Clone)]
enum Progress { DoNegative, DoPositive, Done }

///Fold over all vertices of one facet. A facet is one face of the state hypercube.
fn fold_over_facet<B, F>(model: &Model, facet: &Facet, init: B, mut f: F) -> B
    where F: FnMut(B, &Vertex) -> B {
    let &Facet(ref state, ref face) = facet;
    let &Face(ref variable_index, _) = face;
    //"stack" with progress
    let mut progress: Vec<Progress> = vec![DoNegative; model.variables.len()];
    let mut stack_pointer: usize = 0;
    //Vertex that will be passed to the caller
    let mut vertex = vec![0; model.variables.len()];
    let mut result = init;
    while progress[0] != Done {
        while stack_pointer < progress.len() {
            if stack_pointer == *variable_index {
                vertex[stack_pointer] = model.extract_threshold(state, face);
                //println!["Extract var {:?}", vertex[stack_pointer]];
                progress[stack_pointer] = Done;
            } else {
                match progress[stack_pointer] {
                    DoNegative => {
                        vertex[stack_pointer]
                            = model.extract_threshold(state, &Face(stack_pointer, false));
                        //println!["Extract negative {:?}", vertex[stack_pointer]];
                        progress[stack_pointer] = DoPositive;
                    }
                    DoPositive => {
                        vertex[stack_pointer]
                            = model.extract_threshold(&state, &Face(stack_pointer, true));
                        //println!["Extract positive {:?}", vertex[stack_pointer]];
                        progress[stack_pointer] = Done;
                    }
                    Done => panic!["Done not allowed here."]
                }
            }
            stack_pointer += 1;
        }
        //println!["Vertex {:?} {:?}", vertex, progress];
        result = f(result, &vertex);
        //Reset everything that is already Done (except the first element)
        stack_pointer -= 1; //move to valid index
        while stack_pointer > 0 && progress[stack_pointer] == Done {
            progress[stack_pointer] = DoNegative;
            stack_pointer -= 1;
        }
    }
    result
}
