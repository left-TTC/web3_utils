use proc_macro::TokenStream;
use quote::quote;
use anchor_lang::solana_program::pubkey::Pubkey;
use syn::{parse_macro_input, LitStr};
use std::str::FromStr;


#[proc_macro]
pub fn declare_central_state(input: TokenStream) -> TokenStream {
    let program_id_str = parse_macro_input!(input as LitStr);
    let program_id = Pubkey::from_str(&program_id_str.value()).expect("Invalid program ID");
    let (central_state, nonce) = Pubkey::find_program_address(&[&program_id.to_bytes()], &program_id);

    // 转换为字节数组
    let central_state_bytes = central_state.to_bytes();
    let program_id_bytes = program_id.to_bytes();

    let expanded = quote! {
        pub mod central_state {
            use anchor_lang::solana_program::pubkey::Pubkey;

            pub const KEY_BYTES: [u8; 32] = [#(#central_state_bytes),*];
            pub const KEY: Pubkey = Pubkey::new_from_array(KEY_BYTES);
            pub const NONCE: u8 = #nonce;
            
            pub fn signer_seeds() -> [&'static [u8]; 2] {
                [&super::PROGRAM_ID_BYTES, &[NONCE]]
            }
        }

        pub const PROGRAM_ID_BYTES: [u8; 32] = [#(#program_id_bytes),*];
    };

    TokenStream::from(expanded)
}

