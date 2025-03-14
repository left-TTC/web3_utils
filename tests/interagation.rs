use left_utils::declare_central_state;

declare_central_state!("nCebN34bUfdeUYJT13J1yG16XWQpt55Dx6MseoGUqhR");

#[test]
fn test_central_state() {
    print!("KEY is {}",&central_state::KEY);
    print!("NONCE is {}",&central_state::NONCE);
}