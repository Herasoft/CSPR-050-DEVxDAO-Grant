//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};
/*
use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};*/
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256,
};

// endpoint [

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// endpoint ]
// default [

/// Returns the default set of POC050 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("decimals", vec![], CLType::U8));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256));

    entry_points.add_entry_point(endpoint(
        "transfer",
        vec![
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));

    entry_points.add_entry_point(endpoint(
        "allowance",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("spender", AccountHash::cl_type()),
        ],
        CLType::U256,
    ));

    entry_points.add_entry_point(endpoint(
        "approve",
        vec![
            Parameter::new("spender", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "transfer_from",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "mint",
        vec![
            Parameter::new("account", AccountHash::cl_type()),
            Parameter::new("value", CLType::U256),
        ],
        CLType::Unit
    ));

    entry_points.add_entry_point(endpoint(
        "burn",
        vec![
            Parameter::new("account", AccountHash::cl_type()),
            Parameter::new("value", CLType::U256),
        ],
        CLType::Unit
    ));

    entry_points.add_entry_point(endpoint(
        "burnFrom",
        vec![
            Parameter::new("account", AccountHash::cl_type()),
            Parameter::new("value", CLType::U256),
        ],
        CLType::Unit
    ));

    entry_points.add_entry_point(endpoint(
        "increase_allowance",
        vec![
            Parameter::new("spender", AccountHash::cl_type()),
            Parameter::new("addedValue", CLType::U256),
        ],
        CLType::Unit
    ));
    
    entry_points.add_entry_point(endpoint(
        "decrease_allowance",
        vec![
            Parameter::new("spender", AccountHash::cl_type()),
            Parameter::new("subtractedValue", CLType::U256),
        ],
        CLType::Unit
    ));
        
    entry_points
}

// default ]
