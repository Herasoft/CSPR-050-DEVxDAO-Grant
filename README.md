# POC050 Contract
POC050 purpose is collateral backed stable coin

[![asciicast](https://asciinema.org/a/i9xTxSiv7sRYPn5BwShryHk0K.svg)](https://asciinema.org/a/i9xTxSiv7sRYPn5BwShryHk0K)

# POC-050

Implementation of smart contract Casper network WASM POC-050

## Prerequirenments

- rust toolchain nightly-2021-06-17

## Install
Make sure the `wasm32-unknown-unknown` target is installed.
```
make prepare
```

### Build the Contract
```
make build-contract
```

### Test
```
make test
```

### Verify the package
Use `make test-clean` command to check the dependencies of the package
```
make test-clean
```

## Documentation

### Directory Structure

POC-050 Smart-Contract 
```
contract/           # contract source code
  src/
    data.rs         # data utils
    entry_points.rs # casper entry points
    main.rs         # poc050 contract main
  Cargo.toml        # contract cargo config
```

POC-050 Smart-Contract Test
```
tests/
  src/
    lib.rs          # lib mod
    poc050.rs       # poc050 contract test link
    tests_casper.rs # casper core tests  
    tests_mint.rs   # poc050 mint tests
    tests.rs        # poc050 main tests
  Cargo.toml        # tests cargo config

```

### Contract API

#### Contract meta functions

get contract name
```rust
fn name(): String
```
get contract symbol
```rust
fn symbol(): String
```
get contract decimals
```rust
fn decimals(): u8
```
get contract total_supply
```rust
fn total_supply(): U256
```
get contract currency
```rust
fn currency(): String
```
get contract owner
```rust
fn owner(): U256
```
get contract owner
```rust
fn owner(): U256
```

#### Contract logic functions

get balance_of (account)

Gets the balance of the specified address.
- account The address to query the balance of.
Return An U256 representing the amount owned by the passed address.

```rust
fn balance_of(account: AccountHash): U256
```

get allowance (owner, spender)

Function to check the amount of tokens that an owner allowed to a spender.
- owner address The address which owns the funds.
- spender address The address which will spend the funds.
Return A U256 specifying the amount of tokens still available for the spender.

```rust
fn allowance(owner: AccountHash, spender: AccountHash): U256
```

approve (spender, amount)

Approve the passed address to spend the specified amount of tokens on behalf of msg.sender.
Beware that changing an allowance with this method brings the risk that someone may use both the old
and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this
race condition is to first reduce the spender's allowance to 0 and set the desired value afterwards:
https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
- spender The address which will spend the funds.
- value The amount of tokens to be spent.

```rust
fm approve(spender: AccountHash, amount: U256)
```

transfer (recipient, amount)

Transfer token for a specified address
- recipient The address to transfer to.
- amount The amount to be transferred.

```rust
fn transfer(recipient: AccountHash, amount: U256)
```

transfer_from (owner, recipient, amount) 

Transfer tokens from one address to another
- owner The address which you want to send tokens from
- recipient The address which you want to transfer to
- amount The amount of tokens to be transferred

```rust
fn transfer_from(owner: AccountHash, recipient: AccountHash, amount: U256) 
```

increase_allowance (spender, added_value)

Increase the amount of tokens that an owner allowed to a spender.
approve should be called when allowed_[_spender] == 0. 
To increment allowed value is better to use this function to avoid 2 calls (and wait until the first transaction is mined)
- spender The address which will spend the funds.
- added_value The amount of tokens to increase the allowance by.

```rust
fn increase_allowance(spender: AccountHash, added_value: U256)
```

decrease_allowance (spender, subtracted_value)

Decrease the amount of tokens that an owner allowed to a spender.
approve should be called when allowed_[_spender] == 0. 
To decrement allowed value is better to use this function to avoid 2 calls (and wait until the first transaction is mined)
- spender The address which will spend the funds.
- subtracted_value The amount of tokens to decrease the allowance by.

```rust
fn decrease_allowance(spender AccountHash, subtracted_value: U256)
```

mint (account, value)

Function that mints an amount of the token and assigns it to an account. This encapsulates the modification of balances such that the proper events are emitted.
- account The account that will receive the created tokens.
- value The amount that will be created.

```rust
fn mint(accoutn:AccountHash, value: U256)
```

burn (account, value)

Function that burns an amount of the token of a given account.
- account The account whose tokens will be burnt.
- value The amount that will be burnt.

```rust
fn burn(accoutn:AccountHash, value: U256)
```

burn_from (account, value)

Function that burns an amount of the token of a given account, deducting from the sender's allowance for said account. Uses the internal burn function.
- account The account whose tokens will be burnt.
- value The amount that will be burnt.

```rust
fn burn_from(accoutn:AccountHash, value: U256)
```

### Examples

See tests/src/tests.rs file with working examples and how to use poc050 functions propertly

### License
See LICENSE file
