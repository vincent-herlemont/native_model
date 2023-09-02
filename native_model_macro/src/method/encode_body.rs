use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_native_model_encode_body() -> TokenStream {
    let gen = quote! {
        fn native_model_encode_body(&self) -> Result<Vec<u8>, native_model::EncodeBodyError> {
            native_model_encode_body(self).map_err(|e| native_model::EncodeBodyError {
                msg: format!("{}", e),
                source: e.into(),
            })
        }
    };

    gen.into()
}