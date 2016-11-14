extern crate ode_model;
extern crate rustc_serialize;
extern crate json_utils;

extern crate z3;
use self::z3::{Context, Config};

use std::collections::HashMap;
use std::collections::HashSet;

use rustc_serialize::json::{ToJson, Json, DecoderError};
use json_utils::{FromJson, as_object, JsonMap, create_object};
//use json_utils::{FromJson, JsonMap, , as_object};

use std::env;
use ode_model::*;
use ode_model::model::*;
use ode_model::generator::StateSet2;
use std::fs::File;
use ode_model::formula::Formula;
use ode_model::checker::check;
use ode_model::checker::CheckerContext;

use ode_model::parameters::Colors;
use ode_model::parameters::order_1::*;
use ode_model::parameters::order_n::*;
use ode_model::types::Interval;
use ode_model::generator::compute_directed_edges;

use ode_model::generator::EDGE_HIT;
use ode_model::generator::TOTAL_EDGE;

fn main() {
    //Init global unique Z3 that will outlive everyone!
    let z3 = Context::new(&Config::new());
    unsafe {
        Z3 = &z3 as *const Context;
    }
    let args: Vec<_> = env::args().collect();
    let mut config_file = File::open(args[1].clone()).unwrap();
    //let mut model_file = File::open(args[0].clone()).unwrap();
    //let mut property_file = File::open(args[2].clone()).unwrap();
    //let prop = Formula::from_json(&Json::from_reader(&mut property_file).unwrap()).unwrap();
    let config = Config2::from_json(&Json::from_reader(&mut config_file).unwrap()).unwrap();
    let full_model = config.model;
    let model = full_model.compile();
    let mut ctx = CheckerContext::new(model.clone());
    let result = config.formulas.iter().map(|&AbstractPair { ref first, ref second }| {
        println!("Verify: {:?}", second);
        (first.clone(), check::<Order1>(&mut ctx, second))
    }).collect::<Vec<(String, StateSet2<Order1>)>>();
    //println!["Verify: {:?}", formula];
    //println!["Model: {:?}", full_model];
    print_results(&model, &full_model, &result);
    //println!["{:?}", result.len()];
    /*unsafe {
        println!["Cache hit: {:?}/{:?}", EDGE_HIT, TOTAL_EDGE];
    }*/
}

struct Config2 {
    model: OdeModel,
    formulas: Vec<AbstractPair>
}

struct AbstractPair {
    first: String,
    second: Formula
}

impl FromJson<Config2> for Config2 {
    fn from_json(data: &Json) -> Result<Config2, DecoderError> {
        as_object(data, |map| {
            Ok(Config2 {
                model: try![map.read_item::<OdeModel>("model")],
                formulas: try![map.read_item::<Vec<AbstractPair>>("formulas")]
            })
        })
    }
}

impl FromJson<AbstractPair> for AbstractPair {
    fn from_json(data: &Json) -> Result<AbstractPair, DecoderError> {
        as_object(data, |map| {
            Ok(AbstractPair {
                first: try![map.read_item::<String>("first")],
                second: try![map.read_item::<Formula>("second")]
            })
        })
    }
}

fn print_results(model: &Model, full_model: &OdeModel, result: &Vec<(String, StateSet2<Order1>)>) {
    let mut color_counter: usize = 0;
    let mut state_counter: usize = 0;
    let mut colors = vec![];
    let mut color_indices: HashMap<Order1, usize> = HashMap::new();
    let mut states = vec![];
    let mut state_indices: HashMap<StateID, usize> = HashMap::new();
    for &(ref name, ref data) in result {
        for (state, params) in data {
            if !color_indices.contains_key(params) {
                color_indices.insert(params.clone(), color_counter);
                colors.push(params.clone());
                color_counter += 1;
            }
            if !state_indices.contains_key(state) {
                //print!["{:?},", state];
                state_indices.insert(state.clone(), state_counter);
                states.push(OutState {
                    id: state.clone(),
                    bounds: model.expand_state(state)
                });
                state_counter += 1;
            }
        }
    }
    //println!["{:?}", colors.to_json().to_string()];
    //println!["{:?}", states.to_json().to_string()];
    let k = ResultSet {
        variables: full_model.variables.iter().map(|i| i.name.clone()).collect(),
        parameters: full_model.parameters.iter().map(|i| i.name.clone()).collect(),
        _type: "rectangular".to_string(),
        thresholds: model.variables.clone(),
        states: states,
        params: colors,
        results: result.iter().map(|&(ref name, ref data)| {
            FormulaResult {
                formula: name.clone(),
                data: data.iter().map(|(s, p)| {
                    R {
                        state: state_indices[s],
                        param: color_indices[p]
                    }
                }).collect()
            }
        }).collect()
    };
    println!["{}", k.to_json()];
}

struct ResultSet {
    variables: Vec<String>,
    parameters: Vec<String>,
    _type: String,
    thresholds: Vec<Vec<f64>>,
    states: Vec<OutState>,
    params: Vec<Order1>,
    results: Vec<FormulaResult>
}

impl ToJson for ResultSet {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("variables", &self.variables);
            map.write_item("thresholds", &self.thresholds);
            map.write_item("type", &self._type);
            map.write_item("parameters", &self.parameters);
            map.write_item("states", &self.states);
            map.write_item("parameter_values", &self.params);
            map.write_item("results", &self.results);
        })
    }
}

struct OutState {
    id: usize,
    bounds: Vec<(f64, f64)>
}

impl ToJson for OutState {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("id", &self.id);
            map.write_item("bounds", &self.bounds);
        })
    }
}

struct FormulaResult {
    formula: String,
    data: Vec<R>
}

impl ToJson for FormulaResult {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("formula", &self.formula);
            map.write_item("data", &self.data);
        })
    }
}

struct R {
    state: usize,
    param: usize
}

impl ToJson for R {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("state", &self.state);
            map.write_item("param", &self.param);
        })
    }
}