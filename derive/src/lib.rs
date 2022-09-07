use convert_case::{Case, Casing};
use darling::{ast, FromDeriveInput, FromField, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

/*
#[prop_or_default]
pub active: Active,

classes!(active.or(use_context::<Active>()))
 */

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    all: bool,
}

#[derive(FromField, Clone)]
#[darling(attributes(form))]
struct FieldOpts {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    context: Option<String>,

    #[darling(default)]
    with: Option<String>,

    #[darling(default)]
    include: bool,

    #[darling(default)]
    except: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(form), supports(struct_any))]
struct TraitOpts {
    ident: syn::Ident,
    data: ast::Data<(), FieldOpts>,
}

#[proc_macro_derive(Classable, attributes(classable))]
pub fn classable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let opts = TraitOpts::from_derive_input(&parse_macro_input!(input)).unwrap();
    opts.to_token_stream().into()
}

impl ToTokens for FieldOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let context = self
            .context
            .as_ref()
            .map(|name| syn::Ident::new(name, self.ident.span()));

        let stream = match (context, self.include) {
            (Some(ty), _) => quote! { #ident.or(yew::use_context::<#ty>()) },
            (None, true) => quote! { #ident },
            (_, _) => quote! {},
        };

        tokens.extend(stream)
    }
}

impl ToTokens for TraitOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = match &self.data {
            ast::Data::Struct(fields) => &fields.fields,
            _ => unimplemented!(),
        };

        let stream = quote! {
            impl std::convert::Into<yew::Classes> for #ident {
                fn into(self) -> yew::Classes {
                    classes!(#(#fields),*)
                }
            }
        };

        tokens.extend(stream)
    }
}
