use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

pub fn generate(_input: TokenStream2) -> TokenStream2 {
    let mut tokens = quote! {};
    for n in 1usize..=128 {
        let t_origin = match n {
            1..=8 => quote! {u8},
            9..=16 => quote! {u16},
            17..=32 => quote! {u32},
            33..=64 => quote! {u64},
            65..=128 => quote! {u128},
            _ => unreachable!(),
        };
        let ident = format_ident!("B{}", n);
        let doc_comment = format!("Specifier for {} bits.", n);
        tokens.extend(quote! {
            #[doc = #doc_comment]
            pub enum #ident {}

            impl crate::Specifier for #ident {
                const BITS: usize = #n;
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
