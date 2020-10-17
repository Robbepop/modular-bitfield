use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned as _;

pub fn generate(input: TokenStream2) -> TokenStream2 {
    match generate_or_error(input) {
        Ok(output) => output,
        Err(err) => err.to_compile_error(),
    }
}

fn generate_or_error(input: TokenStream2) -> syn::Result<TokenStream2> {
    let input = syn::parse::<syn::DeriveInput>(input.into())?;
    match input.data {
        syn::Data::Enum(data_enum) => generate_enum(syn::ItemEnum {
            attrs: input.attrs,
            vis: input.vis,
            enum_token: data_enum.enum_token,
            ident: input.ident,
            generics: input.generics,
            brace_token: data_enum.brace_token,
            variants: data_enum.variants,
        }),
        syn::Data::Struct(_) => bail!(
            input.ident,
            "structs are not supported as bitfield specifiers",
        ),
        syn::Data::Union(_) => bail!(
            input.ident,
            "unions are not supported as bitfield specifiers",
        ),
    }
}

fn generate_enum(input: syn::ItemEnum) -> syn::Result<TokenStream2> {
    let enum_ident = &input.ident;
    let count_variants = input.variants.iter().count();
    if !count_variants.is_power_of_two() {
        return Err(syn::Error::new(
            Span2::call_site(),
            "BitfieldSpecifier expected a number of variants which is a power of 2",
        ));
    }
    // We can take `trailing_zeros` returns type as the required amount of bits.
    let bits = count_variants.trailing_zeros() as usize;

    let variants = input
        .variants
        .iter()
        .filter_map(|variant| match &variant.fields {
            syn::Fields::Unit => Some(&variant.ident),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut check_discriminants_tokens = quote! {};
    let mut from_bits_match_arms = quote! {};
    for variant in &variants {
        check_discriminants_tokens.extend(quote_spanned! { variant.span() =>
            impl ::modular_bitfield::private::checks::CheckDiscriminantInRange<[(); #enum_ident::#variant as usize]> for #enum_ident {
                type CheckType = [(); ((#enum_ident::#variant as usize) < (0x1 << #bits)) as usize ];
            }
        });
        use heck::SnakeCase as _;
        let snake_variant = &variant.to_string().to_snake_case();
        let snake_variant = match syn::parse_str::<syn::Ident>(snake_variant) {
            Ok(parsed_ident) => parsed_ident,
            // Use a raw identifier to allow strict keywords.
            Err(_) => format_ident!("r#{}", snake_variant),
        };
        from_bits_match_arms.extend(quote! {
            #snake_variant if #snake_variant == #enum_ident::#variant as <#enum_ident as ::modular_bitfield::Specifier>::Base => {
                #enum_ident::#variant
            }
        });
    }

    Ok(quote! {
        #check_discriminants_tokens

        impl ::modular_bitfield::Specifier for #enum_ident {
            const BITS: usize = #bits;
            type Base = <[(); #bits] as ::modular_bitfield::private::SpecifierBase>::Base;
            type Face = Self;
        }

        impl ::modular_bitfield::private::FromBits<<#enum_ident as ::modular_bitfield::Specifier>::Base> for #enum_ident {
            #[inline(always)]
            fn from_bits(bits: ::modular_bitfield::private::Bits<<#enum_ident as modular_bitfield::Specifier>::Base>) -> Self {
                match bits.into_raw() {
                    #from_bits_match_arms
                    // This API is only used internally and is only invoked on valid input.
                    // Thus it is find to omit error handling for cases where the incoming
                    // value is out of bounds to improve performance.
                    _ => unsafe { ::core::hint::unreachable_unchecked() },
                }
            }
        }

        impl modular_bitfield::private::IntoBits<<#enum_ident as ::modular_bitfield::Specifier>::Base> for #enum_ident {
            #[inline(always)]
            fn into_bits(self) -> ::modular_bitfield::private::Bits<<#enum_ident as ::modular_bitfield::Specifier>::Base> {
                ::modular_bitfield::private::Bits(
                    self as <#enum_ident as ::modular_bitfield::Specifier>::Base
                )
            }
        }
    })
}
