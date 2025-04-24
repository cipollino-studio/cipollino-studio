
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
                    alisa::ABFValue::NamedUnitEnum(stringify!(#variant_name).into())
                })
            },
            Fields::Named(named) => {
                let names = named.named.iter().map(|field| field.ident.as_ref().unwrap());
                let names2 = named.named.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names = quote! { {#(#names, )*} };

                let serialization = quote! {
                    alisa::ABFValue::NamedEnum(
                        stringify!(#variant_name).into(),
                        Box::new(alisa::ABFValue::Map(Box::new([
                            #((stringify!(#names2).into(), #names2.serialize(context)), )*
                        ])))
                    )
                };

                (field_names, serialization)
            },
            Fields::Unnamed(unnamed) => {
                let n_fields = unnamed.unnamed.iter().count();
                let names = (0..n_fields).map(|idx| Ident::new(&format!("field{}", idx), Span::call_site()));
                let names2 = names.clone();
                let field_names = quote! { (#(#names, )*) };

                let serialization = quote! {
                    alisa::ABFValue::NamedEnum(
                        stringify!(#variant_name).into(),
                        Box::new(alisa::ABFValue::Array(Box::new([
                            #(#names2.serialize(context),)*
                        ])))
                    )
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
                let idxs = 0..n_fields;
                quote! {
                    let data = data.as_array()?;
                    if data.len() < #n_fields {
                        return None;
                    }
                    Some(Self::#variant_name(
                        #(alisa::Serializable::deserialize(&data[#idxs], context)?, )*
                    )) 
                }
            },
            Fields::Named(named) => {
                let names = named.named.iter().map(|field| field.ident.as_ref().unwrap());
                quote! {
                    Some(Self::#variant_name {
                        #(#names: alisa::Serializable::deserialize(data.get(stringify!(#names))?, context)?, )*
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
        impl alisa::Serializable for #name <#(#generics_names, )*> {

            fn serialize(&self, context: &alisa::SerializationContext) -> alisa::ABFValue {
                match self {
                    #(#serialize_variants,)*
                }
            }

            fn deserialize(data: &alisa::ABFValue, context: &mut alisa::DeserializationContext) -> Option<Self> {
                let (name, data) = match data {
                    alisa::ABFValue::NamedUnitEnum(name) => (name.as_str(), &alisa::ABFValue::PositiveInt(0)),
                    alisa::ABFValue::NamedEnum(name, data) => (name.as_str(), &**data),
                    _ => { return None; }
                };
                match name {
                    #(#deserialize_variants,)*
                    _ => None
                }
            }

        }
    }
}
