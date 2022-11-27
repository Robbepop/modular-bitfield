mod analyse;
mod config;
mod expand;
mod field_config;
mod field_info;
mod params;

use self::{
    config::Config,
    params::ParamArgs,
};
use core::convert::TryFrom;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    self,
    parse::Result,
};

pub use config::Endian;

/// Analyzes the given token stream for `#[bitfield]` properties and expands code if valid.
pub fn analyse_and_expand(args: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match analyse_and_expand_or_error(args, input) {
        Ok(output) => output,
        Err(err) => err.to_compile_error(),
    }
}

/// Analyzes the given token stream for `#[bitfield]` properties and expands code if valid.
///
/// # Errors
///
/// If the given token stream does not yield a valid `#[bitfield]` specifier.
fn analyse_and_expand_or_error(
    args: TokenStream2,
    input: TokenStream2,
) -> Result<TokenStream2> {
    let input = syn::parse::<syn::ItemStruct>(input.into())?;
    let params = syn::parse::<ParamArgs>(args.into())?;
    let mut config = Config::default();
    config.feed_params(params)?;
    let bitfield = BitfieldStruct::try_from((&mut config, input))?;
    Ok(bitfield.expand(&config))
}

/// Type used to guide analysis and expansion of `#[bitfield]` structs.
struct BitfieldStruct {
    /// The input `struct` item.
    item_struct: syn::ItemStruct,
}
