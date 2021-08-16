// 0 [

#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

// 0 ]
// use alloc [

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

// use alloc ]
// use casper [

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



// ***





//    mapping(address => uint256) internal balances;
//    mapping(address => mapping(address => uint256)) internal allowed;
//    uint256 internal totalSupply_ = 0;

//    mapping(address => bool) internal minters;
//    mapping(address => uint256) internal minterAllowed;

//    address public blacklister;
//    mapping(address => bool) internal blacklisted;

//  address public pauser;
//  bool public paused = false;



// ***




// increaseAllowance (address spender, uint256 addedValue) [

#[no_mangle]
pub extern "C" fn increaseAllowance() {
    let spender: AccountHash = runtime::get_named_arg("spender");
    let addedValue: U256 = runtime::get_named_arg("addedValue");
    let val = _increaseAllowance(spender, addedValue);
    ret(val);
}

// increaseAllowance (address spender, uint256 addedValue) ]
// decreaseAllowance (address spender, uint256 subtractedValue) [

#[no_mangle]
pub extern "C" fn decreaseAllowance() {
    let spender: AccountHash = runtime::get_named_arg("spender");
    let subtractedValue: U256 = runtime::get_named_arg("subtractedValue");
    let val = _decreaseAllowance(spender, subtractedValue);
    ret(val);
}

// decreaseAllowance (address spender, uint256 subtractedValue) ]
// mint (account, value) [

#[no_mangle]
pub extern "C" fn mint() {
    let account: AccountHash = runtime::get_named_arg("account");
    let value: U256 = runtime::get_named_arg("value");
    let val = _mint(account, value);
    ret(val);
}

// mint (account, tokenId) ]
// burn (account, tokenId) [

#[no_mangle]
pub extern "C" fn burn() {
    let account: AccountHash = runtime::get_named_arg("account");
    let value: U256 = runtime::get_named_arg("value");
    let val = _burn(account, value);
    ret(val);
}


// burn (account, value) ]
// burnFrom (account, tokenId) [

#[no_mangle]
pub extern "C" fn burnFrom() {
    let account: AccountHash = runtime::get_named_arg("account");
    let value: U256 = runtime::get_named_arg("value");
    let val = _burnFrom(account, value);
    ret(val);
}


// burnFrom (account, value) ]



// ***




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

    // fns [

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("decimals", vec![], CLType::U8));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U32));
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
        CLType::String
    ));
    entry_points.add_entry_point(endpoint(
        "burn",
        vec![
            Parameter::new("account", AccountHash::cl_type()),
            Parameter::new("value", CLType::U256),
        ],
        CLType::String
    ));
    entry_points.add_entry_point(endpoint(
        "burnFrom",
        vec![
            Parameter::new("account", AccountHash::cl_type()),
            Parameter::new("value", CLType::U256),
        ],
        CLType::String
    ));

    // fns ]
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

fn _transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
    let sender_key = balance_key(&sender);
    let recipient_key = balance_key(&recipient);
    let new_sender_balance: U256 = (get_key::<U256>(&sender_key) - amount);
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>(&recipient_key) + amount);
    set_key(&recipient_key, new_recipient_balance);
}

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
// _approve [

fn _approve(owner: AccountHash, spender: AccountHash, amount: U256) {
    set_key(&allowance_key(&owner, &spender), amount);
}

// _approve ]




// ***



// _increaseAllowance (address spender, uint256 addedValue) [

/**
* @dev Increase the amount of tokens that an owner allowed to a spender.
* approve should be called when allowed_[_spender] == 0. To increment
* allowed value is better to use this function to avoid 2 calls (and wait until
* the first transaction is mined)
* From MonolithDAO Token.sol
* @param spender The address which will spend the funds.
* @param addedValue The amount of tokens to increase the allowance by.
*/

fn _increaseAllowance(spender: AccountHash, addedValue: U256) {
//    Ok(true)
}

/*
function increaseAllowance(
address spender,
uint256 addedValue
)
public
returns (bool)
{
require(spender != address(0));

_allowed[msg.sender][spender] = (
  _allowed[msg.sender][spender].add(addedValue));
emit Approval(msg.sender, spender, _allowed[msg.sender][spender]);
return true;
}
*/

// _increaseAllowance (address spender, uint256 addedValue) ]
// _decreaseAllowance (address spender, uint256 subtractedValue) [


/**
* @dev Decrease the amount of tokens that an owner allowed to a spender.
* approve should be called when allowed_[_spender] == 0. To decrement
* allowed value is better to use this function to avoid 2 calls (and wait until
* the first transaction is mined)
* From MonolithDAO Token.sol
* @param spender The address which will spend the funds.
* @param subtractedValue The amount of tokens to decrease the allowance by.
*/

fn _decreaseAllowance(spender: AccountHash, subtractedValue: U256) {
//    Ok(true)
}

/*
function decreaseAllowance(
address spender,
uint256 subtractedValue
)
public
returns (bool)
{
require(spender != address(0));

_allowed[msg.sender][spender] = (
  _allowed[msg.sender][spender].sub(subtractedValue));
emit Approval(msg.sender, spender, _allowed[msg.sender][spender]);
return true;
}
*/

// _decreaseAllowance (address spender, uint256 subtractedValue) ]


// _mint (account, value) [
/**
* @dev Internal function that mints an amount of the token and assigns it to
* an account. This encapsulates the modification of balances such that the
* proper events are emitted.
* @param account The account that will receive the created tokens.
* @param value The amount that will be created.
*/
//erc20 function _mint(address account, uint256 value) internal {
//erc20 require(account != address(0));
//erc20
//erc20 _totalSupply = _totalSupply.add(value);
//erc20 _balances[account] = _balances[account].add(value);
//erc20 emit Transfer(address(0), account, value);
//erc20 }
//
// _safeMint(address to, uint256 tokenId) [
//erc721 function _safeMint(address to, uint256 tokenId) internal virtual {
//erc721 _safeMint(to, tokenId, "");
//erc721 }
//erc721 
//erc721 function _safeMint(address to, uint256 tokenId, bytes memory _data) internal virtual {
//erc721 _mint(to, tokenId);
//erc721 require(_checkOnERC721Received(address(0), to, tokenId, _data), "ERC721: transfer to non ERC721Receiver implementer");
//erc721 }
// _safeMint(address to, uint256 tokenId) ]
//
/*
    function mint(address _to, uint256 _amount) whenNotPaused onlyMinters notBlacklisted(msg.sender) notBlacklisted(_to) public returns (bool) {
        require(_to != address(0));
        require(_amount > 0);

        uint256 mintingAllowedAmount = minterAllowed[msg.sender];
        require(_amount <= mintingAllowedAmount);

        totalSupply_ = totalSupply_.add(_amount);
        balances[_to] = balances[_to].add(_amount);
        minterAllowed[msg.sender] = mintingAllowedAmount.sub(_amount);
        emit Mint(msg.sender, _to, _amount);
        emit Transfer(address(0), _to, _amount);
        return true;
    }
*/

fn _mint(account: AccountHash, value: U256) {
//  whenNotPaused 
//  onlyMinters 
//  notBlacklisted(msg.sender)
//    Ok(true)
}

// _mint (account, value) ]
// _burn (account, value) ]

/**
* @dev Internal function that burns an amount of the token of a given
* account.
* @param account The account whose tokens will be burnt.
* @param value The amount that will be burnt.
*/
/*
//erc20 function _burn(address account, uint256 value) internal {
//erc20 require(account != address(0));
//erc20 
//erc20 _totalSupply = _totalSupply.sub(value);
//erc20 _balances[account] = _balances[account].sub(value);
//erc20 emit Transfer(account, address(0), value);
//erc20 }
*/

    /**
     * @dev allows a minter to burn some of its own tokens
     * Validates that caller is a minter and that sender is not blacklisted
     * amount is less than or equal to the minter's account balance
     * @param _amount uint256 the amount of tokens to be burned
    */
/*
    function burn(uint256 _amount) whenNotPaused onlyMinters notBlacklisted(msg.sender) public {
        uint256 balance = balances[msg.sender];
        require(_amount > 0);
        require(balance >= _amount);

        totalSupply_ = totalSupply_.sub(_amount);
        balances[msg.sender] = balance.sub(_amount);
        emit Burn(msg.sender, _amount);
        emit Transfer(msg.sender, address(0), _amount);
    }

*/

fn _burn(account: AccountHash, value: U256) {
//  whenNotPaused 
//  onlyMinters 
//  notBlacklisted(msg.sender)
//    Ok(true)
}

// _burn (account, value) ]
// _burnFrom (account, value) [

/**
* @dev Internal function that burns an amount of the token of a given
* account, deducting from the sender's allowance for said account. Uses the
* internal burn function.
* @param account The account whose tokens will be burnt.
* @param value The amount that will be burnt.
*/
/*
function _burnFrom(address account, uint256 value) internal {
// Should https://github.com/OpenZeppelin/zeppelin-solidity/issues/707 be accepted,
// this function needs to emit an event with the updated approval.
_allowed[account][msg.sender] = _allowed[account][msg.sender].sub(
  value);
_burn(account, value);
}
*/

fn _burnFrom(account: AccountHash, value: U256) {
//    Ok(true)
}

// _burnFrom (account, value) ]

// _minterAllowance(minter) -> U256 [

/**
 * @dev Get minter allowance for an account
 * @param minter The address of the minter
*/

fn _minterAllowance(minter: AccountHash) -> U256 {
//        return minterAllowed[minter];

}

/*
    function minterAllowance(address minter) public view returns (uint256) {
        return minterAllowed[minter];
    }
*/

// _minterAllowance(minter) -> U256 ]
// _isMinter(address account) -> bool [

/**
 * @dev Checks if account is a minter
 * @param account The address to check    
*/

fn _isMinter(account: AccountHash) -> bool {
//        return minters[account];

}

/*
    function isMinter(address account) public view returns (bool) {
        return minters[account];
    }
*/

// _isMinter(address account) -> bool ]
// _allowance(address owner, address spender) -> U256 [

/**
 * @dev Get allowed amount for an account
 * @param owner address The account owner
 * @param spender address The account spender
*/

fn _allowance(owner: AccountHash, spender: AccountHash) -> U256 {
//        return allowed[owner][spender];

}

/*
    function allowance(address owner, address spender) public view returns (uint256) {
        return allowed[owner][spender];
    }
*/

// _allowance(address owner, address spender) -> U256 ]
// _configureMinter(address minter, uint256 minterAllowedAmount) -> bool [

/**
 * @dev Function to add/update a new minter
 * @param minter The address of the minter
 * @param minterAllowedAmount The minting amount allowed for the minter
 * @return True if the operation was successful.
*/

fn _configureMinter(minter: AccountHash, minterAllowedAmount: U256) -> bool {

}
    /*
    function configureMinter(address minter, uint256 minterAllowedAmount) whenNotPaused onlyMasterMinter public returns (bool) {
        minters[minter] = true;
        minterAllowed[minter] = minterAllowedAmount;
        emit MinterConfigured(minter, minterAllowedAmount);
        return true;
    }
    */

// _configureMinter(address minter, uint256 minterAllowedAmount) -> bool ]
// _removeMinter(address minter) -> bool [

/**
 * @dev Function to remove a minter
 * @param minter The address of the minter to remove
 * @return True if the operation was successful.
*/

fn _removeMinter(minter: AccountHash) -> bool {

}
    /*
    function removeMinter(address minter) onlyMasterMinter public returns (bool) {
        minters[minter] = false;
        minterAllowed[minter] = 0;
        emit MinterRemoved(minter);
        return true;
    }
    */

// _removeMinter(address minter) -> bool ]
// _updateMasterMinter(address _newMasterMinter) [

fn updateMasterMinter(_newMasterMinter: AccountHash) {

}
/*
    function updateMasterMinter(address _newMasterMinter) onlyOwner public {
        require(_newMasterMinter != address(0));
        masterMinter = _newMasterMinter;
        emit MasterMinterChanged(masterMinter);
    }
*/
// _updateMasterMinter(address _newMasterMinter) ]
// _isBlacklisted(address _account) -> bool [

fn updateMasterMinter(_account: AccountHash) -> bool {

}
/*
    function isBlacklisted(address _account) public view returns (bool) {
        return blacklisted[_account];
    }
*/
// _isBlacklisted(address _account) -> bool ]
// _blacklist(address _account) [

/**
 * @dev Adds account to blacklist
 * @param _account The address to blacklist
*/

fn _blacklist(_account: AccountHash) {

}
/*
    function blacklist(address _account) public onlyBlacklister {
        blacklisted[_account] = true;
        emit Blacklisted(_account);
    }
*/

// _blacklist(address _account) ]
// _unBlacklist(address _account) [

/**
 * @dev Removes account from blacklist
 * @param _account The address to remove from the blacklist
*/

fn _unBlacklist(_account: AccountHash) {

}
/*
    function unBlacklist(address _account) public onlyBlacklister {
        blacklisted[_account] = false;
        emit UnBlacklisted(_account);
    }
*/
// _unBlacklist(address _account) ]
// _updateBlacklister(address _newBlacklister) [

fn updateBlacklister(_newBlacklister: AccountHash) {

}
/*
    function updateBlacklister(address _newBlacklister) public onlyOwner {
        require(_newBlacklister != address(0));
        blacklister = _newBlacklister;
        emit BlacklisterChanged(blacklister);
    }
*/
// _updateBlacklister(address _newBlacklister) ]



// ***



// ret [

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

// ret ]

// map [
// get_key(name) - storage::read -> value [

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
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

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}

// balance_key (account) -> "balances_$account" ]
// allowance_key (owner, sender) -> "allowances_$owner_$sender" [

fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("allowances_{}_{}", owner, sender)
}

// allowance_key (owner, sender) -> "allowances_$owner_$sender" ]
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

// lib ]
