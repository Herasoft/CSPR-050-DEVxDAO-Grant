#![allow(unused_imports)]


use crate::poc050::{Sender, Token};

use casper_types::{
    U256,
};

// mint [

#[test]
fn test_poc050_mint() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();
    t.mint(t.bro, amount, Sender(t.futjin));
    assert_eq!(t.balance_of(t.bro), amount);

}

#[test]
#[should_panic]
fn test_poc050_mint_bad_owner_fail() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();
    t.mint(t.futjin, amount, Sender(t.bro));
}

// mint ]
// burn [

#[test]
fn test_poc050_burn() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();
    let half = amount / 2;

    t.mint(t.bro, amount, Sender(t.futjin));
    t.burn(t.bro, amount, Sender(t.futjin));
    assert_eq!(t.balance_of(t.bro), 0.into());

    t.mint(t.bro, amount, Sender(t.futjin));
    t.burn(t.bro, half, Sender(t.futjin));
    assert_eq!(t.balance_of(t.bro), half);
    t.burn(t.bro, half, Sender(t.futjin));
    assert_eq!(t.balance_of(t.bro), 0.into());
}

#[test]
#[should_panic]
fn test_poc050_burn_bad_owner_fail() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();
    t.mint(t.bro, amount, Sender(t.futjin));
    t.burn(t.bro, amount, Sender(t.bro));
}

// burn ]
// burn_from [

#[test]
fn test_poc050_burn_from() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();

    t.mint(t.bro, amount, Sender(t.futjin));

    t.increase_allowance(t.futjin, 1.into(), Sender(t.bro));
    t.burn_from(t.bro, 1.into(), Sender(t.futjin));

    assert_eq!(t.balance_of(t.bro), 4_999_999.into());
}

#[test]
#[should_panic]
fn test_poc050_burn_from_fail() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();

    t.mint(t.bro, amount, Sender(t.futjin));

    t.increase_allowance(t.futjin, 1.into(), Sender(t.bro));
    t.burn_from(t.bro, 2.into(), Sender(t.futjin));
}

// burn ]
