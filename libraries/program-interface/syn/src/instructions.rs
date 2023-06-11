//! Implements the `Interface` traits and associated types for
//! the interfaces contained within the Solana Program Library

use spl_token_metadata_interface::instruction::{
    Emit, Initialize, RemoveKey, TokenMetadataInstruction, UpdateAuthority, UpdateField,
};
use syn::parse_quote;

use crate::interface::{Interface, InterfaceInstruction};

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
            required_args: vec![
                ("name".to_string(), parse_quote! { String }),
                ("symbol".to_string(), parse_quote! { String }),
                ("uri".to_string(), parse_quote! { String }),
            ],
        }
    }
}
impl AsInterfaceInstruction for UpdateField {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "update_field".to_string(),
            required_args: vec![
                ("field".to_string(), parse_quote! { Field }),
                ("value".to_string(), parse_quote! { String }),
            ],
        }
    }
}
impl AsInterfaceInstruction for RemoveKey {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "remove_a_key".to_string(),
            required_args: vec![("key".to_string(), parse_quote! { String })],
        }
    }
}
impl AsInterfaceInstruction for UpdateAuthority {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "update_authority".to_string(),
            required_args: vec![("new_authority".to_string(), parse_quote! { Option<Pubkey> })],
        }
    }
}
impl AsInterfaceInstruction for Emit {
    fn as_interface_instruction() -> InterfaceInstruction {
        InterfaceInstruction {
            interface_namespace: TokenMetadataInstruction::NAMESPACE.to_string(),
            instruction_namespace: "emitting".to_string(),
            required_args: vec![
                ("start".to_string(), parse_quote! { Option<u64> }),
                ("end".to_string(), parse_quote! { Option<u64> }),
            ],
        }
    }
}
