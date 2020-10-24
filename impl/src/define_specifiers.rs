use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};

pub fn generate(_input: TokenStream2) -> TokenStream2 {
    let mut tokens = quote! {};
    for n in 1usize..=128 {
        let t_origin = match n {
            1..=8 => quote! { ::core::primitive::u8 },
            9..=16 => quote! { ::core::primitive::u16 },
            17..=32 => quote! { ::core::primitive::u32 },
            33..=64 => quote! { ::core::primitive::u64 },
            65..=128 => quote! { ::core::primitive::u128 },
            _ => unreachable!(),
        };
        let ident = format_ident!("B{}", n);
        let doc_comment = format!("Specifier for {} bits.", n);
        let max_value = match n {
            // 1..=7 => quote! { ::core::primitive::u8::MAX >> (8 - n) },
            // 8 => quote! { ::core::primitive::u8::MAX },
            // 9..=15 => quote! { ::core::primitive::u16::MAX >> (16 - n) },
            // 16 => quote! { ::core::primitive::u16::MAX },
            // 17..=31 => quote! { ::core::primitive::u32::MAX >> (32 - n) },
            // 32 => quote! { ::core::primitive::u32::MAX },
            // 33..=63 => quote! { ::core::primitive::u64::MAX >> (64 - n) },
            // 64 => quote! { ::core::primitive::u64::MAX },
            // 65..=127 => quote! { ::core::primitive::u128::MAX >> (128 - n) },
            128 => quote! { ::core::primitive::u128::MAX },
            1..=127 => quote! { ((0_u128 << #n) - 1) as Self::Base },
            _ => unreachable!(),
        };
        tokens.extend(quote! {
            #[doc = #doc_comment]
            #[derive(Copy, Clone)]
            pub enum #ident {}

            impl crate::Specifier for #ident {
                const BITS: usize = #n;
                const MAX_VALUE: Self::Base = #max_value;
                type Base = #t_origin;
                type Face = #t_origin;
            }

            impl crate::private::SpecifierBase for [(); #n] {
                type Base = #t_origin;
            }

            impl crate::private::checks::private::Sealed for [(); #n] {}
        })
    }
    tokens
}
