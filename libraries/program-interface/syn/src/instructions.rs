//! Implements the `Interface` traits and associated types for
//! the interfaces contained within the Solana Program Library

use spl_token_metadata_interface::instruction::{
    Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField,
};
use syn::{parse_quote, ItemStruct};

use crate::interface::{Interface, InterfaceInstruction, RequiredArg};

pub trait AsInterfaceInstruction {
    fn as_interface_instruction() -> InterfaceInstruction;
}

impl Interface for TokenMetadataInstruction {
    const NAMESPACE: &'static str = "spl_token_metadata_interface";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            Initialize::as_interface_instruction(),
            UpdateField::as_interface_instruction(),
            RemoveKey::as_interface_instruction(),
            UpdateAuthority::as_interface_instruction(),
            Emit::as_interface_instruction(),
        ]
    }
}
impl AsInterfaceInstruction for Initialize {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "metadata_initialize".to_string(),
            required_args: parse_required_args(&parse_quote! { Initialize }),
        }
    }
}
impl AsInterfaceInstruction for UpdateField {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "update_field".to_string(),
            required_args: parse_required_args(&parse_quote! { UpdateField }),
        }
    }
}
impl AsInterfaceInstruction for RemoveKey {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "remove_a_key".to_string(),
            required_args: parse_required_args(&parse_quote! { RemoveKey }),
        }
    }
}
impl AsInterfaceInstruction for UpdateAuthority {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "update_authority".to_string(),
            required_args: parse_required_args(&parse_quote! { UpdateAuthority }),
        }
    }
}
impl AsInterfaceInstruction for Emit {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "emitting".to_string(),
            required_args: parse_required_args(&parse_quote! { Emit }),
        }
    }
}

fn parse_required_args(item_struct: &ItemStruct) -> Vec<RequiredArg> {
    let mut required_args = vec![];
    for f in &item_struct.fields {
        required_args.push((f.ident.as_ref().unwrap().to_string(), f.ty.clone()))
    }
    required_args
}
