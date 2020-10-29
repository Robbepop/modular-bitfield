use crate::bitfield_attr::{
    AttributeArgs,
    Config,
};
use core::convert::TryFrom;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote_spanned,
    ToTokens,
};
use syn::{
    self,
    parse::Result,
    punctuated::Punctuated,
    spanned::Spanned as _,
    Token,
};

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
    let attrs = syn::parse::<AttributeArgs>(args.into())?;
    let config = Config::try_from(attrs)?;
    let bitfield = BitfieldStruct::try_from(input)?;
    Ok(bitfield.expand(&config))
}

/// Type used to guide analysis and expansion of `#[bitfield]` structs.
struct BitfieldStruct {
    /// The input `struct` item.
    item_struct: syn::ItemStruct,
}

/// Represents the `bitfield` specific attribute `#[bits = N]`.
struct BitsAttributeArgs {
    bits: syn::LitInt,
}

impl syn::parse::Parse for BitsAttributeArgs {
    fn parse(input: &syn::parse::ParseBuffer) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        Ok(BitsAttributeArgs {
            bits: input.parse()?,
        })
    }
}

impl TryFrom<syn::ItemStruct> for BitfieldStruct {
    type Error = syn::Error;

    fn try_from(item_struct: syn::ItemStruct) -> Result<Self> {
        Self::ensure_has_fields(&item_struct)?;
        Self::ensure_no_generics(&item_struct)?;
        Self::ensure_no_bits_markers(&item_struct)?;
        Ok(Self { item_struct })
    }
}

impl BitfieldStruct {
    /// Returns an error if the input struct does not have any fields.
    fn ensure_has_fields(item_struct: &syn::ItemStruct) -> Result<()> {
        if let unit @ syn::Fields::Unit = &item_struct.fields {
            return Err(format_err_spanned!(
                unit,
                "encountered invalid bitfield struct without fields"
            ))
        }
        Ok(())
    }

    /// Returns an error if the input struct is generic.
    fn ensure_no_generics(item_struct: &syn::ItemStruct) -> Result<()> {
        if !item_struct.generics.params.is_empty() {
            return Err(format_err_spanned!(
                item_struct,
                "encountered invalid generic bitfield struct"
            ))
        }
        Ok(())
    }

    /// Ensures that no field in the given input struct has a `#[bits]` marker attribute.
    fn ensure_no_bits_markers(item_struct: &syn::ItemStruct) -> Result<()> {
        for (n, field) in item_struct.fields.iter().enumerate() {
            for attr in field.attrs.iter() {
                if !attr.path.is_ident("bits") {
                    return Err(format_err_spanned!(
                        attr,
                        "encountered unsupported attribute `#[bits]` of field at {}",
                        n
                    ))
                }
            }
        }
        Ok(())
    }

    /// Expands the given `#[bitfield]` struct into an actual bitfield definition.
    pub fn expand(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let check_multiple_of_8 = self.generate_check_multiple_of_8(config);
        let struct_definition = self.generate_struct();
        let constructor_definition = self.generate_constructor();
        let specifier_impl = self.generate_specifier_impl(config);

        let byte_conversion_impls = self.expand_byte_conversion_impls();
        let getters_and_setters = self.expand_getters_and_setters();

        let debug_impl = self.generate_debug_impl();

        quote_spanned!(span=>
            #struct_definition
            #check_multiple_of_8
            #constructor_definition
            #byte_conversion_impls
            #getters_and_setters
            #specifier_impl
            #debug_impl
        )
    }

    /// Expands to the `Specifier` impl for the `#[bitfield]` struct if `specifier = true`.
    ///
    /// Otherwise returns `None`.
    pub fn generate_specifier_impl(&self, config: &Config) -> Option<TokenStream2> {
        if !config.specifier {
            return None
        }
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let bits = self.generate_bitfield_size();
        let next_divisible_by_8 = Self::next_divisible_by_8(&bits);
        Some(quote_spanned!(span =>
            #[allow(clippy::identity_op)]
            const _: () = {
                impl ::modular_bitfield::private::checks::CheckSpecifierHasAtMost128Bits for #ident {
                    type CheckType = [(); (#bits <= 128) as usize];
                }
            };

            #[allow(clippy::identity_op)]
            impl ::modular_bitfield::Specifier for #ident {
                const BITS: usize = #bits;

                type Bytes = <[(); if #next_divisible_by_8 > 128 { 128 } else #next_divisible_by_8] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
                type InOut = Self;

                #[inline]
                fn into_bytes(
                    value: Self::InOut,
                ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
                    ::core::result::Result::Ok(<Self::Bytes>::from_le_bytes(value.bytes))
                }

                #[inline]
                fn from_bytes(
                    bytes: Self::Bytes,
                ) -> ::core::result::Result<Self::InOut, ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>>
                {
                    if bytes > ((0x01 << Self::BITS) - 1) {
                        return ::core::result::Result::Err(::modular_bitfield::error::InvalidBitPattern::new(bytes))
                    }
                    ::core::result::Result::Ok(Self {
                        bytes: bytes.to_le_bytes(),
                    })
                }
            }
        ))
    }

    /// Get a list of fields, both in the form of a `String` as well as the corresponding faillible
    /// getter method
    fn get_field_strings_and_getters(&self) -> (Vec<String>, Vec<syn::Ident>) {
        self.item_struct
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_name = field
                    .ident
                    .as_ref()
                    .map(syn::Ident::to_string)
                    .unwrap_or_else(|| format!("get_{}", i));
                let fallible_getter =
                    quote::format_ident!("{}_or_err", field_name, span = field.span());

                (field_name, fallible_getter)
            })
            .unzip()
    }

    /// Generates the core::fmt::Debug impl if `#[derive(Debug)]` is included.
    pub fn generate_debug_impl(&self) -> Option<TokenStream2> {
        if self.should_derive_debug() {
            let span = self.item_struct.span();
            let ident = &self.item_struct.ident;
            let (field_strings, field_getters) = self.get_field_strings_and_getters();

            Some(quote_spanned!(span=>
                impl ::core::fmt::Debug for #ident {
                    fn fmt(&self, __bf_f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        __bf_f.debug_struct(::core::stringify!(#ident))
                            #(
                                .field(
                                    #field_strings,
                                    self.#field_getters()
                                        .as_ref()
                                        .map(|__bf_field| __bf_field as &dyn ::core::fmt::Debug)
                                        .unwrap_or_else(|__bf_err| __bf_err as &dyn ::core::fmt::Debug)
                                )
                            )*
                            .finish()
                    }
                }
            ))
        } else {
            None
        }
    }

    /// Generates the expression denoting the sum of all field bit specifier sizes.
    ///
    /// # Example
    ///
    /// For the following struct:
    ///
    /// ```
    /// # use modular_bitfield::prelude::*;
    /// #[bitfield]
    /// pub struct Color {
    ///     r: B8,
    ///     g: B8,
    ///     b: B8,
    ///     a: bool,
    ///     rest: B7,
    /// }
    /// ```
    ///
    /// We generate the following tokens:
    ///
    /// ```
    /// # use modular_bitfield::prelude::*;
    /// {
    ///     0usize +
    ///     <B8 as ::modular_bitfield::Specifier>::BITS +
    ///     <B8 as ::modular_bitfield::Specifier>::BITS +
    ///     <B8 as ::modular_bitfield::Specifier>::BITS +
    ///     <bool as ::modular_bitfield::Specifier>::BITS +
    ///     <B7 as ::modular_bitfield::Specifier>::BITS
    /// }
    /// # ;
    /// ```
    ///
    /// Which is a compile time evaluatable expression.
    fn generate_bitfield_size(&self) -> TokenStream2 {
        let span = self.item_struct.span();
        let sum = self
            .item_struct
            .fields
            .iter()
            .map(|field| {
                let span = field.span();
                let ty = &field.ty;
                quote_spanned!(span=>
                    <#ty as ::modular_bitfield::Specifier>::BITS
                )
            })
            .fold(quote_spanned!(span=> 0usize), |lhs, rhs| {
                quote_spanned!(span =>
                    #lhs + #rhs
                )
            });
        quote_spanned!(span=>
            { #sum }
        )
    }

    /// Generates the `CheckTotalSizeMultipleOf8` trait implementation to check for correct bitfield sizes.
    ///
    /// Won't generate a check if `#[bitfield(specifier = true)]` is set.
    fn generate_check_multiple_of_8(&self, config: &Config) -> Option<TokenStream2> {
        if config.specifier {
            return None
        }
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let size = self.generate_bitfield_size();
        Some(quote_spanned!(span=>
            #[allow(clippy::identity_op)]
            const _: () = {
                impl ::modular_bitfield::private::checks::CheckTotalSizeMultipleOf8 for #ident {
                    type Size = ::modular_bitfield::private::checks::TotalSize<[(); #size % 8usize]>;
                }
            };
        ))
    }

    /// Returns a token stream representing the next greater value divisible by 8.
    fn next_divisible_by_8(value: &TokenStream2) -> TokenStream2 {
        let span = value.span();
        quote_spanned!(span=> {
            (((#value - 1) / 8) + 1) * 8
        })
    }

    /// Generates the actual item struct definition for the `#[bitfield]`.
    ///
    /// Internally it only contains a byte array equal to the minimum required
    /// amount of bytes to compactly store the information of all its bit fields.
    fn generate_struct(&self) -> TokenStream2 {
        let span = self.item_struct.span();
        let attrs = self.get_attrs_no_debug();
        let vis = &self.item_struct.vis;
        let ident = &self.item_struct.ident;
        let size = self.generate_bitfield_size();
        let next_divisible_by_8 = Self::next_divisible_by_8(&size);
        quote_spanned!(span=>
            #( #attrs )*
            #[repr(transparent)]
            #[allow(clippy::identity_op, unused_attributes)]
            #vis struct #ident
            {
                bytes: [::core::primitive::u8; #next_divisible_by_8 / 8usize],
            }
        )
    }

    /// Get whether or not `derive(Debug)` is included in the attributes
    fn should_derive_debug(&self) -> bool {
        self.item_struct.attrs.iter().any(|attr| {
            match attr.parse_meta() {
                Ok(syn::Meta::List(meta_list)) => {
                    let is_derive = meta_list
                        .path
                        .get_ident()
                        .map(|ident| *ident == "derive")
                        .unwrap_or(false);

                    if is_derive {
                        meta_list.nested.into_iter().any(|nested| {
                            match nested {
                                syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                                    path.get_ident()
                                        .map(|ident| *ident == "Debug")
                                        .unwrap_or(false)
                                }
                                _ => false,
                            }
                        })
                    } else {
                        false
                    }
                }
                _ => false,
            }
        })
    }

    /// Get the attributes with `Debug` filtered out of any present derives.
    fn get_attrs_no_debug<'a>(&'a self) -> impl Iterator<Item = TokenStream2> + 'a {
        self.item_struct.attrs.iter().map(|attr| {
            match attr.parse_meta() {
                Ok(syn::Meta::List(mut meta_list)) => {
                    let is_derive = meta_list
                        .path
                        .get_ident()
                        .map(|ident| *ident == "derive")
                        .unwrap_or(false);

                    if is_derive {
                        meta_list.nested = meta_list
                            .nested
                            .into_iter()
                            .filter(|nested| {
                                match nested {
                                    syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                                        path.get_ident()
                                            .map(|ident| *ident != "Debug")
                                            .unwrap_or(true)
                                    }
                                    _ => true,
                                }
                            })
                            .collect();

                        let span = meta_list.span();
                        quote_spanned! {span=>
                            #[ #meta_list  ]
                        }
                    } else {
                        attr.to_token_stream()
                    }
                }
                _ => {
                    eprintln!("{}", attr.to_token_stream().to_string());
                    attr.to_token_stream()
                }
            }
        })
    }

    /// Generates the constructor for the bitfield that initializes all bytes to zero.
    fn generate_constructor(&self) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let size = self.generate_bitfield_size();
        let next_divisible_by_8 = Self::next_divisible_by_8(&size);
        quote_spanned!(span=>
            impl #ident
            {
                /// Returns an instance with zero initialized data.
                #[allow(clippy::identity_op)]
                pub const fn new() -> Self {
                    Self {
                        bytes: [0u8; #next_divisible_by_8 / 8usize],
                    }
                }
            }
        )
    }

    /// Generates routines to allow conversion from and to bytes for the `#[bitfield]` struct.
    fn expand_byte_conversion_impls(&self) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let size = self.generate_bitfield_size();
        let next_divisible_by_8 = Self::next_divisible_by_8(&size);
        quote_spanned!(span=>
            impl #ident {
                /// Returns the underlying bits.
                ///
                /// # Layout
                ///
                /// The returned byte array is layed out in the same way as described
                /// [here](https://docs.rs/modular-bitfield/#generated-structure).
                #[inline]
                #[allow(clippy::identity_op)]
                pub const fn as_bytes(&self) -> &[::core::primitive::u8; #next_divisible_by_8 / 8usize] {
                    &self.bytes
                }

                /// Converts the given bytes directly into the bitfield struct.
                #[inline]
                #[allow(clippy::identity_op)]
                pub const fn from_bytes(bytes: [::core::primitive::u8; #next_divisible_by_8 / 8usize]) -> Self {
                    Self { bytes }
                }
            }
        )
    }

    /// Generates code to check for the bit size arguments of bitfields.
    fn expand_getters_and_setters_checks_for_field(
        &self,
        field: &syn::Field,
    ) -> TokenStream2 {
        let span = field.span();
        let ty = &field.ty;
        let checks = field.attrs.iter().map(|attr| {
            let bits_arg = syn::parse::<BitsAttributeArgs>(attr.tokens.clone().into())
                .expect("encountered unexpected invalid bitfield attribute");
            let expected_bits = bits_arg.bits;
            quote_spanned!(expected_bits.span() =>
                let _: ::modular_bitfield::private::checks::BitsCheck::<[(); #expected_bits]> =
                    ::modular_bitfield::private::checks::BitsCheck::<[(); #expected_bits]>{
                        arr: [(); <#ty as ::modular_bitfield::Specifier>::BITS]
                    };
            )
        });
        quote_spanned!(span=>
            const _: () = {
                #( #checks )*
            };
        )
    }

    fn expand_getters_and_setters_for_field(
        &self,
        offset: &mut Punctuated<syn::Expr, syn::Token![+]>,
        n: usize,
        field: &syn::Field,
    ) -> TokenStream2 {
        let span = field.span();
        let struct_ident = &self.item_struct.ident;
        let ident_frag: &dyn quote::IdentFragment = match field.ident.as_ref() {
            Some(field) => field,
            None => &n,
        };
        let get_ident = field
            .ident
            .as_ref()
            .cloned()
            .unwrap_or_else(|| format_ident!("get_{}", n));
        let get_checked_ident = field
            .ident
            .as_ref()
            .map(|ident| format_ident!("{}_or_err", ident))
            .unwrap_or_else(|| format_ident!("get_{}_or_err", n));
        let set_ident = format_ident!("set_{}", ident_frag);
        let set_checked_ident = format_ident!("set_{}_checked", ident_frag);
        let with_ident = format_ident!("with_{}", ident_frag);
        let with_checked_ident = format_ident!("with_{}_checked", ident_frag);

        let doc_ident = field
            .ident
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or(format!("{}", n));
        let ty = &field.ty;
        let vis = &field.vis;
        let bits_checks = self.expand_getters_and_setters_checks_for_field(field);

        let get_assert_msg = format!(
            "value contains invalid bit pattern for field {}.{}",
            struct_ident, doc_ident
        );
        let set_assert_msg = format!(
            "value out of bounds for field {}.{}",
            struct_ident, doc_ident
        );
        let getter_docs = format!("Returns the value of {}.", doc_ident);
        let checked_getter_docs = format!(
            "Returns the value of {}.\n\n\
             #Errors\n\n\
             If the returned value contains an invalid bit pattern for {}.",
            doc_ident, doc_ident,
        );
        let setter_docs = format!(
            "Sets the value of {} to the given value.\n\n\
             #Panics\n\n\
             If the given value is out of bounds for {}.",
            doc_ident, doc_ident,
        );
        let checked_setter_docs = format!(
            "Sets the value of {} to the given value.\n\n\
             #Errors\n\n\
             If the given value is out of bounds for {}.",
            doc_ident, doc_ident,
        );
        let with_docs = format!(
            "Returns a copy of the bitfield with the value of {} \
             set to the given value.\n\n\
             #Panics\n\n\
             If the given value is out of bounds for {}.",
            doc_ident, doc_ident,
        );
        let checked_with_docs = format!(
            "Returns a copy of the bitfield with the value of {} \
             set to the given value.\n\n\
             #Errors\n\n\
             If the given value is out of bounds for {}.",
            doc_ident, doc_ident,
        );

        let expanded = quote_spanned!(span=>
            #[doc = #getter_docs]
            #[inline]
            #vis fn #get_ident(&self) -> <#ty as ::modular_bitfield::Specifier>::InOut {
                self.#get_checked_ident().expect(#get_assert_msg)
            }

            #[doc = #checked_getter_docs]
            #[inline]
            #[allow(dead_code)]
            #vis fn #get_checked_ident(
                &self,
            ) -> ::core::result::Result<
                <#ty as ::modular_bitfield::Specifier>::InOut,
                ::modular_bitfield::error::InvalidBitPattern<<#ty as ::modular_bitfield::Specifier>::Bytes>
            > {
                #bits_checks

                let __bf_read: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<#ty>(&self.bytes[..], #offset)
                };
                <#ty as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
            }

            #[doc = #with_docs]
            #[inline]
            #[allow(dead_code)]
            #vis fn #with_ident(
                mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut
            ) -> Self {
                self.#set_ident(new_val);
                self
            }

            #[doc = #checked_with_docs]
            #[inline]
            #[allow(dead_code)]
            #vis fn #with_checked_ident(
                mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut,
            ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
                self.#set_checked_ident(new_val)?;
                ::core::result::Result::Ok(self)
            }

            #[doc = #setter_docs]
            #[inline]
            #[allow(dead_code)]
            #vis fn #set_ident(&mut self, new_val: <#ty as ::modular_bitfield::Specifier>::InOut) {
                self.#set_checked_ident(new_val).expect(#set_assert_msg)
            }

            #[doc = #checked_setter_docs]
            #[inline]
            #vis fn #set_checked_ident(
                &mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut
            ) -> ::core::result::Result<(), ::modular_bitfield::error::OutOfBounds> {
                let __bf_base_bits: ::core::primitive::usize = 8usize * ::core::mem::size_of::<<#ty as ::modular_bitfield::Specifier>::Bytes>();
                let __bf_max_value: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    !0 >> (__bf_base_bits - <#ty as ::modular_bitfield::Specifier>::BITS)
                };
                let __bf_spec_bits: ::core::primitive::usize = <#ty as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    <#ty as ::modular_bitfield::Specifier>::into_bytes(new_val)
                }?;
                // We compare base bits with spec bits to drop this condition
                // if there cannot be invalid inputs.
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value) {
                    return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds)
                }
                ::modular_bitfield::private::write_specifier::<#ty>(&mut self.bytes[..], #offset, __bf_raw_val);
                ::core::result::Result::Ok(())
            }
        );
        offset.push(syn::parse_quote! { <#ty as ::modular_bitfield::Specifier>::BITS });
        expanded
    }

    fn expand_getters_and_setters(&self) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let mut offset = {
            let mut offset = Punctuated::<syn::Expr, Token![+]>::new();
            offset.push(syn::parse_quote! { 0usize });
            offset
        };
        let setters_and_getters =
            self.item_struct
                .fields
                .iter()
                .enumerate()
                .map(|(n, field)| {
                    self.expand_getters_and_setters_for_field(&mut offset, n, field)
                });
        quote_spanned!(span=>
            impl #ident {
                #( #setters_and_getters )*
            }
        )
    }
}
