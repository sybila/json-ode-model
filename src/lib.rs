extern crate rustc_serialize;

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson, Object};

//======================= Data definitions ========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct OdeModel {
    pub name: String,
    pub parameters: Vec<Parameter>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub range: Range,
    pub thresholds: Vec<f64>
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarPoints {
    pub point_count: i32,
    pub segment_count: i32
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub range: Range
}

#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

pub trait FromJson<T> {
    fn from_json(json: &Json) -> T;
}

// =============== Trait implementations ==========================================================

//Tod JSON

impl ToJson for OdeModel {
    fn to_json(&self) -> Json {
        make_object(|map| {
            add_item(map, "name", &self.name);
            add_item(map, "parameters", &self.parameters);
        })
    }
}

impl ToJson for Parameter {
    fn to_json(&self) -> Json {
        make_object(|map| {
            add_item(map, "name", &self.name);
            add_item(map, "range", &self.range);
        })
    }
}

impl ToJson for Range {
    fn to_json(&self) -> Json {
        make_object(|map| {
            add_item(map, "min", &self.min);
            add_item(map, "max", &self.max);
        })
    }
}

//From JSON

impl FromJson<Range> for Range {
    fn from_json(json: &Json) -> Range {
        read_object(json, |map| {
            let min = read_f64(map, "min").unwrap();
            let max = read_f64(map, "max").unwrap();
            Ok(Range { min: min, max: max })
        }).unwrap()
    }
}

// ============================= Helper functions =================================================


fn make_object<F>(builder: F) -> Json
    where F: Fn(&mut BTreeMap<String, Json>) {
    let mut map = BTreeMap::new();
    builder(&mut map);
    Json::Object(map)
}

//we use a special read_object (not read_item) so that we avoid copying
fn read_object<F, T>(json: &Json, reader: F) -> Result<T, String>
    where F: Fn(&Object) -> Result<T, String> {

    if let &Json::Object(ref map) = json { reader(map) } else {
        Err("Given JSON has a wrong type. Expected Object.".to_string())
    }
}

fn add_item<T : ToJson>(map: &mut BTreeMap<String, Json>, field: &str, value: &T) {
    map.insert(field.to_string(), value.to_json());
}

fn read_f64(map: &Object, key: &str) -> Result<f64, String> {
    read_item(map, key, "64bit float", |json| {
        if let &Json::F64(value) = json { Some(value) } else { None }
    })
}

fn read_string(map: &Object, key: &str) -> Result<String, String> {
    read_item(map, key, "String", |json| {
        if let &Json::String(ref value) = json { Some(value.clone()) } else { None }
    })
}

fn read_i64(map: &Object, key: &str) -> Result<i64, String> {
    read_item(map, key, "64bit integer", |json| {
        if let &Json::I64(value) = json { Some(value) } else { None }
    })
}

//Read a value of a key from a json map using provided getter.
//The expected string is used in error messages.
fn read_item<T, F>(map: &Object, key: &str, expected: &str, getter: F) -> Result<T, String>
    where F : Fn(&Json) -> Option<T> {
    let result = map.get(key);
    match result {
        Some(json) => {
            getter(json).as_result(||
                Err("Value for $key has a wrong type. Expected $expected".to_string())
            )
        }
        None => Err("Value for $key not found".to_string())
    }
}

//Compose an error message based on the result
fn read_error<T, S>(key: &str, result: &Option<S>, expected: &str) -> Result<T, String> {
    if result.is_some() {
        Err("Value for $key has a wrong type. Expected $expected".to_string())
    } else {
        Err("Value for $key not found".to_string())
    }
}
