use quote::quote;
use syn::{AttributeArgs, ItemStruct, NestedMeta};

/// Processes a struct to convert the fields to `Option`s
///
/// For now, this will always convert _all_ field types to `Option`, but the
/// goal is to someday add filtering for skipping some fields. The usage is
/// straightforward: just decorate your `struct` with `#[make_optional]`. For
/// example, the following
///
/// ```
/// use adsabs_macro::make_optional;
///
/// #[make_optional]
/// struct ExampleStruct {
///     id: usize,
///     name: String,
/// }
/// ```
///
/// will be re-written to something like
///
/// ```
/// struct ExampleStruct {
///     id: Option<usize>,
///     name: Option<String>,
/// }
/// ```
#[proc_macro_attribute]
pub fn make_optional(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);
    let mut input = syn::parse_macro_input!(input as ItemStruct);
    impl_make_optional(&args, &mut input).into()
}

fn impl_make_optional(_args: &[NestedMeta], obj: &mut ItemStruct) -> proc_macro2::TokenStream {
    match obj.fields {
        syn::Fields::Named(ref mut fields) => fields.named.iter_mut().for_each(update_field),
        syn::Fields::Unnamed(ref mut fields) => fields.unnamed.iter_mut().for_each(update_field),
        syn::Fields::Unit => {}
    }
    quote! {
        #obj
    }
}

fn update_field(field: &mut syn::Field) {
    // Add skip_serializing_if for serde
    let attr = syn::parse_quote!(
        #[serde(skip_serializing_if = "Option::is_none")]
    );
    field.attrs.push(attr);

    // Update the field to be an Option
    let orig_ty = &field.ty;
    field.ty = syn::Type::Verbatim(quote!(Option<#orig_ty>));
}
