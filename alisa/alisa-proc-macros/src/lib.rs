
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, DataStruct, DeriveInput, Ident, Generics};

mod enum_serialization;

fn has_attr(attrs: &Vec<Attribute>, query: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(query))
}

fn serializable_struct(strct: DataStruct, name: Ident, generics: Generics) -> proc_macro2::TokenStream {

    // TODO: ensure struct fields are named

    let serializable_fields = strct.fields.iter().filter(|field| !has_attr(&field.attrs, "no_serialize"));
    let serializable_field_names = serializable_fields.clone().map(|field| field.ident.as_ref().unwrap());
    let serializable_field_types = serializable_fields.clone().map(|field| field.ty.to_token_stream()); 
    let serializable_field_names_2 = serializable_field_names.clone(); 
    let generics_names = generics.type_params().map(|param| &param.ident);

    quote! {
        impl alisa::Serializable for #name <#(#generics_names, )*> {

            fn deserialize(data: &alisa::ABFValue, context: &mut alisa::DeserializationContext) -> Option<Self> {
                let mut result = Self::default();
                #(
                    if let Some(value) = data.get(stringify!(#serializable_field_names_2)) {
                        if let Some(value) = <#serializable_field_types>::deserialize(value, context) { 
                            result.#serializable_field_names_2 = value;
                        }
                    }
                )*
                Some(result)
            }

            fn serialize(&self, context: &alisa::SerializationContext) -> alisa::ABFValue {
                alisa::ABFValue::Map(Box::new([
                    #((stringify!(#serializable_field_names).into(), self.#serializable_field_names.serialize(context)), )*
                ]))
            }

        }
    }
}



#[proc_macro_derive(Serializable, attributes(no_serialize))]
pub fn serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let output = match input.data {
        syn::Data::Struct(data_struct) => serializable_struct(data_struct, input.ident, input.generics),
        syn::Data::Enum(data_enum) => enum_serialization::serializable_enum(data_enum, input.ident, input.generics),
        syn::Data::Union(_data_union) => panic!("cannot serialize union."),
    };

    output.into()
}
