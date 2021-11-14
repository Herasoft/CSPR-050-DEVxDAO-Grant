extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};

use core::convert::TryInto;

use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256,
};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

// get_key(name) - storage::read -> value [

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

// get_key(name) - storage::read -> value ]
// set_key(name, value) storage::write(value) [

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

// set_key(name, value) storage::write(value) ]
// map ]
// balance_key (account) -> "balances_$account" [

pub fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}
    
// balance_key (account) -> "balances_$account" ]
// allowance_key (owner, sender) -> "allowances_$owner_$sender" [

pub fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("allowances_{}_{}", owner, sender)
}

// allowance_key (owner, sender) -> "allowances_$owner_$sender" ]
// minters_key (account) -> "minters_$account" [

pub fn minters_key(account: &AccountHash) -> String {
    format!("minters_{}", account)
}

// minters_key (account) -> "minters_$account" ]
// minter_allowed_key (account) -> "minter_allowed_$account" [

pub fn minter_allowed_key(account: &AccountHash) -> String {
    format!("minter_allowed_{}", account)
}

// minter_allowed_key (account) -> "minter_allowed_$account" ]
// blacklisted_key (account) -> "blacklisted_$account" [

pub fn blacklisted_key(account: &AccountHash) -> String {
    format!("blacklisted_{}", account)
}
    
// blacklisted_key (account) -> "blacklisted_$account" ]
