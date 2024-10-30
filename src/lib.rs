//!
//! Stylus Journal
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::{string::String, vec::Vec};
use alloy_primitives::{Address, U256, U64};
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{block, msg, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Journal {
        mapping(address => Entry[]) journals;
    }

    pub struct Entry {
        string title;
        string body;
        uint64 timestamp;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[external]
impl Journal {
    pub fn new_entry(&mut self, title: String, body: String) {
        let timestamp = block::timestamp();
        let sender = msg::sender();
        let mut journal = self.journals.setter(sender);
        let mut new_element = journal.grow();
        new_element.title.set_str(title);
        new_element.body.set_str(body);
        new_element.timestamp.set(U64::from(timestamp));
    }

    pub fn get_entry(&self, address: Address, idx: U256) -> Result<Vec<u8>, Vec<u8>> {
        let journal = self.journals.get(address);
        if let Some(entry) = journal.get(idx) {
            // Ok((
            //     entry.title.get_string(),
            //     entry.body.get_string(),
            //     entry.timestamp.get(),
            // ))
            Ok(entry.body.get_string().as_bytes().to_vec())
        } else {
            Err(vec![0])
        }
    }
}
