use super::super::model::Vertex;
use super::super::model::evaluable::Evaluable2;
use super::super::model::VariableIndex;
use super::super::model::Model;
use super::super::types::Interval;
use super::Colors;
use std::cmp::Ordering;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

use rustc_serialize::json::{ToJson, Json, DecoderError};
use json_utils::{FromJson, as_object, JsonMap, create_object};

/// A variant of Colors that is represented by a formula of inequality polynomials of order 1.
/// That is: x < k (k \in R) | phi1 and phi2 | phi1 or phi2.
/// Each inequality can be represented as a pair (k, </>).
/// The whole formula is stored in form of a DNF where negations are always directly
/// applied to the inequalities (by flipping the sign).
/// Because each polynomial contains only one variable, we can further compress the representation
/// of each DNF clause. Each clause can contain only up to 2 literals with each variable
/// (all other are redundant). These <=2 literals can then by compressed into an interval
/// (a, b) where a,b \in R + {-inf, inf}. Assuming a variable ordering, a list of intervals
/// fully describes a DNF clause:
/// (x > 1 & x < 2 & y < 1 & y > 0) = ((1,2),(0,1))
/// (x > 1 & y < 1 & y < 2) = ((1,inf),(-inf,1))
///
/// The whole formula is then represented as a list of these compressed DNF clauses.
///
/// Finally, empty list can be used as a tautological clause
/// (it effectively represents a clause filled with (-inf, inf) intervals)
/// This is mainly to ensure a model-independent representation.
///
/// In accordance with general rules for & and | operators,
/// () represents a contradictory formula and (()) represents a tautology.
///
#[derive(Debug, Clone, Hash)]
pub struct Order1(pub Vec<Clause>);

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Clause(pub Vec<Interval>);

impl Colors for Order1 {
    fn tt() -> Self { Order1(vec![Clause(vec![])]) }

    fn ff() -> Self { Order1(vec![]) }


    fn divide(model: &Model, variable_index: &VariableIndex, vertex: &Vertex) -> (Self, Self) {
        let ref equation = model.equations[*variable_index];
        let ref thresholds = model.variables[*variable_index];
        let mut parameter = None;
        let mut derivation = 0.0;
        let mut denominator = 0.0;
        for summand in equation {
            let mut result = summand.multiplier;
            for var in &summand.variable_indices {
                result *= model.variables[*var][vertex[*var]];
            }
            for &Evaluable2(ref var, ref f) in &summand.functions {
                result *= f.eval(&model.variables[*var][vertex[*var]]);
            }
            if let Some(index) = summand.parameter_indices.first() {
                if parameter.is_some() && parameter == Some(index) {
                    panic!["More than one parameter per equation!"]
                }
                denominator += result;
                parameter = Some(index);
            } else {
                derivation += result;
            }
        }
        //println!["Vertex: {:?} parameter: {:?} derivation: {:?} denominator: {:?}", vertex, parameter, derivation, denominator];
        // 0 > a + b * p
        // -a/b > p | b > 0
        // -a/b < p | b < 0
        if let Some(parameter) = parameter {
            let split = -derivation / denominator;
            let mut lower = vec![Interval::one(); model.parameter_bounds.len()];
            let mut upper = vec![Interval::one(); model.parameter_bounds.len()];
            if denominator > 0.0 {
                lower[*parameter] = Interval(NEG_INFINITY, split);
                upper[*parameter] = Interval(split, INFINITY);
            } else {
                // when denominator is negative, the condition is reversed because the
                // function is decreasing
                lower[*parameter] = Interval(split, INFINITY);
                upper[*parameter] = Interval(NEG_INFINITY, split);
            }
            (Order1(vec![Clause(lower)]), Order1(vec![Clause(upper)]))
        } else if derivation > 0.0 {
            (Self::ff(), Self::tt())
        } else if derivation < 0.0 {
            (Self::tt(), Self::ff())
        } else {
            (Self::ff(), Self::ff())
        }
    }

    /*
    (A | B) | (C | D) = (A | B | C | D)
    The only problem is that the result might not be minimal.
    */
    fn or(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for c2 in &other.0 {
            result.safe_push_clause(c2.clone());
        }
        result
    }

    /*
    (A | B) & (C | D) = ((A&C) | (B&C) | (A&D) | (B&D))
    A&C as formulas is trivial. 
    In compressed form, it is performed as intersection 
    of intervals. 
    */
    fn and(&self, other: &Self) -> Self {
        let mut result = Order1(vec![]);
        //TODO: Don't allocate/drop a new clause when not necessary, use a "working" buffer.
        for c1 in &self.0 {
            for c2 in &other.0 {
                //Push A&C if it is valid and adds something new.
                if let Some(clause) = Clause::and(c1, c2) {
                    result.safe_push_clause(clause);
                }
            }
        }
        result
    }

    /*
     (A | B) & !(C | D) = (A | B) & (!C & !D)
     = ((A&!C&!D) | (B&!C&!D)
     (1,2) ->
     (x < 5 & x > 0) & !(x < 3 & x > 1) =
     (x < 5 & x > 0) & (x >= 3 | x <= 1) =
     (x < 5 & x > 0 & x >= 3) | (x < 5 & x > 0 & x <= 1) =
     (x < 5 & x > 3) | (x < 1 & x > 0)
     ((3, 5)) | ((0,1))
     (A & B) & !((C & D) | (E & F)) =
     (A & B) & (!(C & D) & !(E & F)) =
    */
    fn not(&self) -> Self {
        if self.is_empty() {
            Self::tt()
        } else {
            self.0.iter()
                .map(|c| Order1(Clause::not(c)))
                .fold(Self::tt(), |acc, c| acc.and(&c))
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///Even when using safe_push_clause, some incomparable new clauses might create an
    ///overlap which needs to be merged. Example: (1,3)(2,4)
    ///This procedure will try to merge these overlapping clauses, but is O(n^2) in size of formula.
    fn optimize(&self) -> Self {
        let mut result = Order1(vec![]);
        for i in &self.0 {
            //either i can be merged with something already in result and push that,
            //or else push it directly.
            let push = result.0.iter().fold(None, |acc, r| {
                acc.or_else(|| Clause::or(i, r))
            }).unwrap_or_else(|| i.clone());
            result.safe_push_clause(push);
        }
        result
    }

    fn model_bounds(model: &Model) -> Self {
        model.full_order_1_colors()
    }
}

impl PartialEq for Order1 {
    // A & !B
    fn eq(&self, other: &Order1) -> bool {
        if self.is_empty() {
            other.is_empty()
        } else {
            self.and(&other.not()).is_empty()
        }
    }
}

impl Eq for Order1 {}

impl Order1 {
    ///If given clause is strictly smaller than something in the formula, skip it.
    ///If it is strictly bigger, replace all such clauses with this one.
    ///If it is incomparable, append it to the formula.
    fn safe_push_clause(&mut self, add: Clause) {
        let cmp: Vec<Option<Ordering>> = self.0.iter().map(|c| c.partial_cmp(&add)).collect();
        if cmp.iter().any(|i| i == &Some(Ordering::Greater) || i == &Some(Ordering::Equal)) {
                                                //already included or even surpassed, do nothing
        } else if cmp.iter().all(|i| i.is_none()) {            
            self.0.push(add);                   //incomparable to everything, must add
        } else {
            //cmp is a mix of None and Less 
            self.0.retain(|c| !(c < &add));     //remove all Less
            self.0.push(add);                   //then add
        }
    }
}

//Ordering of clauses based on strict sub/super-set properties.
impl PartialOrd<Clause> for Clause {    
    fn partial_cmp(&self, other: &Clause) -> Option<Ordering> {        
        if self.is_true() && other.is_true() {
            Some(Ordering::Equal)
        } else if self.is_true() {
            Some(Ordering::Greater)
        } else if other.is_true() {
            Some(Ordering::Less) 
        } else {
            self.0.iter().zip(other.0.iter()).fold(Some(Ordering::Equal), |order, (l, r)| {
                order.and_then(|ord| {
                    if let Some(interval_ord) = l.partial_cmp(r) {
                        if interval_ord == ord || interval_ord == Ordering::Equal {
                            Some(ord)           //Nothing changed
                        } else if ord == Ordering::Equal {                            
                            Some(interval_ord)  //Promote equal to less/greater
                        } else { None }
                    } else { None }                     
                })
            })
        }
    }
}

impl Clause {

    ///Create a conjunction of two clauses if it is satisfiable, else return None.
    fn and(left: &Self, right: &Self) -> Option<Self> {
        if left.is_true() {
            Some(right.clone())
        } else if right.is_true() {
            Some(left.clone())
        } else if left.0.len() != right.0.len() {
            panic!["Clauses are from different models!"]
        } else {
            let mut result = vec![];
            for (l, r) in left.0.iter().zip(right.0.iter()) {
                if let Some(i) = l.and(r) { 
                    result.push(i) 
                } else {
                    return None;
                }
            }
            Some(Clause(result))
        }
    }

    fn or(left: &Self, right: &Self) -> Option<Self> {
        if left.is_true() {
            Some(left.clone())
        } else if right.is_true() {
            Some(right.clone())
        } else if left.0.len() != right.0.len() {
            panic!["Clauses are from different models!"]
        } else if left == right {
            Some(left.clone())
        } else {
            let mut result = vec![];
            let mut can_merge = true;
            for (l, r) in left.0.iter().zip(right.0.iter()) {
                if l != r {
                    if can_merge {
                        if let Some(merged) = l.or(r) {
                            result.push(merged);
                            can_merge = false;
                        } else { return None; }
                    } else { return None; }
                } else {
                    result.push(l.clone());
                }
            }
            Some(Clause(result))
        }
    }

    fn not(clause: &Self) -> Vec<Self> {
        if clause.is_true() {
            vec![]
        } else {
            let size = clause.0.len();
            let mut result = vec![];
            let mut cache = vec![Interval::one(); size];
            for (dim, interval) in clause.0.iter().enumerate() {
                let (lower, higher) = interval.not();
                if let Some(l) = lower {
                    cache[dim] = l;
                    result.push(Clause(cache.clone()));
                }
                if let Some(h) = higher {
                    cache[dim] = h;
                    result.push(Clause(cache.clone()));
                }
                cache[dim] = Interval::one();
            }
            result
        }
    }

    fn is_true(&self) -> bool { self.0.iter().all(|i| i == &Interval(NEG_INFINITY, INFINITY)) }
}

impl ToJson for Clause {
    fn to_json(&self) -> Json {
        self.0.to_json()
    }
}

impl ToJson for Order1 {
    fn to_json(&self) -> Json {
        create_object(|map| {
            map.write_item("type", &"rectangular".to_string());
            map.write_item("rectangles", &self.0);
        })
        //self.0.to_json()
    }
}