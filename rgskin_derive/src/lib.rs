use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemStruct, Path, Token, Ident};
use syn::parse::{Parse, ParseStream};

#[proc_macro_derive(MergeDefault, attributes(merge))]
pub fn derive_merge_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("MergeDefault only works on structs with named fields"),
        },
        _ => panic!("MergeDefault only works on structs"),
    };

    let field_calls = fields.iter().filter_map(|f| {
        let is_skipped = f.attrs.iter().any(|attr| {
            attr.path().is_ident("merge") && 
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") { Ok(()) } else { Err(meta.error("unsupported")) }
            }).is_ok()
        });

        if is_skipped { return None; }

        let field_name = &f.ident;
        Some(quote! {
            if self.#field_name == default_km.#field_name {
                self.#field_name = shared_km.#field_name.clone();
            }
        })
    });

    let expanded = quote! {
        impl #name {
            /// merges right into left if left is default
            pub fn merge_default(&mut self, shared_km: &#name) {
                let default_km = #name::default();
                #(#field_calls)*
            }
        }
    };

    TokenStream::from(expanded)
}

// textures

#[proc_macro_derive(GetAllTextures)]
pub fn derive_get_all_textures(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut field_calls = Vec::new();

    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                for field in fields.named {
                    let field_name = field.ident.unwrap();
                    field_calls.push(quote! {
                        textures.extend( 
                            (&crate::io::traits::Wrap(&self.#field_name))._extract_textures() 
                        );
                    });
                }
            }
            Fields::Unnamed(fields) => {
                for (i, _) in fields.unnamed.iter().enumerate() {
                    let idx = syn::Index::from(i);
                    field_calls.push(quote! {
                        textures.extend( 
                            (&crate::io::traits::Wrap(&self.#idx))._extract_textures() 
                        );
                    });
                }
            }
            Fields::Unit => {}
        },
        _ => {
            return syn::Error::new_spanned(name, "GetAllTextures can only be derived on structs")
                .to_compile_error()
                .into();
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
            let ident: Ident = input.parse()?;
            
            if ident != "strategy" {
                return Err(syn::Error::new(ident.span(), "Expected 'strategy'"));
            }
            
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

    let inject_merge_attr = |attrs: &mut Vec<syn::Attribute>| {
        let has_merge_attr = attrs.iter().any(|a| a.path().is_ident("merge"));
        if !has_merge_attr {
            let new_attr: syn::Attribute = syn::parse_quote! {
                #[merge(strategy = #strat_path)]
            };
            attrs.push(new_attr);
        }
    };

    match &mut item_struct.fields {
        syn::Fields::Named(fields) => {
            for field in &mut fields.named {
                inject_merge_attr(&mut field.attrs);
            }
        }
        syn::Fields::Unnamed(fields) => {
            for field in &mut fields.unnamed {
                inject_merge_attr(&mut field.attrs);
            }
        }
        syn::Fields::Unit => {}
    }

    let expanded = quote! {
        #item_struct
    };

    proc_macro::TokenStream::from(expanded)
}
