//! POC050 purpose is collateral backed stable coin
//! Notes: The code uses the rustdoc documentation system for the code and livecomment for the architecture
  

// 0 [

#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

// 0 ]
// use lib [

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};

use core::convert::TryInto;

// use lib ]
// use casper [

pub mod entry_points;
mod data;
pub use data::*;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256,
};

// use casper ]
// api [
// GET name, symbol, decimals, total_supply [

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn decimals() {
    let val: u8 = get_key("decimals");
    ret(val)
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("total_supply");
    ret(val)
}

// GET name, symbol, decimals, total_supply ]
// GET currency, masterMinter [

#[no_mangle]
pub extern "C" fn masterMinter() {
    let val: U256 = get_key("masterMinter");
    ret(val)
}

#[no_mangle]
pub extern "C" fn currency() {
    let val: String = get_key("currency");
    ret(val)
}

// GET currency, masterMinter ]
// GET pauser, paused [

#[no_mangle]
pub extern "C" fn pauser() {
    let val: U256 = get_key("pauser");
    ret(val)
}

#[no_mangle]
pub extern "C" fn paused() {
    let val: bool = get_key("paused");
    ret(val)
}

// GET pauser, paused ]
// balance_of (account) [

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg("account");
    let val: U256 = get_key(&balance_key(&account));
    ret(val)
}

// balance_of (account) ]
// allowance (owner, spender) [

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let spender: AccountHash = runtime::get_named_arg("spender");
    let val: U256 = get_key(&allowance_key(&owner, &spender));
    ret(val)
}

// allowance (owner, spender) ]
// approve (spender, amount) [

#[no_mangle]
pub extern "C" fn approve() {
    let spender: AccountHash = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    _approve(runtime::get_caller(), spender, amount);
}

// approve (spender, amount) ]
// transfer (recipient, amount) [

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer(runtime::get_caller(), recipient, amount);
}

// transfer (recipient, amount) ]
// transfer_from (owner, recipient, amount) [

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer_from(owner, recipient, amount);
}

// transfer_from (owner, recipient, amount) ]



// increase_allowance (address spender, uint256 addedValue) [

#[no_mangle]
pub extern "C" fn increase_allowance() {
    let spender: AccountHash = runtime::get_named_arg("spender");
    let addedValue: U256 = runtime::get_named_arg("addedValue");
    _increase_allowance(spender, addedValue);
}

// increase_allowance (address spender, uint256 addedValue) ]
// decrease_allowance (address spender, uint256 subtractedValue) [

#[no_mangle]
pub extern "C" fn decrease_allowance() {
    let spender: AccountHash = runtime::get_named_arg("spender");
    let subtractedValue: U256 = runtime::get_named_arg("subtractedValue");
    _decrease_allowance(spender, subtractedValue);
}

// decrease_allowance (address spender, uint256 subtractedValue) ]
// mint (account, value) [

#[no_mangle]
pub extern "C" fn mint() {
    let account: AccountHash = runtime::get_named_arg("account");
    let value: U256 = runtime::get_named_arg("value");
    _mint(account, value);
}

// mint (account, tokenId) ]
// burn (account, tokenId) [

#[no_mangle]
pub extern "C" fn burn() {
    let account: AccountHash = runtime::get_named_arg("account");
    let value: U256 = runtime::get_named_arg("value");
    _burn(account, value);
}

// burn (account, value) ]
// burn_from (account, tokenId) [

#[no_mangle]
pub extern "C" fn burn_from() {
    let account: AccountHash = runtime::get_named_arg("account");
    let value: U256 = runtime::get_named_arg("value");
    _burn_from(account, value);
}

// burn_from (account, value) ]



// call [

#[no_mangle]
pub extern "C" fn call() {
    // params: token_name, token_symbol, token_decimals, token_total_supply, currency [

    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let token_decimals: u8 = runtime::get_named_arg("token_decimals");
    let token_total_supply: U256 = runtime::get_named_arg("token_total_supply");
    let currency: String = runtime::get_named_arg("currency");

    // params: token_name, token_symbol, token_decimals, token_total_supply, currency ]

    let entry_points = entry_points::default();

    // props: name, symbol, decimals, total_supply, currency [

    let mut named_keys = NamedKeys::new();
    named_keys.insert("name".to_string(), storage::new_uref(token_name).into());
    named_keys.insert("symbol".to_string(), storage::new_uref(token_symbol).into());
    named_keys.insert(
        "decimals".to_string(),
        storage::new_uref(token_decimals).into(),
    );
    named_keys.insert(
        "total_supply".to_string(),
        storage::new_uref(token_total_supply).into(),
    );
    named_keys.insert("currency".to_string(), storage::new_uref(currency).into());

    // props: name, symbol, decimals, total_supply, currency ]
    // props: map<balances_{caller}> = total_supply [

    named_keys.insert(
        balance_key(&runtime::get_caller()),
        storage::new_uref(token_total_supply).into(),
    );

    // props: map<balances_{caller}> = total_supply ]
    // contract_hash = new_locked_contract (entry_points, named_keys) [

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);

    // contract_hash = new_locked_contract (entry_points, named_keys) ]
    // POC050, POC050_hash = contract_hash [

    runtime::put_key("POC050", contract_hash.into());
    runtime::put_key("POC050_hash", storage::new_uref(contract_hash).into());

    // POC050, POC050_hash = contract_hash ]
}

// call ]
// api ]
// lib [
// _transfer, _transfer_from [

/// _transfer internal function

fn _transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
    let sender_key = balance_key(&sender);
    let recipient_key = balance_key(&recipient);

    let new_sender_balance: U256 = (get_key::<U256>(&sender_key) - amount);
    set_key(&sender_key, new_sender_balance);
    
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
    set_key(&recipient_key, new_recipient_balance);
}

/// _transfer_from internal function

fn _transfer_from(owner: AccountHash, recipient: AccountHash, amount: U256) {
    let key = allowance_key(&owner, &runtime::get_caller());
    _transfer(owner, recipient, amount);
    _approve(
        owner,
        runtime::get_caller(),
        (get_key::<U256>(&key) - amount),
    );
}

// _transfer, _transfer_from ]
// _approve (owner, spender, amount) [

/// _approve internal function

fn _approve(owner: AccountHash, spender: AccountHash, amount: U256) {
    set_key(&allowance_key(&owner, &spender), amount);
}

// _approve (owner, spender, amount) ]
// _increase_allowance (spender, addedValue) [

/// _increase_allowance internal function
// @dev Increase the amount of tokens that an owner allowed to a spender.
// approve should be called when allowed_[_spender] == 0. To increment
// allowed value is better to use this function to avoid 2 calls (and wait until
// the first transaction is mined)
// @param spender The address which will spend the funds.
// @param addedValue The amount of tokens to increase the allowance by.

fn _increase_allowance(spender: AccountHash, added_value: U256) {
    let key = allowance_key(
        &runtime::get_caller(), 
        &spender);
    let new_value = get_key::<U256>(&key) + added_value;
    set_key(&key, new_value);
}

// _increase_allowance (spender, addedValue) ]
// _decrease_allowance (spender, subtracted_value) [

/// _decrease_allowance internal function
// @dev Decrease the amount of tokens that an owner allowed to a spender.
// approve should be called when allowed_[_spender] == 0. To decrement
// allowed value is better to use this function to avoid 2 calls (and wait until
// the first transaction is mined)
// @param spender The address which will spend the funds.
// @param subtracted_value The amount of tokens to decrease the allowance by.

fn _decrease_allowance(spender: AccountHash, subtracted_value: U256) {
    let key = allowance_key(
        &runtime::get_caller(), 
        &spender);
    let new_value = get_key::<U256>(&key) - subtracted_value;
    set_key(&key, new_value);
}

// _decrease_allowance (spender, subtracted_value) ]
// _mint (account, value) [

/// _mint internal function
// @dev Internal function that mints an amount of the token and assigns it to
// an account. This encapsulates the modification of balances such that the
// proper events are emitted.
// @param account The account that will receive the created tokens.
// @param value The amount that will be created.

fn _mint(account: AccountHash, amount: U256) {
    let recipient_key = balance_key(&account);
    let minter_allowed_key = minter_allowed_key(&runtime::get_caller());

    let new_total_supply: U256 = get_key::<U256>("total_supply") + amount;
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
    let new_minter_allowed_amount: U256 = (get_key::<U256>(&minter_allowed_key) - amount);

    set_key("total_supply", new_total_supply);
    set_key(&recipient_key, new_recipient_balance);
    set_key(&minter_allowed_key, new_minter_allowed_amount);
}

// _mint (account, value) ]
// _burn (account, value) ]

/// _burn internal function
// @dev Internal function that burns an amount of the token of a given
// account.
// @param account The account whose tokens will be burnt.
// @param value The amount that will be burnt.

fn _burn(account: AccountHash, amount: U256) {
    let recipient_key = balance_key(&account);

    let new_total_supply: U256 = get_key::<U256>("total_supply") - amount;
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) - amount);

    set_key("total_supply", new_total_supply);
    set_key(&recipient_key, new_recipient_balance);
}

// _burn (account, value) ]
// _burn_from (account, value) [

/// _burn_from intenal function
// @dev Internal function that burns an amount of the token of a given
// account, deducting from the sender's allowance for said account. Uses the
// internal burn function.
// @param account The account whose tokens will be burnt.
// @param value The amount that will be burnt.

fn _burn_from(account: AccountHash, value: U256) {

    // allowance (account, sender) minus (value) [
    
    let key = allowance_key(&account, &runtime::get_caller());
    let new_value = get_key::<U256>(&key) - value;
    set_key(&key, new_value);

    // allowance (account, sender) minus (value) ]

    _burn(account, value);
}

// _burn_from (account, value) ]
// ret [

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

// ret ]
