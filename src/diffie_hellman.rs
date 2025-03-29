pub fn private_key(mut p: u64) -> u64 {
    p = p - 1;
    if p <= 1 {
        p = 2;
    }
    p
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

pub fn modular_pow(mut base: u64, mut exponent: u64, modulo: u64) -> u64 {
    if modulo == 1 {
        return 0;
    }
    let mut result = 1;
    base = base.rem_euclid(modulo);
    while exponent > 0 {
        if exponent.rem_euclid(2) == 1 {
            result = (result * base).rem_euclid(modulo);
        }
        exponent = exponent >> 1;
        base = (base * base).rem_euclid(modulo);
    }
    result
}

// println!("HASH -> {:?}", diffie_helllman::private_key(5));
// println!(
//     "Public Key:: -> 8 = {:?}",
//     diffie_hellman::public_key(23, 5, 6)
// );
// println!(
//     "Public Key:: -> 4096 = {:?}",
//     diffie_hellman::public_key(4_294_967_299, 8, 4_294_967_296)
// );
