#![allow(unused_imports)]

// use poc050 token_cfg, Sender, Token [

use crate::poc050::{token_cfg, Sender, Token};

// use poc050 token_cfg, Sender, Token ]

use casper_types::{
    U256,
};

// tests_poc050_{} [
// deploy [

#[test]
fn test_poc050_deploy() {
    let t = Token::deployed();
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.futjin), token_cfg::total_supply());
    assert_eq!(t.balance_of(t.bro), 0.into());
    assert_eq!(t.allowance(t.futjin, t.futjin), 0.into());
    assert_eq!(t.allowance(t.futjin, t.bro), 0.into());
    assert_eq!(t.allowance(t.bro, t.futjin), 0.into());
    assert_eq!(t.allowance(t.bro, t.bro), 0.into());
}

// deploy ]
// transfer [

#[test]
fn test_poc050_transfer() {
    let amount = 10.into();
    let mut t = Token::deployed();
    t.transfer(t.bro, amount, Sender(t.futjin));
    assert_eq!(t.balance_of(t.futjin), token_cfg::total_supply() - amount);
    assert_eq!(t.balance_of(t.bro), amount);
}

// transfer ]
// transfer_too_much! [

#[test]
#[should_panic]
fn test_poc050_transfer_too_much() {
    let amount = 1.into();
    let mut t = Token::deployed();
    t.transfer(t.futjin, amount, Sender(t.bro));
}

// transfer_too_much! ]
// approve [

#[test]
fn test_poc050_approve() {
    let amount = 10.into();
    let mut t = Token::deployed();
    t.approve(t.bro, amount, Sender(t.futjin));
    assert_eq!(t.balance_of(t.futjin), token_cfg::total_supply());
    assert_eq!(t.balance_of(t.bro), 0.into());
    assert_eq!(t.allowance(t.futjin, t.bro), amount);
    assert_eq!(t.allowance(t.bro, t.futjin), 0.into());
}

// approve ]
// transfer_from [

#[test]
fn test_poc050_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed();
    t.approve(t.bro, allowance, Sender(t.futjin));
    t.transfer_from(t.futjin, t.dax, amount, Sender(t.bro));
    assert_eq!(t.balance_of(t.futjin), token_cfg::total_supply() - amount);
    assert_eq!(t.balance_of(t.bro), 0.into());
    assert_eq!(t.balance_of(t.dax), amount);
    assert_eq!(t.allowance(t.futjin, t.bro), allowance - amount);
}

// transfer_from ]
// transfer_from_too_much! [

#[test]
#[should_panic]
fn test_poc050_transfer_from_too_much() {
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed();
    t.transfer_from(t.futjin, t.dax, amount, Sender(t.bro));
}

// transfer_from_too_much! ]
// tests_poc050_{} ]
