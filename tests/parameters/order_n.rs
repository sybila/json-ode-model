extern crate ode_model;

extern crate z3;
use self::z3::*;

use ode_model::parameters::Colors;
use ode_model::parameters::order_n::*;

pub fn test_all(z3: &Context) {
    order_n_colors_emptiness();
    order_n_colors_and(z3);
    order_n_colors_or(z3);
    order_n_colors_not(z3);
}

pub fn order_n_colors_emptiness() {
    assert![OrderN::tt().is_not_empty()];
    assert![OrderN::ff().is_empty()];
}

pub fn order_n_colors_and(z3: &Context) {
    let x = z3.named_real_const("x");
    let three = z3.from_u64(3);
    let two = z3.from_u64(2);

    let gt3 = OrderN::new(x.gt(&three));
    let gt2 = OrderN::new(x.gt(&two));
    let lt2 = OrderN::new(x.lt(&two));
    let tt = OrderN::tt();

    assert![tt.and(&gt3).is_not_empty()];
    assert![tt.and(&gt3).is_not_empty()];
    assert![gt3.and(&gt3).is_not_empty()];
    assert![gt3.and(&lt2).is_empty()];
    assert![gt3.and(&gt2).is_not_empty()];
    assert![lt2.and(&gt2).is_empty()];
}

pub fn order_n_colors_or(z3: &Context) {
    let x = z3.named_real_const("x");
    let three = z3.from_u64(3);
    let two = z3.from_u64(2);

    let gt3 = OrderN::new(x.gt(&three));
    let gt2 = OrderN::new(x.gt(&two));
    let lt2 = OrderN::new(x.lt(&two));
    let tt = OrderN::tt();
    let ff = OrderN::ff();
    
    assert![gt3.or(&tt).is_not_empty()];
    assert![gt3.or(&ff).is_not_empty()];
    assert![gt3.or(&gt2).is_not_empty()];
    assert![gt3.or(&lt2).is_not_empty()];
    assert![ff.or(&ff).is_empty()];
}

pub fn order_n_colors_not(z3: &Context) {
    let x = z3.named_real_const("x");
    let three = z3.from_u64(3);
    let two = z3.from_u64(2);

    let gt3 = OrderN::new(x.gt(&three));
    let gt2 = OrderN::new(x.gt(&two));
    let lt2 = OrderN::new(x.lt(&two));
    let lt3 = OrderN::new(x.lt(&three));

    assert![gt3.and(&gt2.not()).is_empty()];
    assert![lt2.and(&lt3.not()).is_empty()];
    assert![gt2.and(&gt3.not()).is_not_empty()];
    assert![lt3.and(&lt2.not()).is_not_empty()];
}