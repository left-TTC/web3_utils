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

    let expanded = quote! {
        /// Pre-computed central state PDA
        pub const CENTRAL_STATE_PDA: anchor_lang::solana_program::pubkey::Pubkey = 
            anchor_lang::solana_program::pubkey::Pubkey::new_from_array(#central_state);
            
        /// Nonce for central state PDA
        pub const CENTRAL_STATE_NONCE: u8 = #nonce;
        
        /// Program ID bytes
        pub const PROGRAM_ID_BYTES: [u8; 32] = [#(#program_id.to_bytes()),*];
    };

    TokenStream::from(expanded)
}


