use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[derive(FromField)]
struct FieldOpts {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl ToTokens for FieldOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident.as_ref().unwrap();
        let ty = &self.ty;

        let new = quote! {
            pub fn #ident(&self) -> cobul::Model<#ty> {
                let input = self.0.change(stringify!(#ident), |x| &mut x.#ident);
                cobul::Model{ input, value: self.#ident.clone() }
            }
        };

        tokens.extend(new);
    }
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_any))]
struct TraitOpts {
    ident: syn::Ident,
    data: ast::Data<(), FieldOpts>,
}

impl ToTokens for TraitOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = match &self.data {
            ast::Data::Struct(fields) => &fields.fields,
            _ => unimplemented!(),
        };
        let ident = &self.ident;

        let newtype = syn::Ident::new(&format!("{}Form", self.ident), self.ident.span());

        let new = quote! {
            pub struct #newtype(cobul::State<#ident>);

            impl std::ops::Deref for #newtype {
                type Target = #ident;
                fn deref(&self) -> &Self::Target { &self.0.deref() }
            }

            impl #newtype {
                #(#fields)*
            }

            impl Form for #ident {
                type Wrapper = #newtype;
                fn from(data: cobul::State<#ident>) -> Self::Wrapper {
                    #newtype(data)
                }
            }
        };

        tokens.extend(new);
    }
}

#[proc_macro_derive(Form)]
pub fn derive_hasura(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let opts = TraitOpts::from_derive_input(&parse_macro_input!(input)).unwrap();
    opts.to_token_stream().into()
}
