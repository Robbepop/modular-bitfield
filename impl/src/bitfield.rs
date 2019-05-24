use proc_macro2::TokenStream as TokenStream2;
use syn::{
    self,
    Token,
    punctuated::Punctuated,
    parse::{
        Parse,
        ParseStream,
        Result,
    }
};
use quote::{
    quote,
    quote_spanned,
};

pub fn generate(args: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match bitfield_impl(args.into(), input.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn bitfield_impl(args: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let _ = args;
    let input = syn::parse::<BitfieldStruct>(input.into())?;
    input.validate()?;
    input.expand()
}

struct BitfieldStruct {
    ast: syn::ItemStruct,
}

/// Represents the `bitfield` specific attribute `#[bits = N]`.
struct BitsAttributeArgs {
    size: syn::LitInt,
}

impl syn::parse::Parse for BitsAttributeArgs {
    fn parse(input: &syn::parse::ParseBuffer) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        Ok(BitsAttributeArgs {
            size: input.parse()?,
        })
    }
}

impl Parse for BitfieldStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ast: input.parse()?,
        })
    }
}

impl BitfieldStruct {
    fn expand(&self) -> Result<TokenStream2> {
        if let unit@syn::Fields::Unit = &self.ast.fields {
            bail!(
                unit,
                "unit structs are not supported",
            )
        }
        let size = {
            let mut size = Punctuated::<syn::ExprPath, Token![+]>::new();
            for field in self.ast.fields.iter() {
                let ty = &field.ty;
                size.push(syn::parse_quote!( <#ty as Specifier>::BITS ));
            }
            size
        };
        let mut expanded = quote! {};
        let attrs = &self.ast.attrs;
        let internal_methods = self.expand_internal_methods()?;
        let getters_and_setters = self.expand_getters_and_setters()?;
        let ident = &self.ast.ident;
        expanded.extend(quote!{
            #(#attrs)*
            #[repr(C)]
            pub struct #ident
            {
                data: [u8; (#size) / 8],
            }

            impl modular_bitfield::checks::CheckTotalSizeMultipleOf8 for #ident {
                type Size = modular_bitfield::checks::TotalSize<[(); (#size) % 8]>;
            }

            impl #ident
            {
                pub fn new() -> Self {
                    Self {
                        data: [0; (#size) / 8],
                    }
                }

                #internal_methods
                #getters_and_setters
            }
        });
        Ok(expanded)
    }

    fn expand_internal_methods(&self) -> Result<TokenStream2> {
        Ok(quote! {
            #[inline(always)]
            fn get<T>(&self, start: usize) -> <T as modular_bitfield::Specifier>::Base
            where
                T: modular_bitfield::Specifier,
            {
                let end = start + <T as modular_bitfield::Specifier>::BITS;
                let ls_byte = start / 8; // compile-time
                let ms_byte = (end - 1) / 8; // compile-time
                let lsb_offset = start % 8; // compile-time
                let msb_offset = end % 8; // compile-time
                let msb_offset = if msb_offset == 0 { 8 } else { msb_offset };

                let mut buffer = <T as modular_bitfield::Specifier>::Base::default();

                if lsb_offset == 0 && msb_offset == 0 {
                    // Edge-case for whole bytes manipulation.
                    for byte in self.data[ls_byte..(ms_byte + 1)].iter().rev() {
                        buffer.push_bits(8, *byte)
                    }
                } else {
                    if ls_byte != ms_byte {
                        // Most-significant byte
                        buffer.push_bits(msb_offset as u32, self.data[ms_byte]);
                    }

                    if ms_byte - ls_byte >= 2 {
                        // Middle bytes
                        for byte in self.data[(ls_byte + 1)..ms_byte].iter().rev() {
                            buffer.push_bits(8, *byte);
                        }
                    }

                    if ls_byte == ms_byte {
                        buffer.push_bits(<T as modular_bitfield::Specifier>::BITS as u32, self.data[ls_byte] >> lsb_offset);
                    } else {
                        buffer.push_bits(8 - lsb_offset as u32, self.data[ls_byte] >> lsb_offset);
                    }
                }

                buffer
            }

            #[inline(always)]
            fn set<T>(&mut self, start: usize, new_val: <T as modular_bitfield::Specifier>::Base)
            where
                T: modular_bitfield::Specifier,
            {
                let end = start + <T as modular_bitfield::Specifier>::BITS;
                let ls_byte = start / 8; // compile-time
                let ms_byte = (end - 1) / 8; // compile-time
                let lsb_offset = start % 8; // compile-time
                let msb_offset = end % 8; // compile-time
                let msb_offset = if msb_offset == 0 { 8 } else { msb_offset };

                let mut input = new_val;

                if lsb_offset == 0 && msb_offset == 0 {
                    // Edge-case for whole bytes manipulation.
                    for byte in self.data[ls_byte..(ms_byte + 1)].iter_mut() {
                        *byte = input.pop_bits(8);
                    }
                } else {
                    // Least-significant byte
                    let stays_same = self.data[ls_byte] & ((0x1 << lsb_offset as u32) - 1);
                    let overwrite = input.pop_bits(8 - lsb_offset as u32);
                    self.data[ls_byte] = stays_same | (overwrite << lsb_offset as u32);

                    if ms_byte - ls_byte >= 2 {
                        // Middle bytes
                        for byte in self.data[(ls_byte + 1)..ms_byte].iter_mut() {
                            *byte = input.pop_bits(8);
                        }
                    }

                    if ls_byte != ms_byte {
                        // Most-significant byte
                        if msb_offset == 8 {
                            // We don't need to respect what was formerly stored in the byte.
                            self.data[ms_byte] = input.pop_bits(msb_offset as u32);
                        } else {
                            // All bits that do not belong to this field should be preserved.
                            let stays_same = self.data[ms_byte] & !((0x1 << msb_offset) - 1);
                            let overwrite = input.pop_bits(msb_offset as u32);
                            self.data[ms_byte] = stays_same | overwrite;
                        }
                    }
                }
            }
        })
    }

    fn expand_getters_and_setters(&self) -> Result<TokenStream2> {
        let mut expanded = quote! {};
        let mut offset = Punctuated::<syn::Expr, Token![+]>::new();
        offset.push(syn::parse_quote!{ 0 });
        for (n, field) in self.ast.fields.iter().enumerate() {
            use crate::ident_ext::IdentExt as _;
            let field_name = field.ident
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or(format!("{}", n));
            let getter_name = syn::Ident::from_str(format!("get_{}", field_name));
            let setter_name = syn::Ident::from_str(format!("set_{}", field_name));
            let checked_setter_name = syn::Ident::from_str(format!("set_{}_checked", field_name));
            let field_type = &field.ty;

            let mut bits_check_tokens = quote! {};
            for attr in field.attrs.iter().filter(|attr| attr.path.is_ident("bits")) {
                let bits_arg = syn::parse::<BitsAttributeArgs>(attr.tts.clone().into()).unwrap();
                let expected_bits = bits_arg.size;
                bits_check_tokens.extend(quote_spanned! { expected_bits.span() =>
                    let _ = modular_bitfield::checks::BitsCheck::<
                        [(); #expected_bits]
                    >{
                        arr: [(); <#field_type as modular_bitfield::Specifier>::BITS]
                    };
                })
            }

            let set_assert_msg = proc_macro2::Literal::string(
                &format!("value out of bounds for field {}.{}", self.ast.ident, field_name)
            );

            let getter_docs = format!("Returns the value of {}.", field_name);
            let setter_docs = format!(
                "Sets the value of {} to the given value.\n\n\
                 #Panics\n\n\
                 If the given value is out of bounds for {}",
                 field_name,
                 field_name,
            );
            let checked_setter_docs = format!(
                "Sets the value of {} to the given value.\n\n\
                 #Errors\n\n\
                 If the given value is out of bounds for {}",
                 field_name,
                 field_name,
            );

            expanded.extend(quote!{
                #[doc = #getter_docs]
                #[inline]
                pub fn #getter_name(&self) -> <#field_type as modular_bitfield::Specifier>::Face {
                    #bits_check_tokens

                    <#field_type as modular_bitfield::Specifier>::Face::from_bits(
                        modular_bitfield::Bits(self.get::<#field_type>(#offset))
                    )
                }

                #[doc = #setter_docs]
                #[inline]
                pub fn #setter_name(&mut self, new_val: <#field_type as modular_bitfield::Specifier>::Face) {
                    self.#checked_setter_name(new_val).expect(#set_assert_msg)
                }

                #[doc = #checked_setter_docs]
                #[inline]
                pub fn #checked_setter_name(
                    &mut self,
                    new_val: <#field_type as modular_bitfield::Specifier>::Face
                ) -> Result<(), modular_bitfield::Error> {
                    use ::core::mem::size_of;
                    let base_bits = 8 * size_of::<<#field_type as modular_bitfield::Specifier>::Base>();
                    let max_value: <#field_type as modular_bitfield::Specifier>::Base = {
                        !0 >> (base_bits - <#field_type as modular_bitfield::Specifier>::BITS)
                    };
                    let spec_bits = <#field_type as modular_bitfield::Specifier>::BITS;
                    let raw_val = new_val.into_bits().into_raw();
                    // We compare base bits with spec bits to drop this condition
                    // if there cannot be invalid inputs.
                    if !(base_bits == spec_bits || raw_val <= max_value) {
                        return Err(modular_bitfield::Error::OutOfBounds)
                    }
                    self.set::<#field_type>(#offset, raw_val);
                    Ok(())
                }
            });
            offset.push(syn::parse_quote!{ <#field_type as modular_bitfield::Specifier>::BITS });
        }
        Ok(expanded)
    }

    pub fn has_generics(&self) -> bool {
        // The `lt_token` and `gt_token` don't constitute generics on their own.
        // Rustc accepts this as a struct without generics:
        //
        // ```
        // struct S<> {
        //     ...
        // }
        // ```
        //
        // So we have to check whether the params are actually empty.
        !self.ast.generics.params.is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        if self.has_generics() {
            bail!(
                self.ast.generics,
                "generics are not supported for bitfields",
            )
        }
        if let unit@syn::Fields::Unit = &self.ast.fields {
            bail!(
                unit,
                "unit structs are not supported",
            )
        }
        Ok(())
    }
}
