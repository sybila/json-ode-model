use std::cmp::Ordering;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

use rustc_serialize::json::{ToJson, Json, DecoderError};
use json_utils::{FromJson};
use std::hash::Hash;
use std::hash::Hasher;

//Interval behaves as "atomic"

#[derive(Debug, Clone)]
pub struct Interval(pub f64, pub f64);

impl Interval {

	pub fn one() -> Interval {
		Interval(NEG_INFINITY, INFINITY)
	}

	pub fn new(min: f64, max: f64) -> Option<Interval> {
		if min < max { Some(Interval(min, max)) } else { None }
	}

	///Create intersection of two intervals, assuming result is not empty
	pub fn and(&self, right: &Interval) -> Option<Interval> {
		let lower = self.0.max(right.0);
		let upper = self.1.min(right.1);
		Interval::new(lower, upper)
	}

	///Create union of two intervals, assuming they can be merged
	pub fn or(&self, other: &Interval) -> Option<Interval> {
		//this is not the same as intersection, because here we consider >=, not >
		if self.0 > other.1 || other.0 > self.1 {
			None
		} else {
			let lower = self.0.min(other.0);
			let upper = self.1.max(other.1);
			Some(Interval(lower, upper))
		}
	}

	///Invert this interval, creating up to two new intervals, one
	///below and one above this one.
	pub fn not(&self) -> (Option<Interval>, Option<Interval>) {
		(Interval::new(NEG_INFINITY, self.0), Interval::new(self.1, INFINITY))
	}
}

impl Hash for Interval {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let precision = 1000000.0;
		state.write_i64((self.0 * precision) as i64);
		state.write_i64((self.1 * precision) as i64);
	}
}

impl ToJson for Interval {
	fn to_json(&self) -> Json {
		Json::Array(vec![Json::F64(self.0), Json::F64(self.1)])
	}
}

impl FromJson<Interval> for Interval {
	fn from_json(json: &Json) -> Result<Interval, DecoderError> {
		if let &Json::Array(ref items) = json {
			Interval::new(
				try![f64::from_json(&items[0])],
				try![f64::from_json(&items[1])]
			).ok_or(DecoderError::ApplicationError("Invalid interval".to_string()))
		} else { Err(DecoderError::ExpectedError("Array".to_string(), json.to_string()))}
	}
}

impl PartialOrd<Interval> for Interval {
	fn partial_cmp(&self, other: &Interval) -> Option<Ordering> {
		if self.0 == other.0 && self.1 == other.1 {
			Some(Ordering::Equal)
		} else if self.0 <= other.0 && self.1 >= other.1 {
			Some(Ordering::Greater)
		} else if self.0 >= other.0 && self.1 <= other.1 {
			Some(Ordering::Less)
		} else { None }
	}
}

impl PartialEq<Interval> for Interval {
	fn eq(&self, other: &Interval) -> bool {
		self.0 == other.0 && self.1 == other.1
	}
}