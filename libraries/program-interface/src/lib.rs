//! Crate for defining and implementing Solana program interfaces
//! for instructions
extern crate self as spl_program_interface;

// Simply exporting both the proc_macro crate and the syn crate
// so that everything is available downstream
pub use spl_program_interface_derive::SplProgramInterface;
pub use spl_program_interface_syn::*;
