use anchor_lang::solana_program::hash::hashv;
use anchor_lang::prelude::*;

pub const HASH_PREFIX: &str = "WEB3 Name Service";

pub fn get_hashed_name(name: &str) -> Vec<u8>{
    hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
        .as_ref()
        .to_vec()
}


pub fn get_PDA_key(
    program_id: &Pubkey,
    hashed_name: Vec<u8>,
    root_opt: Option<&Pubkey>,
) -> (Pubkey, Vec<u8>) {        
    let mut seeds_vec: Vec<u8> = hashed_name;

    //root domain(when create a root domian,use default)
    let root_domian = root_opt.cloned().unwrap_or_default();
    //add root to the seed
    for b in root_domian.to_bytes() {
        seeds_vec.push(b);
    }

    let (name_account_key, bump) =
        Pubkey::find_program_address(&seeds_vec.chunks(32).collect::<Vec<&[u8]>>(), program_id);
    seeds_vec.push(bump);

    (name_account_key, seeds_vec)
}


