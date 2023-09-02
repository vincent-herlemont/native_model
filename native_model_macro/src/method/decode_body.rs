use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_native_model_decode_body() -> TokenStream {
    let gen = quote! {
        fn native_model_decode_body(data: Vec<u8>) -> Result<Self, native_model::DecodeBodyError> {
            native_model_decode_body(data).map_err(|e| native_model::DecodeBodyError {
                msg: format!("{}", e),
                source: e.into(),
            })
        }
    };

    gen.into()
}