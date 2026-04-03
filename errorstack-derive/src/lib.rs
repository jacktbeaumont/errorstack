use proc_macro::TokenStream;

/// Derive macro for [`ErrorStack`].
///
/// See the `errorstack` crate documentation for usage.
#[proc_macro_derive(ErrorStack, attributes(source, stack_source, location))]
pub fn derive_error_stack(input: TokenStream) -> TokenStream {
    let _ = input;
    TokenStream::new()
}
