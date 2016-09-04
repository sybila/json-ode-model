extern crate json_utils;
extern crate rustc_serialize;

use json_utils::{FromJson, JsonMap, create_object, as_object};
use rustc_serialize::json::{ToJson, Json, DecoderError};

use Evaluable::*;

// ============================= ODE Model =========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct OdeModel {
    pub name: String,
    pub variables: Vec<Variable>,
    pub parameters: Vec<Parameter>
}

impl ToJson for OdeModel {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("name", &self.name);
            map.write_item("variables", &self.variables);
            map.write_item("parameters", &self.parameters);
        })
    }
}

impl FromJson<OdeModel> for OdeModel {
    fn from_json(json: &Json) -> Result<OdeModel, DecoderError> {
        as_object(json, |map| {
            map.read_item::<String>("name").and_then(|name| {
            map.read_item::<Vec<Parameter>>("parameters").and_then(|params| {
            map.read_item::<Vec<Variable>>("variables").and_then(|vars| {
                Ok(OdeModel { name: name, parameters: params, variables: vars })
            }) }) })
        })
    }
}

// ============================= Variable =========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub range: Range,
    pub thresholds: Vec<f64>,
    pub var_points: Option<VarPoints>,
    pub equation: Vec<Summand>
}

impl ToJson for Variable {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("name", &self.name);
            map.write_item("range", &self.range);
            map.write_item("thresholds", &self.thresholds);
            map.write_item("varPoints", &self.var_points);
            map.write_item("equation", &self.equation);
        })
    }
}

impl FromJson<Variable> for Variable {
    fn from_json(json: &Json) -> Result<Variable, DecoderError> {
        as_object(json, |map| {
            map.read_item::<String>("name").and_then(|n| {
            map.read_item::<Range>("range").and_then(|r| {
            map.read_item::<Vec<f64>>("thresholds").and_then(|th| {
            map.read_item::<Option<VarPoints>>("varPoints").and_then(|vp| {
            map.read_item::<Vec<Summand>>("equation").and_then(|eq| {
                Ok(Variable { name: n, range: r, thresholds: th, var_points: vp, equation: eq})
            }) }) }) }) })
        })
    }
}

// ============================= Var Points ========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct VarPoints {
    pub point_count: u64,
    pub segment_count: u64
}

impl ToJson for VarPoints {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("pointCount", &self.point_count);
            map.write_item("segmentCount", &self.segment_count);
        })
    }
}

impl FromJson<VarPoints> for VarPoints {
    fn from_json(json: &Json) -> Result<VarPoints, DecoderError> {
        as_object(json, |map| {
            map.read_item::<u64>("pointCount").and_then(|pc| {
            map.read_item::<u64>("segmentCount").and_then(|sc| {
                Ok(VarPoints { point_count: pc, segment_count: sc })
            }) })
        })
    }
}

// ============================= Parameter =========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub range: Range
}

impl ToJson for Parameter {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("name", &self.name);
            map.write_item("range", &self.range);
        })
    }
}

impl FromJson<Parameter> for Parameter {
    fn from_json(json: &Json) -> Result<Parameter, DecoderError> {
        as_object(json, |map| {
            map.read_item::<String>("name").and_then(|name| {
            map.read_item::<Range>("range").and_then(|range| {
                Ok(Parameter { name: name, range: range })
            }) })
        })
    }
}

// ============================= Range =============================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl ToJson for Range {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("min", &self.min);
            map.write_item("max", &self.max);
        })
    }
}

impl FromJson<Range> for Range {
    fn from_json(json: &Json) -> Result<Range, DecoderError> {
        as_object(json, |map| {
            map.read_item::<f64>("min").and_then(|min| {
            map.read_item::<f64>("max").and_then(|max| {
                Ok(Range { min: min, max: max} )
            }) })
        })
    }
}

// ============================= Summand ===========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Summand {
    pub constant: f64,
    pub variable_indices: Vec<usize>,
    pub parameter_indices: Vec<usize>,
    pub evaluables: Vec<Evaluable>
}

impl ToJson for Summand {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("constant", &self.constant);
            map.write_item("variableIndices", &self.variable_indices);
            map.write_item("parameterIndices", &self.parameter_indices);
            map.write_item("evaluables", &self.evaluables);
        })
    }
}

impl FromJson<Summand> for Summand {
    fn from_json(json: &Json) -> Result<Summand, DecoderError> {
        as_object(json, |map| {
            map.read_item::<f64>("constant").and_then(|c| {
            map.read_optional_item::<Vec<u64>>("variableIndices").and_then(|vi| {
            map.read_optional_item::<Vec<u64>>("parameterIndices").and_then(|pi| {
            map.read_optional_item::<Vec<Evaluable>>("evaluables").and_then(|eval| {
                Ok(Summand { constant: c,
                    variable_indices: vi.unwrap_or(vec!()).iter().map(|i| *i as usize).collect(),
                    parameter_indices: pi.unwrap_or(vec!()).iter().map(|i| *i as usize).collect(),
                    evaluables: eval.unwrap_or(vec!())
                })
            }) }) }) })
        })
    }
}

// ============================= Evaluable =========================================================

#[derive(Debug, PartialEq, Clone)]
pub enum Evaluable {
    Hill { variable_index: usize, theta: f64, n: f64, a: f64, b: f64 },
    Sigmoid { variable_index: usize, theta: f64, k: f64, a: f64, b: f64 },
    Step { variable_index: usize, theta: f64, a: f64, b: f64 },
    Ramp { variable_index: usize, low: f64, high: f64, a: f64, b: f64 },
    RampApproximation { variable_index: usize, approximation: Vec<Point> }
}

impl ToJson for Evaluable {
    fn to_json(&self) -> Json {
        create_object(|map| {
            match self {
                &Hill { variable_index, theta, n, a, b } => {
                    map.write_item("type", &"hill".to_string());
                    map.write_item("variableIndex", &variable_index);
                    map.write_item("theta", &theta);
                    map.write_item("n", &n);
                    map.write_item("a", &a);
                    map.write_item("b", &b);
                }
                &Sigmoid { variable_index, theta, k, a, b } => {
                    map.write_item("type", &"sigmoid".to_string());
                    map.write_item("variableIndex", &variable_index);
                    map.write_item("theta", &theta);
                    map.write_item("k", &k);
                    map.write_item("a", &a);
                    map.write_item("b", &b);
                }
                &Step { variable_index, theta, a, b } => {
                    map.write_item("type", &"step".to_string());
                    map.write_item("variableIndex", &variable_index);
                    map.write_item("theta", &theta);
                    map.write_item("a", &a);
                    map.write_item("b", &b);
                }
                &Ramp { variable_index, low, high, a, b } => {
                    map.write_item("type", &"ramp".to_string());
                    map.write_item("variableIndex", &variable_index);
                    map.write_item("lowThreshold", &low);
                    map.write_item("highThreshold", &high);
                    map.write_item("a", &a);
                    map.write_item("b", &b);
                }
                &RampApproximation { variable_index, ref approximation } => {
                    map.write_item("type", &"ramp_approximation".to_string());
                    map.write_item("variableIndex", &variable_index);
                    map.write_item("approximation", approximation);
                }
            }
        })
    }
}

impl FromJson<Evaluable> for Evaluable {
    fn from_json(json: &Json) -> Result<Evaluable, DecoderError> {
        as_object(json, |map| {
            map.read_item::<String>("type").and_then(|t| {
                match t.as_ref() {
                    "hill" => {
                        map.read_item::<u64>("variableIndex").and_then(|vi| {
                        map.read_item::<f64>("theta").and_then(|t| {
                        map.read_item::<f64>("n").and_then(|n| {
                        map.read_item::<f64>("a").and_then(|a| {
                        map.read_item::<f64>("b").and_then(|b| {
                            Ok(Hill { variable_index: vi as usize, theta: t, n: n, a: a, b: b })
                        }) }) }) }) })
                    }
                    "sigmoid" => {
                        map.read_item::<u64>("variableIndex").and_then(|vi| {
                        map.read_item::<f64>("theta").and_then(|t| {
                        map.read_item::<f64>("k").and_then(|k| {
                        map.read_item::<f64>("a").and_then(|a| {
                        map.read_item::<f64>("b").and_then(|b| {
                            Ok(Sigmoid { variable_index: vi as usize, theta: t, k: k, a: a, b: b })
                        }) }) }) }) })
                    }
                    "step" => {
                        map.read_item::<u64>("variableIndex").and_then(|vi| {
                        map.read_item::<f64>("theta").and_then(|t| {
                        map.read_item::<f64>("a").and_then(|a| {
                        map.read_item::<f64>("b").and_then(|b| {
                            Ok(Step { variable_index: vi as usize, theta: t, a: a, b: b })
                        }) }) }) })
                    }
                    "ramp" => {
                        map.read_item::<u64>("variableIndex").and_then(|vi| {
                        map.read_item::<f64>("lowThreshold").and_then(|lt| {
                        map.read_item::<f64>("highThreshold").and_then(|ht| {
                        map.read_item::<f64>("a").and_then(|a| {
                        map.read_item::<f64>("b").and_then(|b| {
                            Ok(Ramp { variable_index: vi as usize, low: lt, high: ht, a: a, b: b })
                        }) }) }) }) })
                    }
                    "ramp_approximation" => {
                        map.read_item::<u64>("variableIndex").and_then(|vi| {
                        map.read_item::<Vec<Point>>("approximation").and_then(|a| {
                           Ok(RampApproximation { variable_index: vi as usize, approximation: a })
                        }) })
                    }
                    other => Err(DecoderError::UnknownVariantError(other.to_string()))
                }
            })
        })
    }
}

// ============================= Point =============================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub threshold: f64,
    pub value: f64
}

impl ToJson for Point {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("threshold", &self.threshold);
            map.write_item("value", &self.value);
        })
    }
}

impl FromJson<Point> for Point {
    fn from_json(json: &Json) -> Result<Point, DecoderError> {
        as_object(json, |map| {
            map.read_item::<f64>("threshold").and_then(|tr| {
            map.read_item::<f64>("value").and_then(|val| {
               Ok(Point { threshold: tr, value: val })
            }) })
        })
    }
}