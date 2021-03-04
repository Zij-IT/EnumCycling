use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EnumCycle)]
pub fn derive_enum_cycle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let up = up_fn(&input.data);
    let down = down_fn(&input.data);

    let expanded = quote!(
        impl #impl_generics EnumCycle for #name #ty_generics #where_clause {
            #up

            #down
        }
    );

    proc_macro::TokenStream::from(expanded)
}

fn up_fn(data: &Data) -> TokenStream {
    match data {
        Data::Enum(en) => {
            let vars = en.variants.iter().collect::<Vec<_>>();
            let variants = en.variants.iter().enumerate().map(|(i, v)| {
                let name = &v.ident;
                if let Some(next_name) = vars.get(i + 1) {
                    let next = *next_name;
                    quote_spanned! {v.span()=>
                        Self::#name => Self::#next,
                    }
                } else {
                    let first = &vars[0].ident;
                    quote_spanned! {v.span()=>
                        Self::#name => Self::#first
                    }
                }

            });

            quote! {
                fn up(&self) -> Self {
                    match *self {
                        #(#variants)*
                    }
                }
            }
        }
        _ => unimplemented!("Not able to be implemented for other types"),
    }
}

fn down_fn(data: &Data) -> TokenStream {
    match data {
        Data::Enum(en) => {
            let vars = en.variants.iter().collect::<Vec<_>>();
            let variants = en.variants.iter().enumerate().rev().map(|(i, v)| {
                let name = &v.ident;
                if i != 0 {
                    let next = &vars[i - 1].ident;
                    quote_spanned! {v.span()=>
                        Self::#name => Self::#next,
                    }
                } else {
                    let leng = vars.len();
                    let last = &vars[leng - 1].ident;
                    quote_spanned! {v.span()=>
                        Self::#name => Self::#last
                    }
                }
            });

            quote! {
                fn down(&self) -> Self {
                    match *self {
                        #(#variants)*
                    }
                }
            }
        },
        _ => unimplemented!("Not able to be implemented for other types"),
    }
}
