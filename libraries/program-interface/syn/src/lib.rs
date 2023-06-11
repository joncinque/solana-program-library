//! `syn` parsing crate for validating and implementing the
//! necessary components for Solana program interface implementations

mod error;
mod instructions;
mod interface;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Attribute, ItemEnum, ItemFn};

use crate::{
    error::SplProgramInterfaceError,
    interface::{evaluate_interface_instructions, InterfaceInstruction},
};

/// "Builder" struct for the macro attribute that will run
/// the necessary checks and then generate the necessary
/// tokens to implement the interface instruction discriminators
/// (Native, Shank)
#[derive(Debug)]
pub struct InterfaceInstructionBuilder {
    pub item_enum: ItemEnum,
    pub pack_unpack: TokenStream,
}

impl TryFrom<ItemEnum> for InterfaceInstructionBuilder {
    type Error = SplProgramInterfaceError;

    fn try_from(item_enum: ItemEnum) -> Result<Self, Self::Error> {
        let pack_unpack = process_enum(&item_enum)?;
        Ok(Self {
            item_enum,
            pack_unpack,
        })
    }
}

impl Parse for InterfaceInstructionBuilder {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        ItemEnum::parse(input)?.try_into().map_err(|e| {
            syn::Error::new(
                input.span(),
                format!("Failed to parse interface instructions: {}", e),
            )
        })
    }
}

impl ToTokens for InterfaceInstructionBuilder {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

impl From<&InterfaceInstructionBuilder> for TokenStream {
    fn from(builder: &InterfaceInstructionBuilder) -> Self {
        let _item_enum = &builder.item_enum;
        let pack_unpack = &builder.pack_unpack;
        quote! {
            #pack_unpack
        }
    }
}

/// Validate the interface instructions from a collection of
/// defined functions (Anchor)
pub fn process_functions(functions: Vec<&ItemFn>) -> Result<(), SplProgramInterfaceError> {
    let mut declared_instructions = vec![];
    for func in functions {
        if let Some(interface_attr) = func
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("interface"))
        {
            if let Ok((interface_namespace, instruction_namespace)) =
                extract_interface_from_attribute(interface_attr)
            {
                declared_instructions.push(InterfaceInstruction::from_item_fn(
                    &interface_namespace,
                    &instruction_namespace,
                    func,
                ));
            }
        }
    }
    evaluate_interface_instructions(declared_instructions)
}

/// Validate the interface instructions from a defined
/// instruction enum and implement the required
/// traits (Native, Shank)
fn process_enum(item_enum: &ItemEnum) -> Result<TokenStream, SplProgramInterfaceError> {
    let mut declared_instructions = vec![];
    for variant in &item_enum.variants {
        if let Some(interface_attr) = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("interface"))
        {
            if let Ok((interface_namespace, instruction_namespace)) =
                extract_interface_from_attribute(interface_attr)
            {
                declared_instructions.push(InterfaceInstruction::from_variant(
                    &interface_namespace,
                    &instruction_namespace,
                    variant,
                ));
            }
        }
    }
    evaluate_interface_instructions(declared_instructions).map(|_| generate_pack_unpack(item_enum))
}

/// Extracts the interface namespace and instruction namespace
/// from an attribute annotation
fn extract_interface_from_attribute(
    interface_attr: &Attribute,
) -> Result<(String, String), SplProgramInterfaceError> {
    let mut res: (String, String) = (String::default(), String::default());
    match interface_attr.parse_nested_meta(|meta| {
        res = (
            meta.path.segments[0].ident.to_string(),
            meta.path.segments[1].ident.to_string(),
        );
        Ok(())
    }) {
        Ok(_) => Ok(res),
        Err(e) => {
            println!("Error parsing interface attribute: {}", e);
            Err(SplProgramInterfaceError::ParseError)
        }
    }
}

/// Generate the pack and unpack implementations for the
/// instruction enum declared by the program
fn generate_pack_unpack(item_enum: &ItemEnum) -> TokenStream {
    // let ident = &item_enum.ident;
    // let (unpack_arms, pack_arms) = build_pack_unpack_arms(item_enum);
    // quote! {
    //     impl InterfaceInstructionPack for #ident {
    //         fn unpack(buf: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
    //             let (discrim, rest) = buf.split_at(8);
    //             if discrim.len() < 8 {
    //                 return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
    //             }
    //             match discrim {
    //                 #(#unpack_arms)*
    //                 _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData)
    //             }
    //         }
    //         fn pack<W: std::io::Write>(&self, writer: &mut W) -> Result<(), solana_program::program_error::ProgramError> {
    //             match self {
    //                 #(#pack_arms)*
    //             }
    //         }
    //     }
    // }
    quote! {}
}

/// Build the pack and unpack arms for the generated tokens
fn build_pack_unpack_arms(item_enum: &ItemEnum) -> (Vec<TokenStream>, Vec<TokenStream>) {
    // TODO
    (vec![quote! {}], vec![quote! {}])
}
