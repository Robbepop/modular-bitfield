use super::{
    config::{
        Config,
        ReprKind,
    },
    field_info::FieldInfo,
    BitfieldStruct,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
    quote_spanned,
};
use syn::{
    self,
    punctuated::Punctuated,
    spanned::Spanned as _,
    Token,
};

impl BitfieldStruct {
    /// Expands the given `#[bitfield]` structure into an actual bit-field definition.
    pub fn expand(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let check_filled = self.generate_check_for_filled(config);
        let struct_definition = self.generate_struct(config);
        let constructor_definition = self.generate_constructor(config);
        let specifier_impl = self.generate_specifier_impl(config);

        let byte_conversion_impls = self.expand_byte_conversion_impls(config);
        let getters_and_setters = self.expand_getters_and_setters(config);
        let bytes_check = self.expand_optional_bytes_check(config);
        let repr_impls_and_checks = self.expand_repr_from_impls_and_checks(config);
        let debug_impl = self.generate_debug_impl(config);

        quote_spanned!(span=>
            #struct_definition
            #check_filled
            #constructor_definition
            #byte_conversion_impls
            #getters_and_setters
            #specifier_impl
            #bytes_check
            #repr_impls_and_checks
            #debug_impl
        )
    }

    /// Generates code to check for the bit size arguments of bit-fields.
    fn expand_bits_checks_for_field(field_info: FieldInfo<'_>) -> TokenStream2 {
        let FieldInfo {
            index,
            field,
            config,
        } = field_info;
        let span = field.span();
        let bits_check = match &config.bits {
            Some(bits) => {
                let ty = &field.ty;
                let expected_bits = bits.value;
                let span = bits.span;
                let check_name = format_ident!("_{}_CHECK_EXPECTED_BITS", index);
                let check_name_inner =
                    format_ident!("_{}_check_expected_bits_inner", index);
                Some(quote_spanned!(span =>
                    const #check_name: () = {
                        let #check_name_inner: ::modular_bitfield::private::checks::BitsCheck::<[(); #expected_bits]> =
                            ::modular_bitfield::private::checks::BitsCheck::<[(); #expected_bits]>{
                                arr: [(); <#ty as ::modular_bitfield::Specifier>::BITS]
                            };
                    };
                ))
            }
            None => None,
        };

        quote_spanned!(span=>
            #bits_check
        )
    }

    /// Generates routines to allow conversion from and to bytes for the `#[bitfield]` structure.
    fn expand_byte_conversion_impls(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let size = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&size, false);
        let from_bytes = if config.filled_enabled() {
            quote_spanned!(span =>
                /// Converts the given bytes directly into the bit-field structure.
                #[inline]
                #[must_use]
                pub const fn from_bytes(bytes: [::core::primitive::u8; #next_divisible_by_8]) -> Self {
                    Self { bytes }
                }
            )
        } else {
            quote_spanned!(span=>
                /// Converts the given bytes directly into the bit-field structure.
                ///
                /// # Errors
                ///
                /// If the given bytes contain bits at positions that are undefined for `Self`.
                #[inline]
                pub fn from_bytes(
                    bytes: [::core::primitive::u8; #next_divisible_by_8]
                ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
                    if bytes[(#next_divisible_by_8) - 1] >= (0x01 << (8 - (((#next_divisible_by_8) * 8usize) - #size))) {
                        return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds)
                    }
                    ::core::result::Result::Ok(Self { bytes })
                }
            )
        };
        quote_spanned!(span =>
            #[automatically_derived]
            impl #ident {
                /// Returns the underlying bits.
                ///
                /// # Layout
                ///
                /// The returned byte array is laid out in the same way as described
                /// [here](https://docs.rs/modular-bitfield/#generated-structure).
                #[inline]
                #[must_use]
                pub const fn into_bytes(self) -> [::core::primitive::u8; #next_divisible_by_8] {
                    self.bytes
                }

                #from_bytes
            }
        )
    }

    fn expand_getters_and_setters(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let mut offset = Punctuated::<syn::Expr, Token![+]>::new();
        let bits_checks = self
            .field_infos(config)
            .map(Self::expand_bits_checks_for_field);

        let setters_and_getters = self.field_infos(config).map(|field_info| {
            self.expand_getters_and_setters_for_field(&mut offset, &field_info)
        });
        quote_spanned!(span =>
            #[automatically_derived]
            #[allow(clippy::no_effect_underscore_binding)]
            impl #ident {
                #( #bits_checks )*

                #( #setters_and_getters )*
            }
        )
    }

    fn expand_getters_and_setters_for_field(
        &self,
        offset: &mut Punctuated<syn::Expr, syn::Token![+]>,
        info: &FieldInfo<'_>,
    ) -> proc_macro2::TokenStream {
        let FieldInfo {
            index: _, field, ..
        } = &info;
        let span = field.span();
        let ty = &field.ty;
        let mut zero = Punctuated::<syn::Expr, Token![+]>::new();
        zero.push(syn::parse_quote! { 0usize });
        let getters = self.expand_getters_for_field(
            if offset.is_empty() { &mut zero } else { offset },
            info,
        );
        let setters = self.expand_setters_for_field(
            if offset.is_empty() { &mut zero } else { offset },
            info,
        );
        let getters_and_setters = quote_spanned!(span=>
            #getters
            #setters
        );
        offset.push(syn::parse_quote! { <#ty as ::modular_bitfield::Specifier>::BITS });
        getters_and_setters
    }

    fn expand_getters_for_field(
        &self,
        offset: &Punctuated<syn::Expr, syn::Token![+]>,
        info: &FieldInfo<'_>,
    ) -> Option<TokenStream2> {
        let FieldInfo {
            index: _,
            field,
            config,
        } = &info;
        if config.skip_getters() {
            return None
        }
        let struct_ident = &self.item_struct.ident;
        let span = field.span();
        let ident = info.ident_frag();
        let name = info.name();

        let retained_attrs = &config.retained_attrs;
        let get_ident = field
            .ident
            .as_ref()
            .cloned()
            .unwrap_or_else(|| format_ident!("get_{}", ident));
        let get_checked_ident = field.ident.as_ref().map_or_else(
            || format_ident!("get_{}_or_err", ident),
            |_| format_ident!("{}_or_err", ident),
        );
        let ty = &field.ty;
        let vis = &field.vis;
        let get_assert_msg = format!(
            "value contains invalid bit pattern for field {}.{}",
            struct_ident, name
        );

        let getter_docs = format!("Returns the value of {}.", name);
        let checked_getter_docs = format!(
            "Returns the value of {}.\n\n\
             #Errors\n\n\
             If the returned value contains an invalid bit pattern for {}.",
            name, name,
        );
        let getters = quote_spanned!(span=>
            #[doc = #getter_docs]
            #[inline]
            #[must_use]
            #( #retained_attrs )*
            #vis fn #get_ident(&self) -> <#ty as ::modular_bitfield::Specifier>::InOut {
                self.#get_checked_ident().expect(#get_assert_msg)
            }

            #[doc = #checked_getter_docs]
            #[inline]
            #( #retained_attrs )*
            #vis fn #get_checked_ident(
                &self,
            ) -> ::core::result::Result<
                <#ty as ::modular_bitfield::Specifier>::InOut,
                ::modular_bitfield::error::InvalidBitPattern<<#ty as ::modular_bitfield::Specifier>::Bytes>
            > {
                let __bf_read: <#ty as ::modular_bitfield::Specifier>::Bytes =
                    ::modular_bitfield::private::read_specifier::<#ty>(&self.bytes[..], #offset);
                <#ty as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
            }
        );
        Some(getters)
    }

    /// Generates the compile-time assertion if the optional `byte` parameter has been set.
    fn expand_optional_bytes_check(&self, config: &Config) -> Option<TokenStream2> {
        let ident = &self.item_struct.ident;
        config.bytes.as_ref().map(|config| {
            let bytes = config.value;
            let check_name = format_ident!(
                "_{}_CHECK_EXPECTED_BYTES",
                ident.to_string().to_uppercase()
            );
            quote_spanned!(config.span =>
                #[automatically_derived]
                #[allow(clippy::no_effect_underscore_binding)]
                impl #ident {
                    const #check_name: () = {
                        struct ExpectedBytes { __bf_unused: [u8; #bytes] }

                        ::modular_bitfield::private::static_assertions::assert_eq_size!(
                            ExpectedBytes,
                            #ident
                        );
                    };
                }
            )
        })
    }

    /// Generates `From` implementations for a `#[repr(uN)]` annotated #[bitfield] structure.
    fn expand_repr_from_impls_and_checks(&self, config: &Config) -> Option<TokenStream2> {
        let ident = &self.item_struct.ident;
        config.repr.as_ref().map(|repr| {
            let kind = &repr.value;
            let span = repr.span;
            let prim = match kind {
                ReprKind::U8 => quote! { ::core::primitive::u8 },
                ReprKind::U16 => quote! { ::core::primitive::u16 },
                ReprKind::U32 => quote! { ::core::primitive::u32 },
                ReprKind::U64 => quote! { ::core::primitive::u64 },
                ReprKind::U128 => quote! { ::core::primitive::u128 },
            };
            let actual_bits = self.generate_target_or_actual_bitfield_size(config);
            let trait_check_ident = match kind {
                ReprKind::U8 => quote! { IsU8Compatible },
                ReprKind::U16 => quote! { IsU16Compatible },
                ReprKind::U32 => quote! { IsU32Compatible },
                ReprKind::U64 => quote! { IsU64Compatible },
                ReprKind::U128 => quote! { IsU128Compatible },
            };
            quote_spanned!(span =>
                #[automatically_derived]
                impl ::core::convert::From<#prim> for #ident
                where
                    [(); #actual_bits]: ::modular_bitfield::private::#trait_check_ident,
                {
                    #[inline]
                    #[must_use]
                    fn from(__bf_prim: #prim) -> Self {
                        Self { bytes: <#prim>::to_le_bytes(__bf_prim) }
                    }
                }

                #[automatically_derived]
                impl ::core::convert::From<#ident> for #prim
                where
                    [(); #actual_bits]: ::modular_bitfield::private::#trait_check_ident,
                {
                    #[inline]
                    #[must_use]
                    fn from(__bf_bitfield: #ident) -> Self {
                        <Self>::from_le_bytes(__bf_bitfield.bytes)
                    }
                }
            )
        })
    }

    fn expand_setters_for_field(
        &self,
        offset: &Punctuated<syn::Expr, syn::Token![+]>,
        info: &FieldInfo<'_>,
    ) -> Option<TokenStream2> {
        let FieldInfo {
            index: _,
            field,
            config,
        } = &info;
        if config.skip_setters() {
            return None
        }
        let struct_ident = &self.item_struct.ident;
        let span = field.span();
        let retained_attrs = &config.retained_attrs;

        let ident = info.ident_frag();
        let name = info.name();
        let ty = &field.ty;
        let vis = &field.vis;

        let set_ident = format_ident!("set_{}", ident);
        let set_checked_ident = format_ident!("set_{}_checked", ident);
        let with_ident = format_ident!("with_{}", ident);
        let with_checked_ident = format_ident!("with_{}_checked", ident);

        let set_assert_msg =
            format!("value out of bounds for field {}.{}", struct_ident, name);
        let setter_docs = format!(
            "Sets the value of {} to the given value.\n\n\
             #Panics\n\n\
             If the given value is out of bounds for {}.",
            name, name,
        );
        let checked_setter_docs = format!(
            "Sets the value of {} to the given value.\n\n\
             #Errors\n\n\
             If the given value is out of bounds for {}.",
            name, name,
        );
        let with_docs = format!(
            "Returns a copy of the bitfield with the value of {} \
             set to the given value.\n\n\
             #Panics\n\n\
             If the given value is out of bounds for {}.",
            name, name,
        );
        let checked_with_docs = format!(
            "Returns a copy of the bitfield with the value of {} \
             set to the given value.\n\n\
             #Errors\n\n\
             If the given value is out of bounds for {}.",
            name, name,
        );
        let setters = quote_spanned!(span=>
            #[doc = #with_docs]
            #[inline]
            #[must_use]
            #( #retained_attrs )*
            #vis fn #with_ident(
                mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut
            ) -> Self {
                self.#set_ident(new_val);
                self
            }

            #[doc = #checked_with_docs]
            #[inline]
            #( #retained_attrs )*
            #vis fn #with_checked_ident(
                mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut,
            ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
                self.#set_checked_ident(new_val)?;
                ::core::result::Result::Ok(self)
            }

            #[doc = #setter_docs]
            #[inline]
            #( #retained_attrs )*
            #vis fn #set_ident(&mut self, new_val: <#ty as ::modular_bitfield::Specifier>::InOut) {
                self.#set_checked_ident(new_val).expect(#set_assert_msg)
            }

            #[doc = #checked_setter_docs]
            #[inline]
            #( #retained_attrs )*
            #vis fn #set_checked_ident(
                &mut self,
                new_val: <#ty as ::modular_bitfield::Specifier>::InOut
            ) -> ::core::result::Result<(), ::modular_bitfield::error::OutOfBounds> {
                let __bf_base_bits: ::core::primitive::usize = 8usize * ::core::mem::size_of::<<#ty as ::modular_bitfield::Specifier>::Bytes>();
                let __bf_max_value: <#ty as ::modular_bitfield::Specifier>::Bytes =
                    !0 >> (__bf_base_bits - <#ty as ::modular_bitfield::Specifier>::BITS);
                let __bf_spec_bits: ::core::primitive::usize = <#ty as ::modular_bitfield::Specifier>::BITS;
                let __bf_raw_val: <#ty as ::modular_bitfield::Specifier>::Bytes =
                    <#ty as ::modular_bitfield::Specifier>::into_bytes(new_val)?;
                // We compare base bits with spec bits to drop this condition
                // if there cannot be invalid inputs.
                if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value) {
                    return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds)
                }
                ::modular_bitfield::private::write_specifier::<#ty>(&mut self.bytes[..], #offset, __bf_raw_val);
                ::core::result::Result::Ok(())
            }
        );
        Some(setters)
    }

    /// Generates the expression denoting the sum of all field bit specifier sizes.
    ///
    /// # Example
    ///
    /// For the following structure:
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
    ///     <B8 as ::modular_bitfield::Specifier>::BITS +
    ///     <B8 as ::modular_bitfield::Specifier>::BITS +
    ///     <B8 as ::modular_bitfield::Specifier>::BITS +
    ///     <bool as ::modular_bitfield::Specifier>::BITS +
    ///     <B7 as ::modular_bitfield::Specifier>::BITS
    /// }
    /// # ;
    /// ```
    ///
    /// Which is a compile time evaluable expression.
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
            .fold(TokenStream2::new(), |lhs, rhs| {
                if lhs.is_empty() {
                    rhs
                } else {
                    quote_spanned!(span =>
                        #lhs + #rhs
                    )
                }
            });
        quote_spanned!(span=>
            { #sum }
        )
    }

    /// Generate check for either of the following two cases:
    ///
    /// - `filled = true`: Check if the total number of required bits is
    ///         - ... the same as `N` if `bits = N` was provided or
    ///         - ... a multiple of 8, otherwise
    /// - `filled = false`: Check if the total number of required bits is
    ///         - ... smaller than `N` if `bits = N` was provided or
    ///         - ... NOT a multiple of 8, otherwise
    fn generate_check_for_filled(&self, config: &Config) -> TokenStream2 {
        match config.bits.as_ref() {
            Some(bits_config) => {
                self.generate_filled_check_for_unaligned_bits(config, bits_config.value)
            }
            None => self.generate_filled_check_for_aligned_bits(config),
        }
    }

    /// Generates the constructor for the bit-field that initializes all bytes to zero.
    fn generate_constructor(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let size = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&size, false);
        quote_spanned!(span=>
            #[automatically_derived]
            impl #ident {
                /// Returns an instance with zero initialized data.
                #[must_use]
                pub const fn new() -> Self {
                    Self {
                        bytes: [0u8; #next_divisible_by_8],
                    }
                }
            }
        )
    }

    /// Generates the `core::fmt::Debug` implementation if `#[derive(Debug)]` is included.
    pub fn generate_debug_impl(&self, config: &Config) -> Option<TokenStream2> {
        config.derive_debug.as_ref()?;
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let fields = self.field_infos(config).map(|info| {
            let FieldInfo {
                index: _,
                field,
                config,
            } = &info;
            if config.skip_getters() {
                return None
            }
            let field_span = field.span();
            let field_name = info.name();
            let field_ident = info.ident_frag();
            let field_getter = field.ident.as_ref().map_or_else(
                || format_ident!("get_{}_or_err", field_ident),
                |_| format_ident!("{}_or_err", field_ident),
            );
            Some(quote_spanned!(field_span =>
                .field(
                    #field_name,
                    self.#field_getter()
                        .as_ref()
                        .map_or_else(
                            |__bf_err| __bf_err as &dyn (::core::fmt::Debug),
                            |__bf_field| __bf_field as &dyn (::core::fmt::Debug),
                        )
                )
            ))
        });
        Some(quote_spanned!(span =>
            #[automatically_derived]
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, __bf_f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    __bf_f.debug_struct(::core::stringify!(#ident))
                        #( #fields )*
                        .finish()
                }
            }
        ))
    }

    /// Generates a check in case `bits = N` is unset to verify that the actual amount of bits is either
    ///
    /// - ... divisible by 8, if `filled = true` or
    /// - ... not divisible by 8, if `filled = false`
    fn generate_filled_check_for_aligned_bits(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let actual_bits = self.generate_bitfield_size();
        let check_ident = if config.filled_enabled() {
            quote_spanned!(span => CheckTotalSizeMultipleOf8)
        } else {
            quote_spanned!(span => CheckTotalSizeIsNotMultipleOf8)
        };

        quote_spanned!(span =>
            #[automatically_derived]
            #[allow(clippy::no_effect_underscore_binding)]
            impl ::modular_bitfield::private::checks::#check_ident for #ident {
                type Size = ::modular_bitfield::private::checks::TotalSize<[(); #actual_bits % 8usize]>;
            }
        )
    }

    /// Generates a check in case `bits = N` is unset to verify that the actual amount of bits is either
    ///
    /// - ... equal to `N`, if `filled = true` or
    /// - ... smaller than `N`, if `filled = false`
    fn generate_filled_check_for_unaligned_bits(
        &self,
        config: &Config,
        required_bits: usize,
    ) -> TokenStream2 {
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let actual_bits = self.generate_bitfield_size();
        let check_ident = if config.filled_enabled() {
            quote_spanned!(span => CheckFillsUnalignedBits)
        } else {
            quote_spanned!(span => CheckDoesNotFillUnalignedBits)
        };
        let comparator = if config.filled_enabled() {
            quote! { == }
        } else {
            quote! { > }
        };

        quote_spanned!(span =>
            #[automatically_derived]
            #[allow(clippy::no_effect_underscore_binding)]
            impl ::modular_bitfield::private::checks::#check_ident for #ident {
                #[allow(clippy::cast_lossless)]
                type CheckType = [(); (#required_bits #comparator #actual_bits) as usize];
            }
        )
    }

    /// Expands to the `Specifier` implementation for the `#[bitfield]` structure if the
    /// `#[derive(BitfieldSpecifier)]` attribute is applied to it as well.
    ///
    /// Otherwise returns `None`.
    pub fn generate_specifier_impl(&self, config: &Config) -> Option<TokenStream2> {
        config.derive_specifier.as_ref()?;
        let span = self.item_struct.span();
        let ident = &self.item_struct.ident;
        let bits = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&bits, true);

        Some(quote_spanned!(span =>
            #[automatically_derived]
            impl ::modular_bitfield::private::checks::CheckSpecifierHasAtMost128Bits for #ident {
                #[allow(clippy::cast_lossless)]
                type CheckType = [(); (#bits <= 128) as usize];
            }

            #[automatically_derived]
            impl ::modular_bitfield::Specifier for #ident {
                const BITS: usize = #bits;

                type Bytes = <[(); if #bits > 128 { 128 } else { #bits }] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
                type InOut = Self;

                #[inline]
                fn into_bytes(
                    value: Self::InOut,
                ) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
                    ::core::result::Result::Ok(
                        <[(); #next_divisible_by_8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(
                            value.bytes
                        )
                    )
                }

                #[inline]
                fn from_bytes(
                    bytes: Self::Bytes,
                ) -> ::core::result::Result<Self::InOut, ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>>
                {
                    let __bf_max_value: Self::Bytes = (0x01 as Self::Bytes)
                        .checked_shl(Self::BITS as ::core::primitive::u32)
                        .unwrap_or(<Self::Bytes>::MAX);
                    if bytes > __bf_max_value {
                        return ::core::result::Result::Err(::modular_bitfield::error::InvalidBitPattern::new(bytes))
                    }
                    let __bf_bytes = bytes.to_le_bytes();
                    ::core::result::Result::Ok(Self {
                        bytes: <[(); #next_divisible_by_8] as ::modular_bitfield::private::ArrayBytesConversion>::bytes_into_array(bytes)
                    })
                }
            }
        ))
    }

    /// Generates the actual item structure definition for the `#[bitfield]`.
    ///
    /// Internally it only contains a byte array equal to the minimum required
    /// amount of bytes to compactly store the information of all its bit fields.
    fn generate_struct(&self, config: &Config) -> TokenStream2 {
        let span = self.item_struct.span();
        let attrs = &config.retained_attributes;
        let vis = &self.item_struct.vis;
        let ident = &self.item_struct.ident;
        let size = self.generate_target_or_actual_bitfield_size(config);
        let next_divisible_by_8 = Self::next_divisible_by_8(&size, false);
        quote_spanned!(span=>
            #( #attrs )*
            #vis struct #ident
            {
                bytes: [::core::primitive::u8; #next_divisible_by_8],
            }
        )
    }

    /// Generates the expression denoting the actual configured or implied bit width.
    fn generate_target_or_actual_bitfield_size(&self, config: &Config) -> TokenStream2 {
        config.bits.as_ref().map_or_else(
            || self.generate_bitfield_size(),
            |bits_config| {
                let span = bits_config.span;
                let value = bits_config.value;
                quote_spanned!(span=>
                    #value
                )
            },
        )
    }

    /// Returns a token stream representing the next greater value divisible by 8.
    fn next_divisible_by_8(value: &TokenStream2, times_8: bool) -> TokenStream2 {
        let span = value.span();
        let expr = quote_spanned!(span => (((#value - 1) / 8) + 1));
        if times_8 {
            quote_spanned!(span => {
                #expr * 8
            })
        } else {
            quote_spanned!(span => {
                #expr
            })
        }
    }
}
