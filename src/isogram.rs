use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    candidate
        .bytes()
        .filter_map(|c| c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase()))
        .all(|c| seen.insert(c))
}

pub fn check_bits(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - 97))
        .try_fold(0u32, |ltr_flags, ltr| {
            println!("LTR:: {}, LTR_FLAGS:: {}", ltr, ltr_flags);
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
