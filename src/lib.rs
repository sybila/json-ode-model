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

impl OdeModel {

    /// Check validity and if the model is invalid, return the cause.
    pub fn is_valid(&self) -> Option<String> {
        self.parameters.iter().fold(None, |a, i| a.or_else(|| i.is_valid())).or_else(|| {
            if self.variables.is_empty() {
                Some("Model has no variables".to_string())
            } else { None }
        }).or_else(|| {
            self.variables.iter().fold(None, |a, i| a.or_else(|| i.is_valid(self)))
        })
    }

    pub fn is_multi_affine(&self) -> bool {
        self.variables.iter().all(|i| i.is_multi_affine())
    }
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
            Ok(OdeModel {
                name: try!(map.read_item::<String>("name")),
                parameters: try!(map.read_item::<Vec<Parameter>>("parameters")),
                variables: try!(map.read_item::<Vec<Variable>>("variables"))
            })
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

impl Variable {
    fn is_valid(&self, model: &OdeModel) -> Option<String> {
        self.range.is_valid().or_else(|| {
            if self.thresholds.is_empty() {
                Some(format!("Variable {} has no thresholds", self.name))
            } else { None }
        }).or_else(|| {
            if self.equation.is_empty() {
                Some(format!("Variable {} has an empty equation", self.name))
            } else { None }
        }).or_else(|| {
            self.equation.iter().fold(None, |a, i| a.or_else(|| i.is_valid(model)))
        })
    }

    fn is_multi_affine(&self) -> bool {
        self.equation.iter().all(|i| i.is_multi_affine())
    }
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
            Ok(Variable {
                name: try!(map.read_item::<String>("name")),
                range: try!(map.read_item::<Range>("range")),
                thresholds: try!(map.read_item::<Vec<f64>>("thresholds")),
                var_points: try!(map.read_optional_item::<VarPoints>("varPoints")),
                equation: try!(map.read_item::<Vec<Summand>>("equation"))
            })
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
            Ok(VarPoints {
                point_count: try!(map.read_item::<u64>("pointCount")),
                segment_count: try!(map.read_item::<u64>("segmentCount"))
            })
        })
    }
}

// ============================= Parameter =========================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub range: Range
}

impl Parameter {
    fn is_valid(&self) -> Option<String> {
        self.range.is_valid()
    }
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
            Ok(Parameter {
                name: try!(map.read_item::<String>("name")),
                range: try!(map.read_item::<Range>("range"))
            })
        })
    }
}

// ============================= Range =============================================================

#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    fn is_valid(&self) -> Option<String> {
        if self.min >= self.max {
            Some(format!("Empty range: {} >= {}", self.min, self.max))
        } else { None }
    }
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
            Ok(Range {
                min: try!(map.read_item::<f64>("min")),
                max: try!(map.read_item::<f64>("max"))
            })
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

impl Summand {
    fn is_valid(&self, model: &OdeModel) -> Option<String> {
        self.variable_indices.iter().fold(None, |a, i| {
            a.or_else(|| check_variable_index(*i, model))
        }).or_else(|| {
            self.parameter_indices.iter().fold(None, |a, i| {
                a.or_else(|| check_parameter_index(*i, model))
            })
        }).or_else(|| {
            self.evaluables.iter().fold(None, |a, i| {
                a.or_else(|| i.is_valid(model))
            })
        })
    }

    fn is_multi_affine(&self) -> bool {
        self.evaluables.iter().all(|i| i.is_multi_affine())
    }
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
            Ok(Summand {
                constant: try!(map.read_item::<f64>("constant")),
                variable_indices: try!(map.read_optional_item::<Vec<u64>>("variableIndices"))
                    .unwrap_or(vec!()).iter().map(|i| *i as usize).collect(),
                parameter_indices: try!(map.read_optional_item::<Vec<u64>>("parameterIndices"))
                    .unwrap_or(vec!()).iter().map(|i| *i as usize).collect(),
                evaluables: try!(map.read_optional_item::<Vec<Evaluable>>("evaluables"))
                    .unwrap_or(vec!())
            })
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

impl Evaluable {
    fn is_valid(&self, model: &OdeModel) -> Option<String> {
        match self {
            &Hill { variable_index, .. } => check_variable_index(variable_index, model),
            &Sigmoid { variable_index, .. } => check_variable_index(variable_index, model),
            &Step { variable_index, .. } => check_variable_index(variable_index, model),
            &Ramp { variable_index, .. } => check_variable_index(variable_index, model),
            &RampApproximation { variable_index, ref approximation } => {
                check_variable_index(variable_index, model).or_else(|| {
                    if approximation.is_empty() {
                        Some("RampApproximation with no points".to_string())
                    } else { None }
                })
            }
        }
    }

    fn is_multi_affine(&self) -> bool {
        match self {
            &Hill { .. } => false,
            &Sigmoid { .. } => false,
            &Step { .. } => true,
            &Ramp { .. } => true,
            &RampApproximation { .. } => true
        }
    }

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
                        Ok(Hill {
                            variable_index: try!(map.read_item::<u64>("variableIndex")) as usize,
                            theta: try!(map.read_item::<f64>("theta")),
                            n: try!(map.read_item::<f64>("n")),
                            a: try!(map.read_item::<f64>("a")),
                            b: try!(map.read_item::<f64>("b"))
                        })
                    }
                    "sigmoid" => {
                        Ok(Sigmoid {
                            variable_index: try!(map.read_item::<u64>("variableIndex")) as usize,
                            theta: try!(map.read_item::<f64>("theta")),
                            k: try!(map.read_item::<f64>("k")),
                            a: try!(map.read_item::<f64>("a")),
                            b: try!(map.read_item::<f64>("b"))
                        })
                    }
                    "step" => {
                        Ok(Step {
                            variable_index: try!(map.read_item::<u64>("variableIndex")) as usize,
                            theta: try!(map.read_item::<f64>("theta")),
                            a: try!(map.read_item::<f64>("a")),
                            b: try!(map.read_item::<f64>("b"))
                        })
                    }
                    "ramp" => {
                        Ok(Ramp {
                            variable_index: try!(map.read_item::<u64>("variableIndex")) as usize,
                            low: try!(map.read_item::<f64>("lowThreshold")),
                            high: try!(map.read_item::<f64>("highThreshold")),
                            a: try!(map.read_item::<f64>("a")),
                            b: try!(map.read_item::<f64>("b"))
                        })
                    }
                    "ramp_approximation" => {
                       Ok(RampApproximation {
                           variable_index: try!(map.read_item::<u64>("variableIndex")) as usize,
                           approximation: try!(map.read_item::<Vec<Point>>("approximation"))
                       })
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
           Ok(Point {
               threshold: try!(map.read_item::<f64>("threshold")),
               value: try!(map.read_item::<f64>("value"))
           })
        })
    }
}

// Utility stuff

fn check_variable_index(i: usize, model: &OdeModel) -> Option<String> {
    if i < model.variables.len() {
        Some(format!("Invalid variable index: {}", i))
    } else { None }
}

fn check_parameter_index(i: usize, model: &OdeModel) -> Option<String> {
    if i < model.parameters.len() {
        Some(format!("Invalid parameter index: {}", i))
    } else { None }
}