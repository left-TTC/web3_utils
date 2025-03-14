use proc_macro::TokenStream;
use quote::quote;
use anchor_lang::solana_program::pubkey::Pubkey;
use syn::{parse_macro_input, LitStr};
use std::str::FromStr;


#[proc_macro]
pub fn declare_central_state(input: TokenStream) -> TokenStream {
    let program_id_str = parse_macro_input!(input as LitStr);
    let program_id = program_id_str.value();

    let pubkey = Pubkey::from_str(&program_id).expect("Invalid program ID");
    let pubkey_bytes = pubkey.to_bytes();

    let (central_state, central_state_nonce) = Pubkey::find_program_address(&[&pubkey_bytes], &pubkey);
    let central_state_bytes = central_state.to_bytes();

    let expanded = quote! {
        pub mod central_state {
            use anchor_lang::solana_program::pubkey::Pubkey;

            pub static KEY_BYTES: [u8; 32] = [#(#central_state_bytes),*];
            pub static KEY: Pubkey = Pubkey::new_from_array(KEY_BYTES);
            pub static NONCE: u8 = #central_state_nonce;
            pub static SIGNER_SEEDS: [&'static [u8]; 2] = [&super::ID_BYTES, &[NONCE]];
        }

        pub static ID_BYTES: [u8; 32] = [#(#pubkey_bytes),*];
    };

    TokenStream::from(expanded)
}


