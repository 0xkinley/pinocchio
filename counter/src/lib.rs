#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(test)]
extern crate std;

pub mod error;
pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("11111111111111111111111111111111");