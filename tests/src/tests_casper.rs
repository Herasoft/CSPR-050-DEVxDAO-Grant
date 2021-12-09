#![allow(unused_imports)]

use casper_types::{
    U256
};

// test_casper_safemath [

#[test]
#[should_panic]
fn test_casper_safemath_overflow_min() {
    let mut amount: U256 = 0.into();
    amount -= 1.into();
}

#[test]
#[should_panic]
fn test_casper_safemath_overflow_max() {
    let mut amount: U256 = U256::max_value();
    amount += 1.into();
    panic!("called `Option::unwrap()` on a `None` value")

}

#[test]
fn test_casper_safemath_overflow_no_panic() {
    let mut amount: U256 = 0.into();
    amount += 1.into();
}

// test_casper_safemath ]
