use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasComponent, attributes(component))]
pub fn derive_has_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("HasComponent can only be derived for structs."),
    };

    let name = input.ident;
    let field_names = data.fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_types = data.fields.iter().map(|f| &f.ty);

    let expanded = quote! {#(
        impl HasComponent<#field_types> for #name {
            fn get_component(&self) -> &#field_types {
                &self.#field_names
            }

            fn get_component_mut(&mut self) -> &mut #field_types {
                &mut self.#field_names
            }
        }
    )*};

    TokenStream::from(expanded)
}
