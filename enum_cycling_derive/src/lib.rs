//! # Enum Cycling
//!
//! Enum Cycling is a crate that allows one to
//! more easily navigate enums in Rust.
#![allow(clippy::pedantic)]

mod enum_cycle;

use syn::DeriveInput;

/// auto-derives `EnumCycle` for the enum. Each variant
/// of the enum will move to the one above / below itself.
/// Variants of the enum can be skipped using `#[skip]`.
///
/// When a variant moves to one that can contain a value,
/// the default value for that type will be used for the values
/// of the returned enum variant.
///
/// Calling up / down on a variant that was skipped will result
/// in a panic.

#[proc_macro_derive(EnumCycle, attributes(skip, cycle))]
pub fn derive_enum_cycle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let tokens = enum_cycle::enum_cycle_inner(&input).unwrap_or_else(|err| err.to_compile_error());

    tokens.into()
}
