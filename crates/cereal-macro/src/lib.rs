use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Fields};

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_deserialize_trait(&ast)
}

fn impl_deserialize_trait(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("not unhandled yet (only struct fields are handled)"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    let gen = if ast.generics.params.is_empty() {
        quote! {
            impl<'de> cereal::Deserialize<'de> for #struct_name {
                fn deserialize(bytes: &mut &'de [u8]) -> ::std::io::Result<Self>
                where
                    Self:Sized {
                        Ok(#struct_name {
                            #(
                                #field_name: cereal::Deserialize::deserialize(bytes)?,
                            )*
                        })
                }
            }
        }
    } else {
        let lifetime = ast
            .generics
            .params
            .iter()
            .filter(|param| matches!(param, syn::GenericParam::Lifetime(_)));
        let param = ast.generics.params.iter();
        let param2 = ast.generics.params.iter();
        let type_param = ast
            .generics
            .params
            .iter()
            .filter(|param| matches!(param, syn::GenericParam::Type(_)));
        quote! {
            impl<'de, #(#param),*> cereal::Deserialize<'de> for #struct_name<#(#param2),*>
            where
                #('de :#lifetime),*
                #(#type_param: cereal::Deserialize<'de>),*
            {
                fn deserialize(mut bytes: &mut &'de [u8]) -> ::std::io::Result<Self>
                where
                    Self:Sized {
                        Ok(#struct_name {
                            #(
                                #field_name: cereal::Deserialize::deserialize(bytes)?,
                            )*
                        })
                }
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_serialize_trait(&ast)
}

fn impl_serialize_trait(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("not unhandled yet (only struct fields are handled)"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    let gen = if ast.generics.params.is_empty() {
        quote! {
            impl cereal::Serialize for #struct_name {
                fn serialize<W>(&self, mut bytes: W) -> ::std::io::Result<usize>
                where
                    Self:Sized,
                    W: std::io::Write {
                        let mut n = 0;
                        #(
                            n += self.#field_name.serialize(&mut bytes)?;
                        )*
                        Ok(n)
                }
            }
        }
    } else {
        let param = ast.generics.params.iter();
        let param2 = ast.generics.params.iter();
        let type_param = ast
            .generics
            .params
            .iter()
            .filter(|param| matches!(param, syn::GenericParam::Type(_)));
        quote! {
            impl<#(#param),*> cereal::Serialize for #struct_name<#(#param2),*>
            where
                #(#type_param: cereal::Serialize),*
            {
                fn serialize<W>(&self, mut bytes: W) -> ::std::io::Result<usize>
                where
                    Self:Sized,
                    W: std::io::Write {
                        let mut n = 0;
                        #(
                            n += self.#field_name.serialize(&mut bytes)?;
                        )*
                        Ok(n)
                }
            }
        }
    };

    gen.into()
}
