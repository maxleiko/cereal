use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Fields};

#[proc_macro_derive(Readable)]
pub fn derive_readable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_readable_trait(&ast)
}

fn impl_readable_trait(ast: &syn::DeriveInput) -> TokenStream {
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
            impl cereal::Readable for #struct_name {
                fn from_bytes(mut bytes: &[u8]) -> ::std::io::Result<Self>
                where
                    Self:Sized {
                        Ok(#struct_name {
                            #(
                                #field_name: cereal::Deserialize::deserialize(&mut bytes)?,
                            )*
                        })
                }
            }
        }
    } else {
        let param = ast.generics.params.iter();
        let param2 = ast.generics.params.iter();
        quote! {
            impl<#(#param),*> cereal::Readable for #struct_name<#(#param2),*> {
                fn from_bytes(mut bytes: &[u8]) -> ::std::io::Result<Self>
                where
                    Self:Sized {
                        Ok(#struct_name {
                            #(
                                #field_name: cereal::Deserialize::deserialize(&mut bytes)?,
                            )*
                        })
                }
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Writable)]
pub fn derive_writable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_writable_trait(&ast)
}

fn impl_writable_trait(ast: &syn::DeriveInput) -> TokenStream {
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
            impl cereal::Writable for #struct_name {
                fn write(&self, bytes: &mut Vec<u8>) -> ::std::io::Result<usize>
                where
                    Self:Sized {
                        let mut n = 0;
                        #(
                            n += self.#field_name.serialize(bytes)?;
                        )*
                        Ok(n)
                }
            }
        }
    } else {
        let param = ast.generics.params.iter();
        let param2 = ast.generics.params.iter();
        quote! {
            impl<#(#param),*> cereal::Writable for #struct_name<#(#param2),*> {
                fn write(&self, bytes: &mut Vec<u8>) -> ::std::io::Result<usize>
                where
                    Self:Sized {
                        let mut n = 0;
                        #(
                            n += self.#field_name.serialize(bytes)?;
                        )*
                        Ok(n)
                }
            }
        }
    };

    gen.into()
}
