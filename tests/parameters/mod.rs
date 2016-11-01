extern crate ode_model;
extern crate z3;
use self::z3::*;

mod order_1;
mod order_n;

use ode_model::parameters::Colors;
use ode_model::parameters::order_0::Order0;
use ode_model::parameters::order_1::Order1;
use ode_model::parameters::order_n::OrderN;
use ode_model::parameters::order_n::Z3;

struct SimpleColorTest;

/// Simple color test that requires only two distinct values, tt and ff.
impl SimpleColorTest {

    fn test_all<C>() where C: Colors {
        SimpleColorTest::test_and::<C>();
        SimpleColorTest::test_or::<C>();
        SimpleColorTest::test_not::<C>();
        SimpleColorTest::test_eq::<C>();
        SimpleColorTest::test_optimize::<C>();
        SimpleColorTest::test_complex::<C>();
    }

    fn test_and<C>() where C: Colors {
        let tt = C::tt();
        let ff = C::ff();
        assert![tt.and(&tt).is_not_empty()];
        assert![tt.and(&ff).is_empty()];
        assert![ff.and(&tt).is_empty()];
        assert![ff.and(&ff).is_empty()];
    }

    fn test_or<C>() where C: Colors {
        let tt = C::tt();
        let ff = C::ff();
        assert![tt.or(&tt).is_not_empty()];
        assert![tt.or(&ff).is_not_empty()];
        assert![ff.or(&tt).is_not_empty()];
        assert![ff.or(&ff).is_empty()];
    }

    fn test_not<C>() where C: Colors {
        let tt = C::tt();
        let ff = C::ff();
        assert![tt.not().is_empty()];
        assert![ff.not().is_not_empty()];
        assert![ff.not().not().is_empty()];
        assert![tt.not().not().is_not_empty()];
    }

    fn test_eq<C>() where C: Colors {
        assert![C::tt() == C::tt()];
        assert![C::tt() != C::ff()];
        assert![C::ff() != C::tt()];
        assert![C::ff() == C::ff()];
    }

    fn test_optimize<C>() where C: Colors {
        assert![C::tt() == C::tt().optimize()];
        assert![C::ff() == C::ff().optimize()];
    }

    fn test_complex<C>() where C: Colors {
        let tt = C::tt();
        let ff = C::ff();
        // !((1 & 0) | 1)
        assert![tt.and(&ff).or(&tt).not().is_empty()];
    }

}

#[test]
fn order_0_basic() {
    SimpleColorTest::test_all::<Order0>();
}

#[test]
fn order_1_basic() {
    SimpleColorTest::test_all::<Order1>();
}

#[test]
fn order_n_full() {
    let z3 = Context::new(&Config::new());
    unsafe {
        Z3 = &z3 as *const Context;
    }
    SimpleColorTest::test_all::<OrderN>();
    self::order_n::test_all(&z3);
    super::generator::generator_simple_order_n();

    //z3 is cleaned up here
}