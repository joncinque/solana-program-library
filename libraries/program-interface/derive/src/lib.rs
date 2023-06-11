//! Proc macro attribute for defining a Solana program interface
//! in native or Shank programs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use spl_program_interface_syn::InterfaceInstructionBuilder;
use syn::parse_macro_input;

/// Proc macro attribute for defining a Solana program interface
/// in native or Shank programs
#[proc_macro_derive(SplProgramInterface, attributes(interface))]
pub fn spl_program_interface(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as InterfaceInstructionBuilder)
        .to_token_stream()
        .into()
}
