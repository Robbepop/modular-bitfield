use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use syn::{punctuated::Punctuated, spanned::Spanned as _};

use crate::bitfield_specifier;

pub(super) fn generate_adt(input: syn::ItemEnum) -> syn::Result<TokenStream2> {
    let span = input.span();
    let enum_ident = &input.ident;

    let maybe_tag_type = input
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("tag"))
        .fold(Ok(None), |acc, attr| {
            if acc?.is_some() {
                return Err(format_err_spanned!(
                    attr,
                    "More than one 'tag' attribute is not permitted",
                ));
            }
            Ok(Some(attr.parse_args::<syn::Type>()?))
        })?;

    let (tag, tag_code) = match maybe_tag_type {
        Some(ty) => (quote_spanned!(span=> #ty), quote_spanned!(span=> )),
        None => {
            let tag = syn::Ident::new(
                &format!("__Bf_{}_Tag", input.ident),
                proc_macro2::Span::mixed_site(),
            );
            let mut tag_enum = input.clone();
            tag_enum.vis = syn::Visibility::Inherited;
            tag_enum.ident = tag.clone();
            tag_enum
                .variants
                .iter_mut()
                .for_each(|var| var.fields = syn::Fields::Unit);
            let tag_specifier = bitfield_specifier::generate_enum(tag_enum.clone())?;
            // Attributes like "bits" are only used by generate_enum
            tag_enum.attrs.clear();
            (
                quote_spanned!(span=> #tag),
                quote_spanned!(span=> #tag_enum #tag_specifier),
            )
        }
    };

    let tag_bits = quote_spanned!(span=>
        <#tag as ::modular_bitfield::Specifier>::BITS
    );
    let bits = input
        .variants
        .iter()
        .filter_map(|variant| match &variant.fields {
            syn::Fields::Named(fs) => Some(fs.named.iter()),
            syn::Fields::Unnamed(fs) => Some(fs.unnamed.iter()),
            syn::Fields::Unit => None,
        })
        .map(|fields| {
            fields
                .map(|field| {
                    let span = field.span();
                    let ty = &field.ty;
                    quote_spanned!(span=>
                        <#ty as ::modular_bitfield::Specifier>::BITS
                    )
                })
                .fold(quote_spanned!(span=> #tag_bits), |lhs, rhs| {
                    quote_spanned!(span =>
                        #lhs + #rhs
                    )
                })
        })
        .reduce(|arm1, arm2| {
            quote_spanned!(span=>
                ::modular_bitfield::private::const_max(#arm1, #arm2)
            )
        });

    let next_divisible_by_8 = quote_spanned!(span=> (((#bits - 1) / 8) + 1) * 8);

    let mut offset = Punctuated::<syn::Expr, syn::Token![+]>::new();
    offset.push(syn::parse::<syn::Expr>(tag_bits.into())?);

    let from_bytes_arms = input.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let span = variant.span();

        let mut offset = offset.clone();
        let data_from_bytes = variant.fields.iter().enumerate().map(|(i, field)| {
            let ty = &field.ty;
            let temp = temp_id(i);
            let tokens = quote_spanned! {span=>
                let __bf_read: <#ty as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<#ty>(&array[..], #offset)
                };
                let #temp = <#ty as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
                    .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(bytes))?;
            };
            offset.push(syn::parse_quote! { <#ty as ::modular_bitfield::Specifier>::BITS });
            tokens
        });

        let construct = constructor(variant);
        quote_spanned!(span=>
            #tag::#ident => {
                #( #data_from_bytes )*
                Ok(#enum_ident::#construct)
            }
        )
    });

    let into_bytes_arms = input.variants.iter().map(|variant| {
        let span = variant.span();
        let ident = &variant.ident;
        let construct = constructor(variant);

        let write_tag = check_bounds_and_write(
            span,
            quote_spanned!(span=> #tag),
            quote_spanned!(span=> #tag::#ident),
            quote_spanned!(span=> 0usize),
        );

        let mut offset = offset.clone();
        let data_to_bytes = variant.fields.iter().enumerate().map(|(i, field)| {
            let ty = &field.ty;
            let temp = temp_id(i);
            let tokens = check_bounds_and_write(
                span,
                quote_spanned!(span=> #ty),
                quote_spanned!(span=> #temp),
                quote_spanned!(span=> #offset),
            );
            offset
                .push(syn::parse_quote! { <#ty as ::modular_bitfield::Specifier>::BITS });
            tokens
        });

        quote_spanned! {span=> #enum_ident::#construct => {
            let mut __bf_bytes = [0u8; #next_divisible_by_8 / 8usize];
            #write_tag
            #( #data_to_bytes )*
            ::core::result::Result::Ok(
                <[(); #next_divisible_by_8] as ::modular_bitfield::private::ArrayBytesConversion>::array_into_bytes(__bf_bytes)
            )
        }}
    });

    Ok(quote_spanned!(span=>
        #tag_code
        #[allow(clippy::identity_op)]
        const _: () = {
            impl ::modular_bitfield::private::checks::CheckSpecifierHasAtMost128Bits for #enum_ident {
                type CheckType = [(); (#bits <= 128) as ::core::primitive::usize];
            }
        };
        impl ::modular_bitfield::Specifier for #enum_ident {
            const BITS: usize = #bits;
            type Bytes = <[(); #bits] as ::modular_bitfield::private::SpecifierBytes>::Bytes;
            type InOut = Self;

            #[inline]
            fn into_bytes(input: Self::InOut) -> ::core::result::Result<Self::Bytes, ::modular_bitfield::error::OutOfBounds> {
                match input {
                    #( #into_bytes_arms ),*
                }
            }

            #[inline]
            fn from_bytes(bytes: Self::Bytes) -> ::core::result::Result<Self::InOut, ::modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
                let array = <[(); #next_divisible_by_8] as ::modular_bitfield::private::ArrayBytesConversion>::bytes_into_array(bytes);
                let __tag_read: <#tag as ::modular_bitfield::Specifier>::Bytes = {
                    ::modular_bitfield::private::read_specifier::<#tag>(&array[..], 0)
                };
                let tag = <#tag as ::modular_bitfield::Specifier>::from_bytes(__tag_read)
                    .map_err(|_| ::modular_bitfield::error::InvalidBitPattern::new(bytes))?;
                match tag {
                    #( #from_bytes_arms ),*
                    #[allow(unreachable_patterns)]
                    _ => ::core::result::Result::Err(
                        ::modular_bitfield::error::InvalidBitPattern::new(bytes))
                }
            }
        }
    ))
}
fn temp_id(i: usize) -> syn::Ident {
    syn::Ident::new(&format!("__bf_temp_{}", i), proc_macro2::Span::mixed_site())
}
fn constructor(variant: &syn::Variant) -> TokenStream2 {
    let span = variant.span();
    let ident = &variant.ident;
    let args = variant.fields.iter().enumerate().map(|(i, field)| {
        let temp = temp_id(i);
        match &field.ident {
            None => quote_spanned!(span=> #temp),
            Some(id) => quote_spanned!(span=> #id: #temp),
        }
    });
    match variant.fields {
        syn::Fields::Named(_) => quote_spanned!(span=> #ident{#( #args ),*}),
        syn::Fields::Unnamed(_) => quote_spanned!(span=> #ident(#( #args ),*)),
        syn::Fields::Unit => quote_spanned!(span=> #ident),
    }
}

fn check_bounds_and_write(
    span: proc_macro2::Span,
    ty: TokenStream2,
    value: TokenStream2,
    offset: TokenStream2,
) -> TokenStream2 {
    quote_spanned! {span=>
        let __bf_base_bits: ::core::primitive::usize = 8usize * ::core::mem::size_of::<<#ty as ::modular_bitfield::Specifier>::Bytes>();
        let __bf_max_value: <#ty as ::modular_bitfield::Specifier>::Bytes = {
            !0 >> (__bf_base_bits - <#ty as ::modular_bitfield::Specifier>::BITS)
        };
        let __bf_spec_bits: ::core::primitive::usize = <#ty as ::modular_bitfield::Specifier>::BITS;
        let __bf_raw_val: <#ty as ::modular_bitfield::Specifier>::Bytes = {
            <#ty as ::modular_bitfield::Specifier>::into_bytes(#value)
        }?;
        // We compare base bits with spec bits to drop this condition
        // if there cannot be invalid inputs.
        if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value) {
            return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds)
        }
        ::modular_bitfield::private::write_specifier::<#ty>(
            &mut __bf_bytes, #offset, __bf_raw_val);
    }
}
