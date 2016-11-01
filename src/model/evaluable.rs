use self::Function::*;
use super::VariableIndex;

#[derive(Clone)]
pub struct Evaluable2(pub VariableIndex, pub Function);

#[derive(Clone)]
pub enum Function {
    Step { a: f64, b: f64, theta: f64 },
    Ramp { a: f64, b: f64, low: f64, high: f64 },
    Approximation { thresholds: Vec<f64>, values: Vec<f64> },
    Explicit { thresholds: Vec<f64>, values: Vec<f64> }
}

impl Function {
    pub fn eval(&self, x: &f64) -> f64 {
        match self {
            &Step { a, b, theta } => if *x < theta { a } else { b },
            &Ramp { a, b, low, high } => if *x <= low { a } else if *x >= high { b } else {
                a + (*x - low) / (high - low) * (b - a)
            },
            &Approximation { ref thresholds, ref values } => {
                if x <= thresholds.first().unwrap() {
                    *values.first().unwrap()
                } else if x >= thresholds.last().unwrap() {
                    *values.last().unwrap()
                } else {
                    match thresholds.binary_search_by(
                        |probe| probe.partial_cmp(x).expect("Floating point error")
                    ) {
                        Ok(index) => values[index],
                        Err(i_high) => {
                            let i_low = i_high - 1;
                            Ramp {
                                a: values[i_low], b: values[i_high],
                                low: thresholds[i_low], high: thresholds[i_high]
                            }.eval(x)
                        }
                    }
                }
            }
            &Explicit { ref thresholds, ref values } => {
                if let Some(index) = thresholds.iter().position(|a| a == x) {
                    values[index]
                } else {
                    panic!["Invalid threshold for explicit evaluable"]
                }
            }
        }
    }
}