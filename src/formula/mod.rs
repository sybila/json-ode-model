extern crate json_utils;
extern crate rustc_serialize;

use rustc_serialize::json::{Json, DecoderError};
use json_utils::{FromJson, JsonMap, as_object};

use super::model::VariableIndex;
use super::model::ThresholdIndex;

#[derive(Debug)]
pub enum CompareOp {
    LT, GT
}

#[derive(Debug)]
pub enum Proposition {
    True, False,
    Equation(VariableIndex, CompareOp, ThresholdIndex)
}

#[derive(Debug)]
pub enum Formula {
    Atom(Proposition),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    EX(Box<Formula>),
    EU(Box<Formula>, Box<Formula>),
    AU(Box<Formula>, Box<Formula>)
}

impl FromJson<CompareOp> for CompareOp {
    fn from_json(json: &Json) -> Result<CompareOp, DecoderError> {
        if let Some(str) = json.as_string() {
            match str {
                "LT" => Ok(CompareOp::LT),
                "GT" => Ok(CompareOp::GT),
                _ => Err(DecoderError::UnknownVariantError("Unknown comparison operator".to_string()))
            }
        } else {
            Err(DecoderError::ExpectedError("String".to_string(), json.to_string()))
        }
    }
}

impl FromJson<Proposition> for Proposition {
    fn from_json(json: &Json) -> Result<Proposition, DecoderError> {
        if let Some(str) = json.as_string() {
            match str {
                "True" => Ok(Proposition::True),
                "False" => Ok(Proposition::False),
                _ => Err(DecoderError::UnknownVariantError("Unknown proposition".to_string()))
            }
        } else {
            as_object(json, |map| {
                Ok(Proposition::Equation(
                    try![map.read_item::<u64>("variableIndex")] as usize,
                    try![map.read_item::<CompareOp>("compareOp")],
                    try![map.read_item::<u64>("thresholdIndex")] as usize
                ))
            })
        }
    }
}

impl FromJson<Formula> for Formula {
    fn from_json(json: &Json) -> Result<Formula, DecoderError> {
        as_object(json, |map| {
            let operator = try![map.read_item::<String>("operator")];
            match operator.as_ref() {
                "Atom" => Ok(Formula::Atom(try![map.read_item::<Proposition>("inner")])),
                "Not" => Ok(Formula::Not(Box::new(try![map.read_item::<Formula>("inner")]))),
                "EX" => Ok(Formula::EX(Box::new(try![map.read_item::<Formula>("inner")]))),
                "And" => Ok(Formula::And(
                    Box::new(try![map.read_item::<Formula>("left")]),
                    Box::new(try![map.read_item::<Formula>("right")]),
                )),
                "EU" => Ok(Formula::EU(
                    Box::new(try![map.read_item::<Formula>("path")]),
                    Box::new(try![map.read_item::<Formula>("reach")]),
                )),
                "AU" => Ok(Formula::AU(
                    Box::new(try![map.read_item::<Formula>("path")]),
                    Box::new(try![map.read_item::<Formula>("reach")]),
                )),
                _ => Err(DecoderError::UnknownVariantError("Unknown formula operator".to_string()))
            }
        })
    }
}
