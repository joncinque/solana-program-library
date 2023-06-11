//! Errors for the SPL interface instruction parser.

use quote::quote;
use std::collections::{HashMap, HashSet};

use crate::interface::{InterfaceInstruction, RequiredArg};

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum SplProgramInterfaceError {
    #[error("Error parsing interface attribute")]
    ParseError,
    #[error("Invalid interface namespace")]
    InvalidInterfaceNamespace,
    #[error("Invalid instruction namespace")]
    InvalidInstruction,
    #[error("Instruction not found")]
    InstructionMissing,
    #[error("Instruction not found")]
    InstructionNotFound,
    #[error("Missing argument(s) for instruction")]
    MissingArgument,
}

pub fn invalid_instruction_namespace_verbose(
    declared_interfaces: &HashMap<String, HashSet<InterfaceInstruction>>,
) -> SplProgramInterfaceError {
    println!("\n\nThe following interface instructions were not implemented:\n");
    for (namespace, set) in declared_interfaces {
        for ix in set {
            println!("   - {}::{}", namespace, ix.instruction_namespace);
        }
    }
    println!("\n");
    SplProgramInterfaceError::InstructionMissing
}

pub fn instruction_missing_verbose(
    interface_namespace: &str,
    instruction_namespace: &str,
) -> SplProgramInterfaceError {
    println!("\n\nFound the following unknown interface instructions:\n");
    println!("  - {}::{}", interface_namespace, instruction_namespace);
    println!("\n");
    SplProgramInterfaceError::InstructionNotFound
}

pub fn missing_args_verbose(
    interface_namespace: &str,
    instruction_namespace: &str,
    provided_args: &Vec<RequiredArg>,
    required_args: &Vec<RequiredArg>,
) -> SplProgramInterfaceError {
    println!(
        "\n\nIncorrect arguments for interface instruction `{}::{}`:\n",
        interface_namespace, instruction_namespace
    );
    println!("Provided arguments:");
    for arg in provided_args {
        let ty = arg.1.clone();
        println!("  - {}: {}", arg.0, quote! {#ty});
    }
    println!("\n");
    println!("Required arguments:");
    for arg in required_args {
        let ty = arg.1.clone();
        println!("  - {}: {}", arg.0, quote! {#ty});
    }
    println!("\n");
    SplProgramInterfaceError::MissingArgument
}
