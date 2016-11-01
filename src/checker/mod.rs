use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

use super::formula::Formula;
use super::formula::Proposition;
use super::generator::*;
use super::model::Model;
use super::model::StateID;
use super::model::Facet;
use super::model::TimeFlow;
use super::parameters::Colors;

use super::formula::Formula::*;

pub struct CheckerContext<C: Colors> {
    pub model: Model,
    pub facet_cache: HashMap<Facet, C>,
    pub edge_cache: HashMap<(StateID, TimeFlow), StateSet2<C>>
}

impl <C: Colors> CheckerContext<C> {
    pub fn new(model: Model) -> CheckerContext<C> {
        CheckerContext {
            facet_cache: HashMap::new(),
            edge_cache: HashMap::new(),
            model: model
        }
    }
}

pub fn check<C: Colors + Debug + Clone>(ctx: &mut CheckerContext<C>, formula: &Formula) -> StateSet2<C> {
    match formula {
        &Atom(ref prop) => initial_states::<C>(&ctx.model, prop),
        &Not(ref prop) => {
            let mut result = HashMap::new();
            let inner = check::<C>(ctx, prop);
            //println!["Inner: {:?}", inner.len()];
            for (state, colors) in initial_states::<C>(&ctx.model, &Proposition::True) {
                if let Some(c) = inner.get(&state) {
                    let color_result = colors.and(&c.not()).and(&C::model_bounds(&ctx.model));
                    if !color_result.is_empty() {
                        result.insert(state, color_result);
                    }
                } else {
                    result.insert(state, colors);
                }
            }
            result
        }
        &And(ref p1, ref p2) => {
            let p1 = check::<C>(ctx, p1);
            let p2 = check::<C>(ctx, p2);
            //println!["AND: {:?} {:?}", p1, p2];
            let mut result = HashMap::new();
            for (state, colors) in p1 {
                if let Some(c) = p2.get(&state) {
                    let color_result = colors.and(c).and(&C::model_bounds(&ctx.model));
                    if !color_result.is_empty() {
                        result.insert(state, color_result);
                    }
                }
            }
            result
        }
        &EX(ref prop) => {
            let mut result = HashMap::new();
            for (state, colors) in check::<C>(ctx, prop) {
                let predecessors = compute_directed_edges::<C>(ctx, &state, &false);
                for (predecessor, edge_colors) in predecessors {
                    let pushed_over_edge = edge_colors.and(&colors).and(&C::model_bounds(&ctx.model));
                    safe_add(&mut result, predecessor, pushed_over_edge);
                }
            }
            result
        }
        &EU(ref path, ref reach) => {
            let path = check::<C>(ctx, path);
            //println!["Path: {:?}", path.len()];
            let mut result = HashMap::new();
            let mut queue = vec![];
            //println!["Reach: {:?}", check::<C>(model, reach)];
            for (state, colors) in check(ctx, reach) {
                queue.push(state.clone());
                result.insert(state, colors);
            }
            while let Some(state) = queue.pop() {
                //println!["Process {:?}", result.get(&state).unwrap()];
                let predecessors = compute_directed_edges::<C>(ctx, &state, &false);
                for (predecessor, edge_colors) in predecessors {
                    let pushed_over_edge = edge_colors.and(result.get(&state).unwrap());
                    let valid_for_path = pushed_over_edge.and(
                    path.get(&predecessor).unwrap_or(&C::ff())
                    ).and(&C::model_bounds(&ctx.model));
                    if safe_add(&mut result, predecessor, valid_for_path) {
                        queue.push(predecessor);
                    }
                }
            }
            //println!["Result: {:?}", result.len()];
            result
        }
        &AU(ref path, ref reach) => {
            let path = check::<C>(ctx, path);
            let mut result = HashMap::new();
            let mut queue = vec![];
            let mut uncovered: HashMap<StateID, StateSet2<C>> = HashMap::new();
            for (state, colors) in check(ctx, reach) {
                queue.push(state.clone());
                result.insert(state, colors);
            }
            while let Some(state) = queue.pop() {
                let predecessors = compute_directed_edges::<C>(ctx, &state, &false);
                for (predecessor, edge_colors) in predecessors {
                    let pushed_over_edge = edge_colors.and(result.get(&state).unwrap());
                    let to_cover = uncovered.entry(predecessor.clone()).or_insert_with(|| {
                        compute_directed_edges::<C>(ctx, &predecessor, &true)
                    });
                    let new_uncovered = to_cover.get(&state).unwrap().and(&pushed_over_edge.not());
                    to_cover.insert(state, new_uncovered);
                    let total_uncovered = to_cover.values().fold(C::ff(), |acc, i| acc.or(i));
                    let valid_for_path = pushed_over_edge.and(&total_uncovered.not()).and(
                        path.get(&predecessor).unwrap_or(&C::ff())
                    ).and(&C::model_bounds(&ctx.model));
                    if safe_add(&mut result, predecessor, valid_for_path) {
                        queue.push(predecessor);
                    }
                }
            }
            result
        }
        //_ => HashMap::new()
    }
}

//Performs a union on old and new color sets.
//Returns true if the value for the key changes.
fn safe_add<C: Colors + Debug>(set: &mut StateSet2<C>, key: StateID, value: C) -> bool {
    if !value.is_empty() {
        match set.entry(key) {
            Entry::Vacant(o) => {
                //println!["Vacant!"];
                o.insert(value);
                true
            }
            Entry::Occupied(mut o) => {
                let new_colors = value.and(&o.get().not());
                //println!["Occupied {:?} and ! {:?} = {:?}", value, o.get(), new_colors];
                if !new_colors.is_empty() {
                    let union = value.or(o.get());
                    o.insert(union);
                    true
                } else { false }
            }
        }
    } else { false }
}