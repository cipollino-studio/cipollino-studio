
use proc_macro2::Span;
use quote::quote;
use syn::{DataEnum, Fields, Ident, Generics};

pub fn serializable_enum(enm: DataEnum, name: Ident, generics: Generics) -> proc_macro2::TokenStream {

    if enm.variants.iter().count() == 0 {
        panic!("cannot serialize empty enums.");
    }

    let generics_names = generics.type_params().map(|param| &param.ident);

    let serialize_variants = enm.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let (field_names, serialization) = match &variant.fields {
            Fields::Unit => {
                (quote! {}, quote!{
                    ::alisa::rmpv::Value::Array(vec![
                        stringify!(#variant_name).into()
                    ])
                })
            },
            Fields::Named(named) => {
                let names = named.named.iter().map(|field| field.ident.as_ref().unwrap());
                let names2 = named.named.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names = quote! { {#(#names, )*} };

                let serialization = quote! {
                    ::alisa::rmpv::Value::Array(vec![
                        stringify!(#variant_name).into(),
                        ::alisa::rmpv::Value::Map(vec![
                            #((stringify!(#names2).into(), #names2.serialize(context)), )*
                        ])
                    ])
                };

                (field_names, serialization)
            },
            Fields::Unnamed(unnamed) => {
                let n_fields = unnamed.unnamed.iter().count();
                let names = (0..n_fields).map(|idx| Ident::new(&format!("field{}", idx), Span::call_site()));
                let names2 = names.clone();
                let field_names = quote! { (#(#names, )*) };

                let serialization = quote! {
                    ::alisa::rmpv::Value::Array(vec![
                        stringify!(#variant_name).into(),
                        #(#names2.serialize(context),)*
                    ])
                };

                (field_names, serialization)
            }
        };

        quote! {
            Self:: #variant_name #field_names => {
                #serialization
            }
        }
    });

    let deserialize_variants = enm.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let deserialization = match &variant.fields {
            Fields::Unit => {
                quote! {
                    Some(Self:: #variant_name)
                }
            },
            Fields::Unnamed(unnamed) => {
                let n_fields = unnamed.unnamed.iter().count();
                let idxs = 1..(n_fields + 1);
                quote! {
                    if data.len() < #n_fields + 1 {
                        return None;
                    }
                    Some(Self::#variant_name(
                        #(::alisa::Serializable::deserialize(&data[#idxs], context)?, )*
                    )) 
                }
            },
            Fields::Named(named) => {
                let names = named.named.iter().map(|field| field.ident.as_ref().unwrap());
                quote! {
                    let data = data.get(1)?;
                    Some(Self::#variant_name {
                        #(#names: ::alisa::Serializable::deserialize(::alisa::rmpv_get(data, stringify!(#names))?, context)?, )*
                    })
                }
            }
        };

        quote! {
            stringify!(#variant_name) => {
                #deserialization
            }
        }
    });

    quote! {
        impl ::alisa::Serializable for #name <#(#generics_names, )*> {

            fn serialize(&self, context: &alisa::SerializationContext) -> ::alisa::rmpv::Value {
                match self {
                    #(#serialize_variants,)*
                }
            }

            fn deserialize(data: &::alisa::rmpv::Value, context: &mut ::alisa::DeserializationContext) -> Option<Self> {
                let data = data.as_array()?;
                let variant_name = data[0].as_str()?;
                match variant_name {
                    #(#deserialize_variants,)*
                    _ => None
                }
            }

        }
    }
}