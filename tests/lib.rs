extern crate ode_model;
extern crate rustc_serialize;

use ode_model::FromJson;
use ode_model::Range;
use ode_model::OdeModel;
use ode_model::Parameter;
use rustc_serialize::json::ToJson;
use rustc_serialize::json;

#[test]
fn it_works() {
    let model = OdeModel {
        name: "Example Model".to_string(),
        parameters: vec![
            Parameter { name: "p1".to_string(), range: Range {
                min: 0.05, max: 0.45
            }}
        ]
    };
    let range = Range { min: 3.0, max: 4.0 };
    let json = range.to_json();
    println!("Serialized: {}", json.to_string());
    println!("Deserialized: {:?}", Range::from_json(&json));
}
