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
/// use discretionary::make_optional;
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
        syn::Fields::Named(ref mut fields) => fields.named.iter_mut().for_each(|field| {
            let orig_ty = &field.ty;
            field.ty = syn::Type::Verbatim(quote!(Option<#orig_ty>));
        }),
        syn::Fields::Unnamed(ref mut fields) => fields.unnamed.iter_mut().for_each(|field| {
            let orig_ty = &field.ty;
            field.ty = syn::Type::Verbatim(quote!(Option<#orig_ty>));
        }),
        syn::Fields::Unit => {}
    }
    quote! {
        #obj
    }
}
