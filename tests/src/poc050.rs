// use casper [

use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, AsymmetricType, CLTyped, PublicKey,
    RuntimeArgs, U256, U512,
};
    
// use casper ]
// token_cfg NAME=POC050 SYMBOL=POC DECIMALS=8 total_supply=1000 [

pub mod token_cfg {
    use super::*;
    pub const NAME: &str = "POC050";
    pub const SYMBOL: &str = "POC";
    pub const DECIMALS: u8 = 16;
    pub const CURRENCY: &str = "NAUTS";
    pub fn total_supply() -> U256 {
        21_000_000.into()
    }
}

// token_cfg NAME=POC050 SYMBOL=POC DECIMALS=8 total_supply=1000 ]
// Sender [

pub struct Sender(pub AccountHash);

// Sender ]
// Token [

pub struct Token {
    context: TestContext,
    pub futjin: AccountHash,
    pub bro: AccountHash,
    pub dax: AccountHash,
}

// Token ]
// Token impl [

impl Token {
    
    // deployed -> Token [

    pub fn deployed() -> Token {
        let futjin = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bro = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let dax = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(futjin.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bro.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {
            "token_name" => token_cfg::NAME,
            "token_symbol" => token_cfg::SYMBOL,
            "token_decimals" => token_cfg::DECIMALS,
            "token_total_supply" => token_cfg::total_supply(),
            //
            "currency" => token_cfg::CURRENCY
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address((&futjin).to_account_hash())
            .with_authorization_keys(&[futjin.to_account_hash()])
            .build();
        context.run(session);
        Token {
            context,
            futjin: futjin.to_account_hash(),
            bro: bro.to_account_hash(),
            dax: dax.to_account_hash(),
        }
    }

    // deployed -> Token ]
    // contract_hash -> Hash [

    fn contract_hash(&self) -> Hash {
        self.context
            .query(self.futjin, &[format!("{}_hash", token_cfg::NAME)])
            .unwrap_or_else(|_| panic!("{} contract not found", token_cfg::NAME))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", token_cfg::NAME))
    }

    // contract_hash -> Hash ]
    // query_contract [

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.futjin, &[token_cfg::NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    // query_contract ]
    // call [

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    // call ]
    // name, symbol, decimals, currency [

    pub fn name(&self) -> String {
        self.query_contract("name").unwrap()
    }

    pub fn symbol(&self) -> String {
        self.query_contract("symbol").unwrap()
    }

    pub fn decimals(&self) -> u8 {
        self.query_contract("decimals").unwrap()
    }

    pub fn currency(&self) -> String {
        self.query_contract("currency").unwrap()
    }

    pub fn owner(&self) -> AccountHash {
        self.query_contract("owner").unwrap()
    }

    // name, symbol, decimals, currency ]
    // balance_of (account) -> U256 [

    pub fn balance_of(&self, account: AccountHash) -> U256 {
        let key = format!("balances_{}", account);
        self.query_contract(&key).unwrap_or_default()
    }

    // balance_of (account) -> U256 ]
    // allowance (owner, spender) [

    pub fn allowance(&self, owner: AccountHash, spender: AccountHash) -> U256 {
        let key = format!("allowances_{}_{}", owner, spender);
        self.query_contract(&key).unwrap_or_default()
    }

    // allowance (owner, spender) ]
    // transfer [

    pub fn transfer(&mut self, recipient: AccountHash, amount: U256, sender: Sender) {
        self.call(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    // transfer ]
    // approve [

    pub fn approve(&mut self, spender: AccountHash, amount: U256, sender: Sender) {
        self.call(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount
            },
        );
    }

    // approve ]
    // transfer_from [

    pub fn transfer_from(
        &mut self,
        owner: AccountHash,
        recipient: AccountHash,
        amount: U256,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    // transfer_from ]
    // increase_allowance (address spender, uint256 added_value) [

    pub fn increase_allowance(&mut self, spender: AccountHash, added_value: U256, sender: Sender) -> bool {
        self.call(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender,
                "added_value" => added_value
            },
        );
        true
    }

    // increase_allowance (address spender, uint256 added_value) ]
    // decrease_allowance (address spender, uint256 subtracted_value) [

    pub fn decrease_allowance(&mut self, spender: AccountHash, subtracted_value: U256, sender: Sender) -> bool {
        self.call(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender,
                "subtracted_value" => subtracted_value
            },
        );
        true
    }

    // decrease_allowance (address spender, uint256 subtracted_value) ]
    // mint (account, value) [

    pub fn mint(&mut self, account: AccountHash, value: U256, sender: Sender) -> bool {
        self.call(
            sender,
            "mint",
            runtime_args! {
                "account" => account,
                "value" => value
            },
        );
        true
    }
    
    // mint (account, value) ]
    // burn (account, value) [

    pub fn burn(&mut self, account: AccountHash, value: U256, sender: Sender) -> bool {
        self.call(
            sender,
            "burn",
            runtime_args! {
                "account" => account,
                "value" => value
            },
        );
        true
    }
    
    // burn (account, value) ]
    // burn_from (account, value) [

    pub fn burn_from(&mut self, account: AccountHash, value: U256, sender: Sender) -> bool {
        self.call(
            sender,
            "burn_from",
            runtime_args! {
                "account" => account,
                "value" => value
            },
        );
        true
    }
    
    // burn_from (account, value) ]
}

// Token impl ]
