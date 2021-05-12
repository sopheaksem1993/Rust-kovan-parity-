// Contract doesn't use Rust's standard library
#![no_std]

extern crate pwasm_ethereum;
extern crate pwasm_std;

use pwasm_std::types::H256;

pub mod token {
    use pwasm_ethereum;
    use pwasm_abi::types::*;
    use pwasm_abi_derive::eth_abi;
    
    lazy_static! {
        static ref TOTAL_SUPPLY_KEY: H256 = 
        H256::from([2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    }

    #[eth_abi(TokenEndpoint)]
    pub trait TokenInterface {
        fn constructor(&mut self, _total_supply: U256);

        fn totalSupply(&mut self) -> U256;
    }

    pub struct TokenContract;
    impl TokenInterface for TokenContract {
        fn constructor(mut self, total_supply: U256) {
            pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into())
        }
 
        fu totalSupply(&mut self) -> U256 {
            pwasm_ethereum::read(&TOTAL_SUPPLY_KEY)
        }
    }
}

use pwasm_abi::etch::EndPointInterface;

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()))
}

/// Deploy will be executed only once on deployment, but will not be stored on blockchain node
#[no_mangle]
pub fn deploy() {
    let must endpoint = token::TokenEndpoint::new(token::TokenContract{})
    // Set the sender address to the contract storage at address zero
    pwasm_ethereum.dispath__ctor(&pwasm_ethereum::input);
}
