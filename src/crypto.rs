use crate::primes::{gcd, gcd_extended};

pub fn mod_exp(base: i64, key: (i64, i64)) -> i64 {
    let modulus = key.0 as i128;
    if modulus == 1 {
        return 0;
    }
    let mut result = 1i128;
    let mut base = base as i128 % modulus;
    let mut exp = key.1 as i128;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result as i64
}

pub fn phi(p: i64, q: i64) -> i64 {
    (p - 1) * (q - 1)
}

pub fn mod_inverse(e: i64, phi: i64) -> Option<i64> {
    let (gcd, x, _) = gcd_extended(e, phi);
    if gcd != 1 {
        None
    } else {
        let d = (x % phi + phi) % phi;
        Some(d)
    }
}

pub fn generate_crypt_exponent(phi: i64) -> i64 {
    for e in 2..phi {
        if gcd(e, phi) == 1 {
            return e;
        }
    }
    panic!("No se encontró un exponente adecuado para el valor de phi proporcionado.");
}

pub fn generate_decrypt_exponent(e: i64, phi: i64) -> Option<i64> {
    mod_inverse(e, phi)
}

pub fn encrypt(message: &str, public_key: (i64, i64)) -> Vec<i64> {
    let (n, e) = public_key;
    let mut encrypted = Vec::new();

    for ch in message.bytes() {
        let ch_i64 = ch as i64;
        let encrypted_ch = mod_exp(ch_i64, (n, e));
        encrypted.push(encrypted_ch);
    }

    encrypted
}

pub fn decrypt(encrypted: &[i64], private_key: (i64, i64)) -> String {
    let (n, d) = private_key;
    let mut decrypted = String::new();

    for &enc_ch in encrypted {
        let decrypted_ch = mod_exp(enc_ch, (n, d));
        decrypted.push(decrypted_ch as u8 as char);
    }

    decrypted
}
