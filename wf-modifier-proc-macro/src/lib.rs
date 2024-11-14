use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, spanned::Spanned, ItemImpl};

/// This is a procedural macro that fills in the missing implementations of the `Modifier` for
/// the implementor of the `Modifier`.
///
/// The missing implementations are filled in with `Default::default()`.
#[proc_macro_attribute]
pub fn modifier(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl: ItemImpl = parse(item).expect("expected an item implementation");

    // Get the implemented functions of the item implementation.
    let implemented_functions = item_impl.items.iter().filter_map(|item| {
        if let syn::ImplItem::Fn(method) = item {
            Some(&method.sig.ident)
        } else {
            None
        }
    });

    // Get the functions that are required to be implemented by the item implementation.
    let required_functions = vec![
        ("damage", quote!(f32)),
        ("anti_faction", quote!(f32)),
        ("critical_chance", quote!(f32)),
        ("critical_multiplier", quote!(f32)),
        ("status_chance", quote!(f32)),
        ("attack_speed", quote!(f32)),
        ("fire_rate", quote!(f32)),
        ("status_list", quote!(Vec<Status>)),
        ("ammo_maximum", quote!(f32)),
        ("magazine_capacity", quote!(f32)),
        ("multishot", quote!(f32)),
        ("reload_speed", quote!(f32)),
        ("cost", quote!(u8)),
    ];

    // Get the missing functions that are required to be implemented by the item implementation.
    let missing_functions = required_functions.iter().filter(|required_function| {
        !implemented_functions
            .clone()
            .any(|implemented_function| implemented_function == required_function.0)
    });

    // Generate the default implementations for the missing functions.
    let default_implementations = missing_functions.map(|missing_function| {
        let function_name = syn::Ident::new(&missing_function.0, missing_function.0.span());
        let function_return_type = &missing_function.1;

        quote! {
            fn #function_name(&self, _context: &dyn Weapon) -> #function_return_type {
                Default::default()
            }
        }
    });

    // Generate the existing implementations of the item implementation.
    let existing_implementations = item_impl.items.iter().filter_map(|item| {
        if let syn::ImplItem::Fn(method) = item {
            Some(quote! {
                #method
            })
        } else {
            None
        }
    });

    // Generate the final implementation of the item implementation with the default implementations.
    let item_impl_ident = &item_impl.self_ty;
    let final_item_impl = quote! {
        impl Modifier for #item_impl_ident {
            #(#existing_implementations)*
            #(#default_implementations)*
        }
    };

    // panic!("{:?}", final_item_impl.to_string());

    final_item_impl.into()
}
