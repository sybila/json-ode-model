extern crate ode_model;

use std::fmt::Debug;
use std::collections::HashMap;

use ode_model::model::Model;
use ode_model::model::Summand2;
use ode_model::model::evaluable::Evaluable2;
use ode_model::model::evaluable::Function;
use ode_model::model::StateID;

use ode_model::generator::compute_directed_edges;

use ode_model::parameters::Colors;
use ode_model::parameters::order_0::Order0;
use ode_model::parameters::order_1::Order1;
use ode_model::parameters::order_n::OrderN;

static STATE_0: StateID = 0;
static STATE_1: StateID = 1;
static STATE_2: StateID = 2;

//OrderN methods are called from external module to ensure global Z3 is happy...
pub fn generator_simple_order_n() {
    generator_simple_case_0::<OrderN>();
    generator_simple_case_1::<OrderN>();
    generator_simple_case_2::<OrderN>();
    generator_simple_case_3::<OrderN>();
    generator_simple_case_4::<OrderN>();
    generator_simple_case_5::<OrderN>();
    generator_simple_case_6::<OrderN>();
    generator_simple_case_7::<OrderN>();
    generator_simple_case_8::<OrderN>();
    generator_simple_case_9::<OrderN>();
    generator_simple_case_10::<OrderN>();
    generator_simple_case_11::<OrderN>();
    generator_simple_case_12::<OrderN>();
    generator_simple_case_13::<OrderN>();
    generator_simple_case_14::<OrderN>();
    generator_simple_case_15::<OrderN>();
    generator_simple_case_16::<OrderN>();
    generator_simple_case_17::<OrderN>();
    generator_simple_case_18::<OrderN>();
    generator_simple_case_19::<OrderN>();
    generator_simple_case_20::<OrderN>();
    generator_simple_case_21::<OrderN>();
    generator_simple_case_22::<OrderN>();
    generator_simple_case_23::<OrderN>();
    generator_simple_case_24::<OrderN>();
    generator_simple_case_25::<OrderN>();
    generator_simple_case_26::<OrderN>();
    generator_simple_case_27::<OrderN>();
    generator_simple_case_28::<OrderN>();
    generator_simple_case_29::<OrderN>();
    generator_simple_case_30::<OrderN>();
    generator_simple_case_31::<OrderN>();
    generator_simple_case_32::<OrderN>();
    generator_simple_case_33::<OrderN>();
    generator_simple_case_34::<OrderN>();
    generator_simple_case_35::<OrderN>();
    generator_simple_case_36::<OrderN>();
    generator_simple_case_37::<OrderN>();
    generator_simple_case_38::<OrderN>();
    generator_simple_case_39::<OrderN>();
    generator_simple_case_40::<OrderN>();
    generator_simple_case_41::<OrderN>();
    generator_simple_case_42::<OrderN>();
    generator_simple_case_43::<OrderN>();
}

#[test]
fn generator_simple_order_0() {
    generator_simple_case_0::<Order0>();
    generator_simple_case_1::<Order0>();
    generator_simple_case_2::<Order0>();
    generator_simple_case_3::<Order0>();
    generator_simple_case_4::<Order0>();
    generator_simple_case_5::<Order0>();
    generator_simple_case_6::<Order0>();
    generator_simple_case_7::<Order0>();
    generator_simple_case_8::<Order0>();
    generator_simple_case_9::<Order0>();
    generator_simple_case_10::<Order0>();
    generator_simple_case_11::<Order0>();
    generator_simple_case_12::<Order0>();
    generator_simple_case_13::<Order0>();
    generator_simple_case_14::<Order0>();
    generator_simple_case_15::<Order0>();
    generator_simple_case_16::<Order0>();
    generator_simple_case_17::<Order0>();
    generator_simple_case_18::<Order0>();
    generator_simple_case_19::<Order0>();
    generator_simple_case_20::<Order0>();
    generator_simple_case_21::<Order0>();
    generator_simple_case_22::<Order0>();
    generator_simple_case_23::<Order0>();
    generator_simple_case_24::<Order0>();
    generator_simple_case_25::<Order0>();
    generator_simple_case_26::<Order0>();
    generator_simple_case_27::<Order0>();
    generator_simple_case_28::<Order0>();
    generator_simple_case_29::<Order0>();
    generator_simple_case_30::<Order0>();
    generator_simple_case_31::<Order0>();
    generator_simple_case_32::<Order0>();
    generator_simple_case_33::<Order0>();
    generator_simple_case_34::<Order0>();
    generator_simple_case_35::<Order0>();
    generator_simple_case_36::<Order0>();
    generator_simple_case_37::<Order0>();
    generator_simple_case_38::<Order0>();
    generator_simple_case_39::<Order0>();
    generator_simple_case_40::<Order0>();
    generator_simple_case_41::<Order0>();
    generator_simple_case_42::<Order0>();
    generator_simple_case_43::<Order0>();
}

#[test]
fn generator_simple_order_1() {
    generator_simple_case_0::<Order1>();
    generator_simple_case_1::<Order1>();
    generator_simple_case_2::<Order1>();
    generator_simple_case_3::<Order1>();
    generator_simple_case_4::<Order1>();
    generator_simple_case_5::<Order1>();
    generator_simple_case_6::<Order1>();
    generator_simple_case_7::<Order1>();
    generator_simple_case_8::<Order1>();
    generator_simple_case_9::<Order1>();
    generator_simple_case_10::<Order1>();
    generator_simple_case_11::<Order1>();
    generator_simple_case_12::<Order1>();
    generator_simple_case_13::<Order1>();
    generator_simple_case_14::<Order1>();
    generator_simple_case_15::<Order1>();
    generator_simple_case_16::<Order1>();
    generator_simple_case_17::<Order1>();
    generator_simple_case_18::<Order1>();
    generator_simple_case_19::<Order1>();
    generator_simple_case_20::<Order1>();
    generator_simple_case_21::<Order1>();
    generator_simple_case_22::<Order1>();
    generator_simple_case_23::<Order1>();
    generator_simple_case_24::<Order1>();
    generator_simple_case_25::<Order1>();
    generator_simple_case_26::<Order1>();
    generator_simple_case_27::<Order1>();
    generator_simple_case_28::<Order1>();
    generator_simple_case_29::<Order1>();
    generator_simple_case_30::<Order1>();
    generator_simple_case_31::<Order1>();
    generator_simple_case_32::<Order1>();
    generator_simple_case_33::<Order1>();
    generator_simple_case_34::<Order1>();
    generator_simple_case_35::<Order1>();
    generator_simple_case_36::<Order1>();
    generator_simple_case_37::<Order1>();
    generator_simple_case_38::<Order1>();
    generator_simple_case_39::<Order1>();
    generator_simple_case_40::<Order1>();
    generator_simple_case_41::<Order1>();
    generator_simple_case_42::<Order1>();
    generator_simple_case_43::<Order1>();
}

fn create_model(a: f64, b: f64, c: f64, d: f64) -> Model {
    let eval = Evaluable2(0, Function::Explicit {
        thresholds: vec![0.0, 1.0, 2.0, 3.0],
        values: vec![a, b, c, d]
    });
    Model::new(
        vec![], vec![vec![0.0, 1.0, 2.0, 3.0]],
        vec![vec![Summand2 {
            multiplier: 1.0,
            variable_indices: vec![],
            parameter_indices: vec![],
            functions: vec![eval]
        }]]
    )
}

fn generator_simple_test<C: Colors + Debug>(
    a: f64, b: f64, c: f64, d: f64,
    s1: Vec<StateID>, s2: Vec<StateID>, s3: Vec<StateID>,
    p1: Vec<StateID>, p2: Vec<StateID>, p3: Vec<StateID>
) {
    let model = create_model(a, b, c, d);

    let mut s_0 = HashMap::new();
    for s in s1 { s_0.insert(s, C::tt()); }
    let mut s_1 = HashMap::new();
    for s in s2 { s_1.insert(s, C::tt()); }
    let mut s_2 = HashMap::new();
    for s in s3 { s_2.insert(s, C::tt()); }

    let mut p_0 = HashMap::new();
    for s in p1 { p_0.insert(s, C::tt()); }
    let mut p_1 = HashMap::new();
    for s in p2 { p_1.insert(s, C::tt()); }
    let mut p_2 = HashMap::new();
    for s in p3 { p_2.insert(s, C::tt()); }

    assert_eq![s_0, compute_directed_edges(&model, &STATE_0, &true)];
    assert_eq![s_1, compute_directed_edges(&model, &STATE_1, &true)];
    assert_eq![s_2, compute_directed_edges(&model, &STATE_2, &true)];
    assert_eq![p_0, compute_directed_edges(&model, &STATE_0, &false)];
    assert_eq![p_1, compute_directed_edges(&model, &STATE_1, &false)];
    assert_eq![p_2, compute_directed_edges(&model, &STATE_2, &false)];
}

//0..0..0..0
fn generator_simple_case_0<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, 0.0, 0.0, 0.0,
        vec![0], vec![1], vec![2],
        vec![0], vec![1], vec![2]
    )
}

//+..0..0..0
fn generator_simple_case_1<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, 0.0, 0.0,
        vec![0], vec![1], vec![2],
        vec![0], vec![1], vec![2]
    )
}

//0..+..0..0
fn generator_simple_case_2<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, 1.0, 0.0, 0.0,
        vec![0,1], vec![1], vec![2],
        vec![0], vec![0,1], vec![2]
    )
}

//-..0..0..0
fn generator_simple_case_3<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 0.0, 0.0, 0.0,
        vec![0], vec![1], vec![2],
        vec![0], vec![1], vec![2]
    )
}

//0..-..0..0
fn generator_simple_case_4<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, -1.0, 0.0, 0.0,
        vec![0], vec![0,1], vec![2],
        vec![0,1], vec![1], vec![2]
    )
}

//+..+..0..0
fn generator_simple_case_5<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 1.0, 0.0, 0.0,
        vec![1], vec![1], vec![2],
        vec![], vec![0,1], vec![2]
    )
}

//+..0..+..0
fn generator_simple_case_6<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, 1.0, 0.0,
        vec![0], vec![1,2], vec![2],
        vec![0], vec![1], vec![1,2]
    )
}

//+..0..0..+
fn generator_simple_case_7<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, 0.0, 1.0,
        vec![0], vec![1], vec![2],
        vec![0], vec![1], vec![2]
    )
}

//0..+..+..0
fn generator_simple_case_8<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, 1.0, 1.0, 0.0,
        vec![0,1], vec![2], vec![2],
        vec![0], vec![0], vec![1,2]
    )
}

//-..-..0..0
fn generator_simple_case_9<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, -1.0, 0.0, 0.0,
        vec![0], vec![0,1], vec![2],
        vec![0,1], vec![1], vec![2]
    )
}

//-..0..-..0
fn generator_simple_case_10<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 0.0, -1.0, 0.0,
        vec![0], vec![1], vec![1,2],
        vec![0], vec![1,2], vec![2]
    )
}

//-..0..0..-
fn generator_simple_case_11<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 0.0, 0.0, -1.0,
        vec![0], vec![1], vec![2],
        vec![0], vec![1], vec![2]
    )
}

//0..-..-..0
fn generator_simple_case_12<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, -1.0, -1.0, 0.0,
        vec![0], vec![0], vec![1,2],
        vec![0,1], vec![2], vec![2]
    )
}

//0..+..+..+
fn generator_simple_case_13<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, 1.0, 1.0, 1.0,
        vec![0,1], vec![2], vec![2],
        vec![0], vec![0], vec![1,2]
    )
}

//+..0..+..+
fn generator_simple_case_14<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, 1.0, 1.0,
        vec![0], vec![1,2], vec![2],
        vec![0], vec![1], vec![1,2]
    )
}

//0..-..-..-
fn generator_simple_case_15<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, -1.0, -1.0, -1.0,
        vec![0], vec![0], vec![1],
        vec![0,1], vec![2], vec![]
    )
}

//-..0..-..-
fn generator_simple_case_16<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 0.0, -1.0, -1.0,
        vec![0], vec![1], vec![1],
        vec![0], vec![1,2], vec![]
    )
}

//+..+..+..+
fn generator_simple_case_17<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 1.0, 1.0, 1.0,
        vec![1], vec![2], vec![2],
        vec![], vec![0], vec![1,2]
    )
}

//-..-..-..-
fn generator_simple_case_18<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, -1.0, -1.0, -1.0,
        vec![0], vec![0], vec![1],
        vec![0,1], vec![2], vec![]
    )
}

//+..-..0..0
fn generator_simple_case_19<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, 0.0, 0.0,
        vec![0], vec![0,1], vec![2],
        vec![0,1], vec![1], vec![2]
    )
}

//+..0..-..0
fn generator_simple_case_20<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, -1.0, 0.0,
        vec![0], vec![1], vec![1,2],
        vec![0], vec![1,2], vec![2]
    )
}

//+..0..0..-
fn generator_simple_case_21<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, 0.0, -1.0,
        vec![0], vec![1], vec![2],
        vec![0], vec![1], vec![2]
    )
}

//-..+..0..0
fn generator_simple_case_22<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, 0.0, 0.0,
        vec![0,1], vec![1], vec![2],
        vec![0], vec![0,1], vec![2]
    )
}

//-..0..+..0
fn generator_simple_case_23<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 0.0, 1.0, 0.0,
        vec![0], vec![1,2], vec![2],
        vec![0], vec![1], vec![1,2]
    )
}

//0..+..-..0
fn generator_simple_case_24<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, 1.0, -1.0, 0.0,
        vec![0,1], vec![1], vec![1,2],
        vec![0], vec![0,1,2], vec![2]
    )
}

//0..-..+..0
fn generator_simple_case_25<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, -1.0, 1.0, 0.0,
        vec![0], vec![0,1,2], vec![2],
        vec![0,1], vec![1], vec![1,2]
    )
}

//+..0..-..-
fn generator_simple_case_26<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 0.0, -1.0, -1.0,
        vec![0], vec![1], vec![1],
        vec![0], vec![1,2], vec![]
    )
}

//+..-..0..-
fn generator_simple_case_27<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, 0.0, -1.0,
        vec![0], vec![0,1], vec![2],
        vec![0,1], vec![1], vec![2]
    )
}

//+..-..-..0
fn generator_simple_case_28<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, -1.0, 0.0,
        vec![0], vec![0], vec![1,2],
        vec![0,1], vec![2], vec![2]
    )
}

//0..+..-..-
fn generator_simple_case_29<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, 1.0, -1.0, -1.0,
        vec![0,1], vec![1], vec![1],
        vec![0], vec![0,1,2], vec![]
    )
}

//-..+..0..-
fn generator_simple_case_30<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, 0.0, -1.0,
        vec![0,1], vec![1], vec![2],
        vec![0], vec![0,1], vec![2]
    )
}

//-..+..-..0
fn generator_simple_case_31<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, -1.0, 0.0,
        vec![0,1], vec![1], vec![2,1],
        vec![0], vec![0,1,2], vec![2]
    )
}

//-..0..+..+
fn generator_simple_case_32<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 0.0, 1.0, 1.0,
        vec![0], vec![1,2], vec![2],
        vec![0], vec![1], vec![1,2]
    )
}

//-..+..0..+
fn generator_simple_case_33<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, 0.0, 1.0,
        vec![0,1], vec![1], vec![2],
        vec![0], vec![0,1], vec![2]
    )
}

//-..+..+..0
fn generator_simple_case_34<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, 1.0, 0.0,
        vec![0,1], vec![2], vec![2],
        vec![0], vec![0], vec![1,2]
    )
}

//0..-..+..+
fn generator_simple_case_35<C: Colors + Debug>() {
    generator_simple_test::<C>(
        0.0, -1.0, 1.0, 1.0,
        vec![0], vec![0,1,2], vec![2],
        vec![0,1], vec![1], vec![1,2]
    )
}

//+..-..0..+
fn generator_simple_case_36<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, 0.0, 1.0,
        vec![0], vec![0,1], vec![2],
        vec![0,1], vec![1], vec![2]
    )
}

//+..-..+..0
fn generator_simple_case_37<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, 1.0, 0.0,
        vec![0], vec![0,1,2], vec![2],
        vec![0,1], vec![1], vec![1,2]
    )
}

//+..+..-..-
fn generator_simple_case_38<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, 1.0, -1.0, -1.0,
        vec![1], vec![1], vec![1],
        vec![], vec![0,1,2], vec![]
    )
}

//+..-..+..-
fn generator_simple_case_39<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, 1.0, -1.0,
        vec![0], vec![0,1,2], vec![2],
        vec![0,1], vec![1], vec![1,2]
    )
}

//-..+..+..+
fn generator_simple_case_40<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, 1.0, 1.0,
        vec![0,1], vec![2], vec![2],
        vec![0], vec![0], vec![1,2]
    )
}

//+..-..+..+
fn generator_simple_case_41<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, 1.0, 1.0,
        vec![0], vec![0,1,2], vec![2],
        vec![0,1], vec![1], vec![1,2]
    )
}

//+..-..-..-
fn generator_simple_case_42<C: Colors + Debug>() {
    generator_simple_test::<C>(
        1.0, -1.0, -1.0, -1.0,
        vec![0], vec![0], vec![1],
        vec![0, 1], vec![2], vec![]
    )
}

//-..+..-..-
fn generator_simple_case_43<C: Colors + Debug>() {
    generator_simple_test::<C>(
        -1.0, 1.0, -1.0, -1.0,
        vec![0,1], vec![1], vec![1],
        vec![0], vec![0,1,2], vec![]
    )
}