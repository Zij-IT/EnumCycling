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

    let non_skipped_variants = non_skipped_variants(&input.data)?;

    if non_skipped_variants.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "EnumCycle requires that the enum have at least 1 non-skipped variant.",
        ));
    }

    let cycle_variants = if let Some(cycle_path) = get_cycle_path(input) {
        ident_path_to_variants(cycle_path, non_skipped_variants)?
    } else {
        non_skipped_variants
    };

    let up = simple_path(&cycle_variants, Mode::Up);
    let down = simple_path(&cycle_variants, Mode::Down);

    Ok(quote! {
        impl #impl_generics ::enum_cycling::EnumCycle for #name #ty_generics #where_clause {
            #up
            #down
        }
    })
}

fn non_skipped_variants(data: &Data) -> ::syn::Result<Vec<&Variant>> {
    match data {
        Data::Enum(en) => Ok(en
            .variants
            .iter()
            .filter(|x| {
                !x.attrs
                    .iter()
                    .map(|attr| attr.path.segments.iter())
                    .flatten()
                    .any(|seg| seg.ident == "skip")
            })
            .collect::<Vec<_>>()),
        Data::Struct(s) => Err(::syn::Error::new(
            s.struct_token.span(),
            "This macro only supports enums.",
        )),
        Data::Union(u) => Err(::syn::Error::new(
            u.union_token.span(),
            "This macro only supports enums.",
        )),
    }
}

fn get_cycle_path(input: &DeriveInput) -> Option<Vec<Ident>> {
    use proc_macro2::TokenTree;

    input
        .attrs
        .iter()
        .find(|a| a.path.segments.iter().any(|s| s.ident == "cycle"))
        .and_then(|attr| {
            attr.tokens
                .clone()
                .into_iter()
                .next()
                .and_then(|tt| match tt {
                    TokenTree::Group(g) => Some(
                        g.stream()
                            .into_iter()
                            .filter_map(|x| match x {
                                TokenTree::Ident(x) => Some(x),
                                _ => None,
                            })
                            .collect(),
                    ),
                    _ => None,
                })
        })
}

fn ident_path_to_variants(
    cycle_path: Vec<Ident>,
    variants: Vec<&Variant>,
) -> ::syn::Result<Vec<&Variant>> {
    cycle_path
        .iter()
        .map(|x| -> syn::Result<&Variant> {
            variants
                .iter()
                .find(|&&y| y.ident == *x)
                .copied()
                .ok_or_else(|| {
                    syn::Error::new(x.span(), &format!("No matching variant found for {}.", x))
                })
        })
        .collect()
}

fn simple_path(variants: &[&Variant], mode: Mode) -> TokenStream {
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
                        ::std::iter::repeat(quote!(::std::default::Default::default())).take(fields.unnamed.len());
                    quote! {(#(#defaults),*)}
                }
                Fields::Named(fields) => {
                    let fields = fields
                        .named
                        .iter()
                        .map(|field| field.ident.as_ref().unwrap());
                    quote! {{#(#fields: ::std::default::Default::default()), *}}
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
                _ => ::std::panic!("Unable to call \"{}\" fn on a skipped variant", #name),
            }
        }
    }
}
