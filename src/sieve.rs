use std::u64;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut p: u64 = 2;
    let ints = &mut Vec::new();
    for n in 2..=upper_bound {
        ints.push(n);
    }
    for (idx, n) in ints.clone().iter().enumerate() {
        if p.pow(2) > (upper_bound - 1) as u64 {
            break;
        }
        for i in idx + 1..=ints.clone().len() {
            let curr = ints.get_mut(i as usize);
            if curr.is_none() {
                continue;
            }
            if **curr.as_ref().unwrap() % p == 0 {
                *curr.unwrap() = 0;
            }
        }
        if is_prime(*n) {
            p = *n;
        }
    }
    let mut primes = ints.clone();
    primes.retain(|&p| p != 0);
    primes
}

fn is_prime(n: u64) -> bool {
    for i in (2..n).rev() {
        if n % i == 0 {
            return false;
        }
    }
    true
}
