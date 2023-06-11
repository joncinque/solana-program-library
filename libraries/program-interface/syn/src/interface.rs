//! Crate defining the `Interface` trait and related types
//! and checks.
//!
//! Also provides the collection of currently accepted
//! sRFC interfaces

use quote::quote;
use solana_program::program_error::ProgramError;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction;
use std::collections::{HashMap, HashSet};
use syn::{ItemFn, Type, Variant};

use crate::error::SplProgramInterfaceError;

/// Trait for implementing Shank & Native programs to
/// build a processor
///
/// The derive macro `#[derive(SplInterfaceInstruction)]`
/// will implement this trait for you
pub trait InterfaceInstructionPack: Sized {
    /// Unpacks an instruction from a buffer
    fn unpack(buf: &[u8]) -> Result<Self, ProgramError>;
    /// Packs an instruction into a buffer
    fn pack<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ProgramError>;
}

/// Trait defining a Solana program interface
pub trait Interface {
    /// The interface's namespace
    const NAMESPACE: &'static str;
    /// The instructions required by the interface
    fn instructions() -> Vec<InterfaceInstruction>;
    /// Returns the instructions required by the interface
    /// as a set for evaluation
    fn instruction_set() -> HashSet<InterfaceInstruction> {
        let mut set = HashSet::new();
        for instruction in Self::instructions() {
            set.insert(instruction);
        }
        set
    }
}

/// Trait defining a Solana program interface instruction
#[derive(PartialEq, Eq, Hash)]
pub struct InterfaceInstruction {
    /// The interface's namespace
    pub interface_namespace: String,
    /// The instruction's namespace
    pub instruction_namespace: String,
    /// The instruction's required arguments
    pub required_args: Vec<RequiredArg>,
}
impl InterfaceInstruction {
    /// Returns the 8-byte discriminator for the instruction
    pub fn discriminator(&self) -> [u8; 8] {
        let mut disc = [0u8; 8];
        disc.copy_from_slice(
            &solana_program::hash::hash(
                (self.interface_namespace.to_string() + ":" + &self.instruction_namespace)
                    .as_bytes(),
            )
            .to_bytes()[..8],
        );
        disc
    }
    /// Converts an instruction namespace and `&ItemFn` to an
    /// `InterfaceInstruction` for evaluation (Anchor)
    pub fn from_item_fn(
        interface_namespace: &String,
        instruction_namespace: &String,
        function: &ItemFn,
    ) -> Self {
        let mut required_args = vec![];
        for arg in &function.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    required_args.push((ident.ident.to_string(), *pat_type.ty.clone()));
                }
            }
        }
        Self {
            interface_namespace: interface_namespace.to_string(),
            instruction_namespace: instruction_namespace.to_string(),
            required_args,
        }
    }
    /// Converts an instruction namespace and `&Variant` to an
    /// `InterfaceInstruction` for evaluation (Native, Shank)
    pub fn from_variant(
        interface_namespace: &String,
        instruction_namespace: &String,
        variant: &Variant,
    ) -> Self {
        let mut required_args = vec![];
        for field in &variant.fields {
            if let Some(ident) = &field.ident {
                required_args.push((ident.to_string(), field.ty.clone()));
            }
        }
        Self {
            interface_namespace: interface_namespace.to_string(),
            instruction_namespace: instruction_namespace.to_string(),
            required_args,
        }
    }
}

/// A required argument for an instruction
pub type RequiredArg = (String, Type);

/// Evaluates a program's interface instructions against any declared interfaces
pub fn evaluate_interface_instructions(
    declared_instructions: Vec<InterfaceInstruction>,
) -> Result<(), SplProgramInterfaceError> {
    // Initialize a HashMap to keep track of all declared interfaces
    let mut declared_interfaces: HashMap<String, HashSet<InterfaceInstruction>> = HashMap::new();
    // Iterate through all declared instructions and
    // evaluate them against the declared interfaces
    for declared_ix in declared_instructions {
        if declared_ix.interface_namespace == TokenMetadataInstruction::NAMESPACE {
            process_declared_instruction::<TokenMetadataInstruction>(
                &mut declared_interfaces,
                declared_ix,
            )?
        } else {
            return Err(SplProgramInterfaceError::InvalidInterfaceNamespace);
        }
    }
    // Make sure all declared interfaces have no remaining unmatched instructions
    for x in declared_interfaces.values() {
        if !x.is_empty() {
            dump_remaining_interface_instructions(declared_interfaces);
            return Err(SplProgramInterfaceError::InstructionMissing);
        }
    }
    Ok(())
}

/// Processed a declared instruction by checking to see if it exists in the `HashMap`
/// and if it does, removing the instruction from the `HashSet`
fn process_declared_instruction<I: Interface>(
    declared_interfaces: &mut HashMap<String, HashSet<InterfaceInstruction>>,
    declared_ix: InterfaceInstruction,
) -> Result<(), SplProgramInterfaceError> {
    match declared_interfaces.get_mut(&declared_ix.interface_namespace) {
        Some(set) => {
            if !set.remove(&declared_ix) {
                return Err(SplProgramInterfaceError::InstructionNotFound);
            }
        }
        None => {
            let mut set = I::instruction_set();
            if !set.remove(&declared_ix) {
                match set
                    .iter()
                    .find(|i| &i.instruction_namespace == &declared_ix.instruction_namespace)
                {
                    Some(ins) => {
                        println!(
                            "\n\nIncorrect arguments for interface instruction `{}::{}`:\n",
                            declared_ix.interface_namespace, declared_ix.instruction_namespace
                        );
                        println!("Provided arguments:");
                        for arg in &declared_ix.required_args {
                            let ty = arg.1.clone();
                            println!("  - {}: {}", arg.0, quote! {#ty}.to_string());
                        }
                        println!("\n");
                        println!("Required arguments:");
                        for arg in &ins.required_args {
                            let ty = arg.1.clone();
                            println!("  - {}: {}", arg.0, quote! {#ty}.to_string());
                        }
                        println!("\n");
                        return Err(SplProgramInterfaceError::MissingArgument);
                    }
                    None => {
                        println!("\n\nFound the following unknown interface instructions:\n");
                        println!(
                            "  - {}::{}",
                            declared_ix.interface_namespace, declared_ix.instruction_namespace
                        );
                        return Err(SplProgramInterfaceError::InstructionNotFound);
                    }
                }
            }
            declared_interfaces.insert(declared_ix.interface_namespace, set);
        }
    }
    Ok(())
}

/// Dumps any remaining interface instructions in the evaluation
/// set for error reporting
fn dump_remaining_interface_instructions(
    declared_interfaces: HashMap<String, HashSet<InterfaceInstruction>>,
) {
    println!("\n\nThe following interface instructions were not implemented:\n");
    for (namespace, set) in declared_interfaces {
        for ix in set {
            println!("   - {}::{}", namespace, ix.instruction_namespace);
        }
    }
    println!("\n");
}
