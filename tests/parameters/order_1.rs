extern crate ode_model;

use ode_model::parameters::Colors;
use ode_model::parameters::order_1::Order1;
use ode_model::parameters::order_1::Clause;
use ode_model::types::Interval;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

#[test]
fn order_1_colors_emptiness() {
    assert![Order1(vec![]).is_empty()];
    assert![Order1(vec![Clause(vec![])]).is_not_empty()];
}

#[test]
fn order_1_colors_and() {
    let c1 = Order1(vec![Clause(vec![Interval(0.0, 1.0)])]);
    let c2 = Order1(vec![Clause(vec![Interval(1.0, 2.5)])]);
    let c3 = Order1(vec![Clause(vec![Interval(0.0, 1.0)]), Clause(vec![Interval(2.0, 3.0)])]);

    assert_eq![c1, c1.and(&c1)];

    assert![c1.and(&c2).is_empty()];
    assert![c2.and(&c1).is_empty()];

    assert_eq![c1, c1.and(&c3)];
    assert_eq![c1, c3.and(&c1)];

    assert_eq![Order1(vec![Clause(vec![Interval(2.0, 2.5)])]), c2.and(&c3)];
    assert_eq![Order1(vec![Clause(vec![Interval(2.0, 2.5)])]), c3.and(&c2)];
}

#[test]
fn order_1_colors_or() {
    let c1 = Order1(vec![Clause(vec![Interval(0.0, 1.0)])]);
    let c2 = Order1(vec![Clause(vec![Interval(1.0, 2.5)])]);
    let c3 = Order1(vec![Clause(vec![Interval(0.0, 1.0)]), Clause(vec![Interval(2.0, 3.0)])]);

    assert_eq![c1, c1.or(&c1)];

    assert_eq![Order1(vec![Clause(vec![Interval(0.0, 2.5)])]), c1.or(&c2)];
    assert_eq![Order1(vec![Clause(vec![Interval(0.0, 2.5)])]), c2.or(&c1)];

    assert_eq![c3, c1.or(&c3)];
    assert_eq![c3, c3.or(&c1)];

    assert_eq![Order1(vec![Clause(vec![Interval(0.0, 3.0)])]), c2.or(&c3)];
    assert_eq![Order1(vec![Clause(vec![Interval(0.0, 3.0)])]), c3.or(&c2)];
}

#[test]
fn order_1_colors_not() {
    let c1 = Order1(vec![Clause(vec![Interval(0.0, 1.0)])]);
    let c2 = Order1(vec![Clause(vec![Interval(1.0, 2.5)])]);
    let c3 = Order1(vec![Clause(vec![Interval(0.0, 1.0)]), Clause(vec![Interval(2.0, 3.0)])]);

    assert_eq![c1, c1.not().not()];
    assert_eq![c2, c2.not().not()];
    assert_eq![c3, c3.not().not()];

    assert_eq![Order1(vec![
        Clause(vec![Interval(NEG_INFINITY, 0.0)]),
        Clause(vec![Interval(1.0, INFINITY)])
        ]), c1.not()];

    assert_eq![Order1(vec![
        Clause(vec![Interval(NEG_INFINITY, 1.0)]),
        Clause(vec![Interval(2.5, INFINITY)])
        ]), c2.not()];

    assert_eq![Order1(vec![
        Clause(vec![Interval(NEG_INFINITY, 0.0)]),
        Clause(vec![Interval(1.0, 2.0)]),
        Clause(vec![Interval(3.0, INFINITY)])
        ]), c3.not()];
}