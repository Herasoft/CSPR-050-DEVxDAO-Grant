#![allow(unused_imports)]

use crate::poc050::{Sender, Token};

use casper_types::{
    U256,
};

// incrase_allowance [

#[test]
fn test_poc050_incrace_allowance() {
    let allowance = 500.into();
    let amount: U256 = 5.into();
    let mut t = Token::deployed();
    t.increase_allowance(t.futjin, allowance, Sender(t.futjin));
    assert_eq!(t.allowance(t.futjin, t.bro), allowance - amount);
}

// incrase_allowance ]
// decrease_allowance [

#[test]
fn test_poc050_decrase_allowance() {
    let allowance = 500.into();
    let amount: U256 = 5.into();
    let mut t = Token::deployed();
    t.decrease_allowance(t.futjin, allowance, Sender(t.futjin));
    assert_eq!(t.allowance(t.futjin, t.bro), allowance - amount);
}

// decrease_allowance ]
// mint [

#[test]
fn test_poc050_mint() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();

    t.mint(t.bro, amount, Sender(t.futjin));
    assert_eq!(t.balance_of(t.bro), amount);

}

// mint ]
// burn [

#[test]
fn test_poc050_burn() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();

    t.mint(t.bro, amount, Sender(t.futjin));
    t.burn(t.bro, amount, Sender(t.futjin));

    assert_eq!(t.balance_of(t.bro), amount);
}

// burn ]
// burn_from [

#[test]
fn test_poc050_burn_from() {
    let mut t = Token::deployed();
    let amount = 5_000_000.into();

    t.mint(t.bro, amount, Sender(t.futjin));
    t.burn_from(t.bro, amount, Sender(t.futjin));

    assert_eq!(t.allowance(t.bro, t.futjin), amount);
}

// burn ]
