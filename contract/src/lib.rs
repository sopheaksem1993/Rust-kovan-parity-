// Contract doesn't use Rust's standard library
#![no_std]

extern crate pwasm_ethereum;
extern crate pwasm_std;

use pwasm_std::types::H256;

/// Deploy will be executed only once on deployment, but will not be stored on blockchain node
#[no_mangle]
pub fn deploy() {
    // Set the sender address to the contract storage at address zero
    pwasm_ethereum::write(&H256::zero().into(), &H256::from(pwasm_ethereum::sender()).into());
}

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call() {
    // Read the address of the deployer
    let owner = pwasm_ethereum::read(&H256::zero().into());
    // Send a result pointer to the runtime
    pwasm_ethereum::ret(owner.as_ref());
}
