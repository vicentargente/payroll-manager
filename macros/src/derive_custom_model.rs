use darling::{util::PathList, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, Fields, Ident, Path, Visibility};
use syn::Data::Struct;

pub(crate) fn derive_custom_model_impl(input: TokenStream) -> TokenStream {
    let original_struct = parse_macro_input!(input as DeriveInput);

    let DeriveInput { data, .. } = original_struct.clone();

    if let Struct(data_struct) = data {
        let DataStruct { fields, .. } = data_struct;

        let args = match CustomModelArgs::from_derive_input(&original_struct) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

        let CustomModelArgs { models } = args;

        let mut output = quote!();

        if models.is_empty() {
            panic!(
                "Please specify at least 1 model using the `model` attribute"
            )
        }

        for model in models {
            let generated_model = generate_custom_model(&fields, &model);

            output.extend(quote!(#generated_model));
        }

        output.into()
    } else {
        panic!("DeriveCustomModel can only be used with named structs")
    }
}

fn generate_custom_model(fields: &Fields, model: &CustomModel) -> proc_macro2::TokenStream {
    let CustomModel {
        name,
        fields: target_fields,
        extra_derives
    } = model;

    let mut new_fields = quote!();

    for Field {
        ident,
        attrs,
        vis,
        colon_token,
        ty,
        ..
    } in fields
    {
        // Force visibility to `pub` if it's not already `pub`
        let forced_vis = match vis {
            Visibility::Public(_) => vis.clone(), // Already `pub`, leave it as is
            _ => syn::parse_quote!(pub),         // Not `pub`, force it to `pub`
        };

        let Some(ident) = ident else {
            panic!("Failed to get struct field identifier")
        };

        let path = match Path::from_string(&ident.clone().to_string()) {
            Ok(path) => path,
            Err(error) => panic!("Failed to convert field identifier to path: {error:?}"),
        };

        if !target_fields.contains(&path) {
            continue;
        }

        new_fields.extend(quote! {
            #(#attrs)*
            #forced_vis #ident #colon_token #ty,
        });
    }

    let struct_ident = match Ident::from_string(name) {
        Ok(ident) => ident,
        Err(error) => panic!("{error:?}"),
    };

    let mut extra_derives_output = quote!();
    if !extra_derives.is_empty() {
        extra_derives_output.extend(quote! {
            #(#extra_derives,)*
        })
    }

    quote! {
        #[derive(#extra_derives_output)]
        pub struct #struct_ident {
            #new_fields
        }
    }
}

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(custom_model), supports(struct_named))]
struct CustomModelArgs {
    #[darling(default, multiple, rename = "model")]
    pub models: Vec<CustomModel>,
}

#[derive(FromMeta, Clone)]
struct CustomModel {
    name: String,
    fields: PathList,
    #[darling(default)]
    extra_derives: PathList
}