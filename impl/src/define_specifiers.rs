use proc_macro2::TokenStream as TokenStream2;
use syn;
use quote::{
    quote,
};

pub fn generate(_input: TokenStream2) -> TokenStream2 {
    let mut tokens = quote!{};
    for n in 1usize..=64 {
        let t_origin = match n {
            1..=8 => quote!{u8},
            9..=16 => quote!{u16},
            17..=32 => quote!{u32},
            33..=64 => quote!{u64},
            65..=128 => quote!{u128},
            _ => unreachable!()
        };
        let ident = syn::Ident::new(&format!("B{}", n), proc_macro2::Span::call_site());
        let doc_comment = format!("Specifier for {} bits.", n);
        tokens.extend(quote!{
            #[doc = #doc_comment]
            pub enum #ident {}

            impl crate::Specifier for #ident {
                const BITS: usize = #n;
                type Base = #t_origin;
                type Face = #t_origin;
            }

            impl crate::SpecifierBase for [(); #n] {
                type Base = #t_origin;
            }

            impl crate::checks::private::Sealed for [(); #n] {}
        })
    }
    tokens
}
