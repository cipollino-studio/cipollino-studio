
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, DataStruct, DeriveInput, Ident, Generics};

mod enum_serialization;

fn has_attr(attrs: &Vec<Attribute>, query: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(query))
}

fn serializable_struct(strct: DataStruct, name: Ident, project_type: Option<syn::Type>, generics: Generics) -> proc_macro2::TokenStream {

    // TODO: ensure struct fields are named

    let serializable_fields = strct.fields.iter().filter(|field| !has_attr(&field.attrs, "no_serialize"));
    let serializable_field_names = serializable_fields.clone().map(|field| field.ident.as_ref().unwrap());
    let serializable_field_types = serializable_fields.clone().map(|field| field.ty.to_token_stream()); 
    let serializable_field_names_2 = serializable_field_names.clone(); 
    let impl_generic = if project_type.is_none() {
        quote! { <P: alisa::Project, #generics > }
    } else {
        generics.to_token_stream()
    };
    let context_generic = if let Some(project_type) = project_type {
        quote! { <#project_type> }
    } else {
        quote! { <P> }
    };
    let generics_names = generics.type_params().map(|param| &param.ident);

    quote! {
        impl #impl_generic alisa::Serializable #context_generic for #name <#(#generics_names, )*> {

            fn deserialize(data: &alisa::rmpv::Value, context: &mut alisa::DeserializationContext #context_generic) -> Option<Self> {
                let mut result = Self::default();
                #(
                    if let Some(value) = alisa::rmpv_get(data, stringify!(#serializable_field_names_2)) {
                        if let Some(value) = <#serializable_field_types>::deserialize(value, context) { 
                            result.#serializable_field_names_2 = value;
                        }
                    }
                )*
                Some(result)
            }

            fn serialize(&self, context: &alisa::SerializationContext #context_generic) -> alisa::rmpv::Value {
                alisa::rmpv::Value::Map(vec![
                    #((stringify!(#serializable_field_names).into(), self.#serializable_field_names.serialize(context)), )*
                ])
            }

        }
    }
}



#[proc_macro_derive(Serializable, attributes(project, no_serialize))]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let project_attribute = input.attrs.iter().filter(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "project"
    ).nth(0);
    let project_type = project_attribute.map(|attr| attr.parse_args::<syn::Type>().expect("expected project type!"));

    match input.data {
        syn::Data::Struct(data_struct) => serializable_struct(data_struct, input.ident, project_type, input.generics),
        syn::Data::Enum(data_enum) => enum_serialization::serializable_enum(data_enum, input.ident, project_type, input.generics),
        syn::Data::Union(_data_union) => panic!("cannot serialize union."),
    }.into()
}
