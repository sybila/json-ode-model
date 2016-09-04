extern crate ode_model;
extern crate rustc_serialize;
extern crate json_utils;

use ode_model::*;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use json_utils::FromJson;
use std::fs::File;

// Note: If test fails, first check if floating point numbers match.

#[test]
fn sample_model() {
    let model = OdeModel {
        name: "Sample Model".to_string(),
        variables: vec!(
            Variable {
                name: "var1".to_string(), range: Range { min: 0.0, max: 10.0 },
                thresholds: vec!(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0),
                var_points: Some(VarPoints { point_count: 100, segment_count: 10 }),
                equation: vec!(
                    Summand {
                        constant: 2.0, variable_indices: vec!(),
                        parameter_indices: vec!(), evaluables: vec!()
                    },
                    Summand {
                        constant: 1.0, variable_indices: vec!(1),
                        parameter_indices: vec!(0, 1), evaluables: vec!()
                    },
                    Summand {
                        constant: 1.0, variable_indices: vec!(0),
                        parameter_indices: vec!(), evaluables: vec!(
                            Evaluable::Hill { variable_index: 0, theta: 1.2, n: 5.0, a: 1.0, b: 2.0 },
                            Evaluable::Sigmoid { variable_index: 1, theta: 2.2, k: 5.0, a: 1.0, b: 2.0 },
                            Evaluable::Step { variable_index: 1, theta: 0.9, a: 1.0, b: 2.0 },
                            Evaluable::Ramp { variable_index: 0, low: 1.2, high: 3.3, a: 1.0, b: 2.0 },
                            Evaluable::RampApproximation { variable_index: 1, approximation: vec!(
                                Point { threshold: 2.0, value: 5.0 },
                                Point { threshold: 3.0, value: 8.0 },
                                Point { threshold: 4.0, value: 6.0 },
                                Point { threshold: 6.0, value: 7.0 }
                            )}
                        )
                    }
                )
            }, Variable {
                name: "var2".to_string(), range: Range { min: 1.0, max: 15.0 },
                thresholds: vec!(2.0, 3.0, 4.0, 5.0),
                var_points: None,
                equation: vec!(
                    Summand {
                        constant: 1.0, variable_indices: vec!(),
                        parameter_indices: vec!(), evaluables: vec!()
                    }
                )
            }
        ),
        parameters: vec!(
            Parameter { name: "p1".to_string(), range: Range { min: 1.0, max: 2.0 }},
            Parameter { name: "p2".to_string(), range: Range { min: 1.5, max: 2.8 }}
        )
    };

    //check identity
    let my_json = model.to_json();
    assert_eq!(model, OdeModel::from_json(&my_json).unwrap());

    let mut file = File::open("tests/input_files/sample_model.json").unwrap();
    let json = Json::from_reader(&mut file).unwrap();
    let parsed = OdeModel::from_json(&json).unwrap();
    assert_eq!(model, parsed);
}

#[test]
#[should_panic(expected= "UnknownVariantError(\"unknown\")")]
fn unknown_evaluable() {
    let mut file = File::open("tests/input_files/error_model.json").unwrap();
    let json = Json::from_reader(&mut file).unwrap();
    OdeModel::from_json(&json).unwrap();
}
