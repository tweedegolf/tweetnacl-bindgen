pub fn crypto_hash_sha512_tweet(data: &[u8]) -> [u8; 64] {
    use std::mem::MaybeUninit;

    let mut result = MaybeUninit::uninit();

    unsafe {
        tweetnacl_bindgen::bindings::crypto_hash_sha512_tweet(
            result.as_mut_ptr() as *mut _,
            data.as_ptr(),
            data.len() as _,
        );

        result.assume_init()
    }
}

fn main() {
    println!("output: {:?}", crypto_hash_sha512_tweet(b"foobarbaz"));
}
