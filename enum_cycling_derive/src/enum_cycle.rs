use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Ident, Variant};

enum Mode {
    Up,
    Down,
}

pub fn enum_cycle_inner(input: &DeriveInput) -> ::syn::Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants = non_skipped_variants(&input.data)?;
    let up = variant_matches(&variants, Mode::Up);
    let down = variant_matches(&variants, Mode::Down);

    Ok(quote! {
        impl #impl_generics EnumCycle for #name #ty_generics #where_clause {
            #up
            #down
        }
    })
}

fn non_skipped_variants(data: &Data) -> ::syn::Result<Vec<&Variant>> {
    match data {
        Data::Enum(en) => {
            let variants = en
                .variants
                .iter()
                .filter(|x| {
                    !x.attrs
                        .iter()
                        .map(|attr| attr.path.segments.iter())
                        .flatten()
                        .any(|seg| seg.ident == "skip")
                })
                .collect::<Vec<_>>();

            if variants.len() > 1 {
                Ok(variants)
            } else {
                Err(syn::Error::new(
                    en.enum_token.span(),
                    "EnumCycle requires that the enum have at least 2 non-skipped variants.",
                ))
            }
        }
        Data::Struct(s) => Err(::syn::Error::new(s.struct_token.span(), "This macro only supports enums.")),
        Data::Union(u) => Err(::syn::Error::new(u.union_token.span(), "This macro only supports enums.")),
    }
}

fn variant_matches(variants: &[&Variant], mode: Mode) -> TokenStream {
    let (skip_amt, func_name) = match mode {
        Mode::Up => (variants.len() - 1, Ident::new("up", Span::call_site())),
        Mode::Down => (1, Ident::new("down", Span::call_site())),
    };

    let arms = variants
        .iter()
        .zip(variants.iter().cycle().skip(skip_amt))
        .map(|(&left, &right)| {
            let l_ident = &left.ident;
            let r_ident = &right.ident;

            let l_params = match &left.fields {
                Fields::Unit => quote! {},
                Fields::Unnamed(_) => {
                    quote! {(..)}
                }
                Fields::Named(_) => {
                    quote! {{..}}
                }
            };

            let r_params = match &right.fields {
                Fields::Unit => quote! {},
                Fields::Unnamed(fields) => {
                    let defaults =
                        ::std::iter::repeat(quote!(Default::default())).take(fields.unnamed.len());
                    quote! {(#(#defaults),*)}
                }
                Fields::Named(fields) => {
                    let fields = fields
                        .named
                        .iter()
                        .map(|field| field.ident.as_ref().unwrap());
                    quote! {{#(#fields: Default::default()), *}}
                }
            };

            quote! {
                Self::#l_ident #l_params => Self::#r_ident #r_params
            }
        });

    let name = func_name.to_string();

    quote! {
        fn #func_name(&self) -> Self {
            match *self {
                #(#arms),*,
                _ => panic!("Unable to call \"{}\" fn on a skipped variant", #name),
            }
        }
    }
}
