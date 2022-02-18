use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, Fields, FieldsNamed, GenericArgument, Meta, NestedMeta, Path, Type,
};

#[proc_macro_derive(FieldFilterable, attributes(field_filterable_on))]
pub fn derive_field_filterable_impl(ts: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(ts as syn::DeriveInput);

    let name = input.ident;

    let object_path = get_object_path(input.attrs);

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!();
    };

    let filter_logic_q = fields.iter().map(|f| {
        let name = f.ident.as_ref().expect("cannot be derived on tuples");
        let ty = &f.ty;
        if ty_inner_type("Option", ty).is_some() {
            let name_str = format!("{}", name);
            quote! {
                let #name = if fields.contains(#name_str) { Some(o.#name) } else { None };
            }
        } else {
            quote! {
                let #name = o.#name;
            }
        }
    });

    let return_o = fields.iter().map(|f| {
        let name = f.ident.as_ref().expect("cannot be derived on tuples");
        quote! { #name }
    });

    let expanded = quote! {
        impl FieldFilterable<#object_path> for #name {
            fn field_filter(o: #object_path, fields: std::collections::HashSet<String>) -> Self {
                #(#filter_logic_q)*
                Self { #(#return_o),* }
            }
        }
    };
    expanded.into()
}

fn get_object_path(attrs: Vec<Attribute>) -> Path {
    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        if let Meta::List(list) = meta {
            match get_single_segment(&list.path) {
                // this is the attr name we care about
                Some(ref seg) if seg == "field_filterable_on" => {
                    for nested in list.nested {
                        if let NestedMeta::Meta(Meta::Path(path)) = nested {
                            return path;
                        }
                    }
                }
                _ => (),
            }
        }
    }
    unimplemented!(r#"#[field_filterable_on(<TYPE>)] must be set"#)
}

fn get_single_segment(path: &Path) -> Option<String> {
    if path.segments.len() == 1 {
        Some(path.segments[0].ident.to_string())
    } else {
        None
    }
}

fn ty_inner_type<'a>(wrapper: &str, ty: &'a Type) -> Option<&'a Type> {
    if let Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}
