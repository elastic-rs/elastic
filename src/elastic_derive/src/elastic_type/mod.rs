use super::{
    expect_ident,
    expect_list,
    expect_name_value,
    get_elastic_meta_items,
    get_ident_from_lit,
    get_tokens_from_lit,
};
use quote::Tokens;
use serde_derive_internals::{
    self,
    attr as serde_attr,
};
use syn;

struct ElasticDocumentMapping {
    ident: syn::Ident,
    definition: Tokens,
    impl_block: Tokens,
}

/**
Derive `DocumentType` for the given input.

The input must satisfy the following rules:

- It must be a struct.
- The structs field types must implement `FieldType` (or be ignored).
- A mapping type supplied by `#[elastic(mapping="<ident>")]` must implement `DocumentMapping`,
but not `PropertiesMapping`.
*/
pub fn expand_derive(
    crate_root: Tokens,
    input: &syn::MacroInput,
) -> Result<Vec<Tokens>, DeriveElasticTypeError> {
    // Annotatable item for a struct with struct fields
    let fields = match input.body {
        syn::Body::Struct(ref data) => match *data {
            syn::VariantData::Struct(ref fields) => Some(fields),
            _ => None,
        },
        _ => None,
    };

    let fields = fields.ok_or(DeriveElasticTypeError::InvalidInput)?;

    // Get the serializable fields
    let fields: Vec<(syn::Ident, &syn::Field)> = fields
        .iter()
        .map(|f| get_ser_field(f))
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect();

    let mapping = get_mapping(&crate_root, input);

    let doc_ty_impl_block = get_doc_ty_impl_block(&crate_root, input, &fields, &mapping.ident);

    let props_impl_block = get_props_impl_block(&crate_root, &input.ident, &fields);

    let dummy_wrapper = syn::Ident::new(format!("_IMPL_EASTIC_TYPE_FOR_{}", input.ident));

    let mapping_definition = &mapping.definition;
    let mapping_impl_block = &mapping.impl_block;

    Ok(vec![quote!(
        #[allow(missing_docs)]
        #mapping_definition

        #[allow(non_upper_case_globals, dead_code, unused_variables)]
        const #dummy_wrapper: () = {
            #mapping_impl_block

            #doc_ty_impl_block

            #props_impl_block
        };
    )])
}

fn get_mapping(crate_root: &Tokens, input: &syn::MacroInput) -> ElasticDocumentMapping {
    // Define a struct for the mapping with a few defaults
    fn define_mapping(vis: &syn::Visibility, name: &syn::Ident) -> Tokens {
        quote!(
            #[derive(Default, Clone, Copy, Debug)]
            #vis struct #name;
        )
    }

    // Get the default mapping name
    fn get_default_mapping(item: &syn::MacroInput) -> syn::Ident {
        syn::Ident::from(format!("{}Mapping", item.ident))
    }

    // Get the mapping ident supplied by an #[elastic()] attribute or create a default one
    fn get_mapping_from_attr(item: &syn::MacroInput) -> Option<syn::Ident> {
        let val = get_elastic_meta_items(&item.attrs);

        let val = val
            .iter()
            .filter_map(|meta| expect_name_value("mapping", &meta))
            .next();

        val.and_then(|v| get_ident_from_lit(v).ok())
    }

    // Implement DocumentMapping for the mapping
    fn impl_document_mapping(
        crate_root: &Tokens,
        mapping: &syn::Ident,
        properties: &syn::Ident,
    ) -> Tokens {
        quote!(
            impl #crate_root::__derive::ObjectMapping for #mapping {
                type Properties = #properties;
            }
        )
    }

    if let Some(ident) = get_mapping_from_attr(input) {
        ElasticDocumentMapping {
            ident,
            definition: Tokens::new(),
            impl_block: Tokens::new(),
        }
    } else {
        let ident = get_default_mapping(input);
        let definition = define_mapping(&input.vis, &ident);
        let impl_block = impl_document_mapping(&crate_root, &ident, &input.ident);

        ElasticDocumentMapping {
            ident,
            definition,
            impl_block,
        }
    }
}

// Implement DocumentType for the type being derived with the mapping
fn get_doc_ty_impl_block(
    crate_root: &Tokens,
    item: &syn::MacroInput,
    fields: &[(syn::Ident, &syn::Field)],
    mapping: &syn::Ident,
) -> Tokens {
    struct MetadataBlock {
        instance_methods: Tokens,
        static_impls: Tokens,
    }

    // Implement DocumentMetadata for the type being derived with the mapping
    fn get_doc_ty_methods(
        crate_root: &Tokens,
        item: &syn::MacroInput,
        fields: &[(syn::Ident, &syn::Field)],
    ) -> MetadataBlock {
        struct ElasticMetadataMethods {
            index: Tokens,
            index_is_static: bool,
            ty: Tokens,
            ty_is_static: bool,
            id: Tokens,
        }

        // Get the default method blocks for `DocumentType`
        fn get_doc_type_methods(
            crate_root: &Tokens,
            item: &syn::MacroInput,
            fields: &[(syn::Ident, &syn::Field)],
        ) -> ElasticMetadataMethods {
            // Get the default name for the indexed elasticsearch type name
            fn get_elastic_type_name(item: &syn::MacroInput) -> syn::Lit {
                syn::Lit::Str(
                    format!("{}", item.ident).to_lowercase(),
                    syn::StrStyle::Cooked,
                )
            }

            let (index, index_is_static) = {
                match get_method_from_struct(item, "index") {
                    Some(MethodFromStruct::Literal(name)) => (name, true),
                    Some(MethodFromStruct::Expr(expr)) => (expr, false),
                    _ => {
                        let name = get_elastic_type_name(item);
                        (quote!(#name), true)
                    }
                }
            };

            let (ty, ty_is_static) = {
                match get_method_from_struct(item, "ty") {
                    Some(MethodFromStruct::Literal(name)) => (name, true),
                    Some(MethodFromStruct::Expr(expr)) => (expr, false),
                    _ => (quote!(#crate_root::__derive::DEFAULT_DOC_TYPE), true),
                }
            };

            let id = get_method_from_struct(item, "id")
                .map(|id_expr| match id_expr {
                    MethodFromStruct::Expr(expr) => expr,
                    _ => panic!("id attributes on a struct definition must be of the form #[id(expr = \"expression\")]"),
                })
                .or_else(|| {
                    get_method_from_fields(fields, "id").map(|field| match field {
                        MethodFromField::Field(field) => quote!(&self . #field),
                        MethodFromField::Expr(field, expr) => quote!({
                            let #field = &self . #field;
                            #expr
                        }),
                        _ => panic!("id attributes on a field must be of the form #[id] or #[id(expr = \"expression\")]"),
                    })
                })
                .map(|id_expr| quote!(Some((#id_expr).into())))
                .unwrap_or_else(|| quote!(None));

            ElasticMetadataMethods {
                index,
                index_is_static,
                ty,
                ty_is_static,
                id,
            }
        }

        let ElasticMetadataMethods {
            ref index,
            index_is_static,
            ref ty,
            ty_is_static,
            ref id,
        } = get_doc_type_methods(crate_root, item, fields);

        let doc_ty = &item.ident;

        let (partial_static_index, static_index_block) = if index_is_static {
            let method = quote!(
                fn partial_static_index() -> ::std::option::Option<#crate_root::__derive::Index<'static>> {
                    Some((#index).into())
                }
            );

            let block = quote!(
                impl #crate_root::__derive::StaticIndex for #doc_ty { }
            );

            (Some(method), Some(block))
        } else {
            let method = quote!(
                fn partial_static_index() -> ::std::option::Option<#crate_root::__derive::Index<'static>> {
                    None
                }
            );

            (Some(method), None)
        };

        let (partial_static_ty, static_ty_block) = if ty_is_static {
            let method = quote!(
                fn partial_static_ty() -> ::std::option::Option<#crate_root::__derive::Type<'static>> {
                    Some((#ty).into())
                }
            );

            let block = quote!(
                impl #crate_root::__derive::StaticType for #doc_ty { }
            );

            (Some(method), Some(block))
        } else {
            let method = quote!(
                fn partial_static_ty() -> ::std::option::Option<#crate_root::__derive::Type<'static>> {
                    None
                }
            );

            (Some(method), None)
        };

        let instance_methods = quote!(
            fn index(&self) -> #crate_root::__derive::Index {
                (#index).into()
            }

            fn ty(&self) -> #crate_root::__derive::Type {
                (#ty).into()
            }

            fn partial_id(&self) -> ::std::option::Option<#crate_root::__derive::Id> {
                (#id).into()
            }

            #partial_static_index

            #partial_static_ty
        );

        MetadataBlock {
            instance_methods,
            static_impls: quote!(
                #static_index_block

                #static_ty_block
            ),
        }
    }

    let doc_ty = &item.ident;

    let MetadataBlock {
        instance_methods,
        static_impls,
    } = get_doc_ty_methods(crate_root, item, fields);

    quote!(
        impl #crate_root::__derive::ObjectFieldType for #doc_ty {
            type Mapping = #mapping;
        }

        impl #crate_root::__derive::DocumentType for #doc_ty {
            #instance_methods
        }

        #static_impls
    )
}

// Implement PropertiesMapping for the mapping
fn get_props_impl_block(
    crate_root: &Tokens,
    props_ty: &syn::Ident,
    fields: &[(syn::Ident, &syn::Field)],
) -> Tokens {
    // Get the serde serialisation statements for each of the fields on the type being derived
    fn get_field_ser_stmts(
        crate_root: &Tokens,
        fields: &[(syn::Ident, &syn::Field)],
    ) -> Vec<Tokens> {
        let fields: Vec<Tokens> = fields
            .iter()
            .cloned()
            .map(|(name, field)| {
                let lit = syn::Lit::Str(name.as_ref().to_string(), syn::StrStyle::Cooked);
                let ty = &field.ty;

                quote!(#crate_root::__derive::field_ser::<#ty, _, _, _>(state, #lit)?;)
            })
            .collect();

        fields
    }

    let stmts = get_field_ser_stmts(crate_root, fields);
    let stmts_len = stmts.len();

    quote!(
        impl #crate_root::__derive::PropertiesMapping for #props_ty {
            fn props_len() -> usize { #stmts_len }

            fn serialize_props<S>(state: &mut S) -> ::std::result::Result<(), S::Error>
                where S: #crate_root::__derive::SerializeStruct {
                #(#stmts)*
                Ok(())
            }
        }
    )
}

fn get_ser_field(field: &syn::Field) -> Option<(syn::Ident, &syn::Field)> {
    let ctxt = serde_derive_internals::Ctxt::new();
    let serde_field = serde_attr::Field::from_ast(&ctxt, 0, field);

    // If the `serde` parse fails, return `None` and let `serde` panic later
    match ctxt.check() {
        Err(_) => return None,
        _ => (),
    };

    // Get all fields on struct where there isn't `skip_serializing`
    if serde_field.skip_serializing() {
        return None;
    }

    Some((
        syn::Ident::from(serde_field.name().serialize_name().as_ref()),
        field,
    ))
}

quick_error! {
    #[derive(Debug)]
    pub enum DeriveElasticTypeError {
        InvalidInput {
            display("deriving a document type is only valid for structs")
        }
    }
}

enum MethodFromStruct {
    Literal(Tokens),
    Expr(Tokens),
}

enum MethodFromField {
    Field(Tokens),
    Literal(Tokens, Tokens),
    Expr(Tokens, Tokens),
}

// Get the mapping ident supplied by an #[elastic()] attribute or create a default one
// Parses #[elastic(method = $lit)]
// Parses #[elastic(method(expr = $expr))]
fn get_method_from_struct(item: &syn::MacroInput, method: &str) -> Option<MethodFromStruct> {
    let val = get_elastic_meta_items(&item.attrs);

    // Attempt to get a literal
    if let Some(lit) = val
        .iter()
        .filter_map(|meta| expect_name_value(method, meta))
        .next()
    {
        return Some(MethodFromStruct::Literal(quote!(#lit)));
    }

    if let Some(expr) = val
        .iter()
        .filter_map(|meta| expect_list(method, meta))
        .flat_map(|attrs| attrs)
        .filter_map(|meta| expect_name_value("expr", meta))
        .next()
        .and_then(|expr| get_tokens_from_lit(expr).ok())
    {
        return Some(MethodFromStruct::Expr(quote!(#expr)));
    }

    None
}

fn get_method_from_fields(
    fields: &[(syn::Ident, &syn::Field)],
    method: &str,
) -> Option<MethodFromField> {
    for &(_, ref field) in fields {
        let val = get_elastic_meta_items(&field.attrs);
        let field = &field.ident;

        // Return the field name for `#[method]`
        if val.iter().any(|meta| expect_ident(method, meta)) {
            return Some(MethodFromField::Field(quote!(#field)));
        }

        // Return the literal value for `#[method = literal]`
        if let Some(lit) = val
            .iter()
            .filter_map(|meta| expect_name_value(method, meta))
            .next()
        {
            return Some(MethodFromField::Literal(quote!(#field), quote!(#lit)));
        }

        // Return the expr value for `#[method(expr = expr)]`
        if let Some(expr) = val
            .iter()
            .filter_map(|meta| expect_list(method, meta))
            .flat_map(|attrs| attrs)
            .filter_map(|meta| expect_name_value("expr", meta))
            .next()
            .and_then(|expr| get_tokens_from_lit(expr).ok())
        {
            return Some(MethodFromField::Expr(quote!(#field), quote!(#expr)));
        }
    }

    None
}
