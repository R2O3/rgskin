use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemStruct, Path, Token, Ident};
use syn::parse::{Parse, ParseStream};

// textures

#[proc_macro_derive(GetAllTextures)]
pub fn derive_get_all_textures(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut field_calls = Vec::new();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.unwrap();

                field_calls.push(quote! {
                    textures.extend( 
                        (&crate::io::traits::Wrap(&self.#field_name))._extract_textures() 
                    );
                });
            }
        }
    }

    let expanded = quote! {
        impl crate::io::traits::GetAllTextures for #name {
            fn get_all_textures(&self) -> Vec<std::sync::Arc<std::sync::RwLock<crate::io::texture::Texture>>> {
                #[allow(unused_imports)]
                use crate::io::traits::{ExtractSpecific, ExtractFallback};
                
                let mut textures = Vec::new();
                #( #field_calls )*
                textures
            }
        }
    };

    TokenStream::from(expanded)
}

// merge

struct MergeStrategy {
    path: Path,
}

impl Parse for MergeStrategy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![=]) {
            let _ident: Ident = input.parse()?;
            let _eq: Token![=] = input.parse()?;
        }
        let path: Path = input.parse()?;
        Ok(Self { path })
    }
}

#[proc_macro_attribute]
pub fn merge_for_all(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let strategy = parse_macro_input!(attr as MergeStrategy);
    let strat_path = strategy.path;
    
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in &mut fields.named {
            let has_merge_attr = field.attrs.iter().any(|a| a.path().is_ident("merge"));

            if !has_merge_attr {
                let new_attr: syn::Attribute = syn::parse_quote! {
                    #[merge(strategy = #strat_path)]
                };
                field.attrs.push(new_attr);
            }
        }
    }

    let expanded = quote::quote! {
        #item_struct
    };

    proc_macro::TokenStream::from(expanded)
}
