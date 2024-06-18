pub mod crypto;
pub mod primes;

use crate::{
    crypto::{decrypt, encrypt, generate_crypt_exponent, generate_decrypt_exponent, phi},
    primes::generate_prime_number,
};

fn main() {
    let limit = 1_000_000;
    let p = generate_prime_number(limit).unwrap();
    let q = generate_prime_number(limit * 2).unwrap();
    let n = p * q;

    let euler = phi(p, q);
    let e = generate_crypt_exponent(euler);
    let d = generate_decrypt_exponent(e, euler).unwrap();

    let public_key = (n, e);
    let private_key = (n, d);

    let message = "hello world";
    let encrypted_message = encrypt(message, public_key);

    println!("Mensaje encriptado:");
    for &ch in &encrypted_message {
        print!("{} ", ch);
    }
    println!();

    let decrypted_message = decrypt(&encrypted_message, private_key);
    println!("Mensaje desencriptado: {}", decrypted_message);
}
