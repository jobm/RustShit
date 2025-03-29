use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub fn is_valid(code: &str) -> bool {
    if code.trim() == "0"
        || code.contains(char::is_alphabetic)
        || code.chars().any(|c| c.is_ascii_punctuation())
    {
        return false;
    }
    code.chars()
        .filter(|num| num.is_digit(10))
        .rev()
        .enumerate()
        .filter_map(|(idx, num)| {
            if idx % 2 == 1 {
                if (num.to_digit(10).unwrap() * 2) > 9 {
                    Some(num.to_digit(10).unwrap() * 2 - 9)
                } else {
                    Some(num.to_digit(10).unwrap() * 2)
                }
            } else {
                Some(num.to_digit(10).unwrap())
            }
        })
        .into_iter()
        .sum::<u32>()
        .rem_euclid(10)
        == 0
}

pub fn count_valid_luhns(mode: &str) -> u64 {
    let chunks = [
        (10_000_000..=15_000_000),
        (15_000_000..=20_000_000),
        (20_000_000..=25_000_000),
        (25_000_000..=30_000_000),
        (30_000_000..=35_000_000),
        (35_000_000..=40_000_000),
        (40_000_000..=45_000_000),
        (45_000_000..=50_000_000),
        (50_000_000..=55_000_000),
        (55_000_000..=60_000_000),
        (60_000_000..=65_000_000),
        (65_000_000..=70_000_000),
        (70_000_000..=75_000_000),
        (75_000_000..=80_000_000),
        (80_000_000..=85_000_000),
        (85_000_000..=90_000_000),
        (90_000_000..=95_000_000),
        (95_000_000..=100_000_000),
    ];
    if mode.eq("mutex") {
        let count = Arc::new(std::sync::Mutex::new(0));
        std::thread::scope(|s| {
            for chunk in chunks {
                s.spawn(|| {
                    let count: Arc<Mutex<u64>> = Arc::clone(&count);
                    let mut count = count.lock().unwrap();
                    for i in chunk {
                        if is_valid(i.to_string().as_str()) {
                            *count = count.wrapping_add(1);
                        }
                    }
                });
            }
        });
        return count.lock().unwrap().to_owned();
    } else if mode.eq("atomic") {
        let counter = AtomicU64::new(0);
        std::thread::scope(|s| {
            for chunk in chunks {
                s.spawn(|| {
                    for i in chunk {
                        if is_valid(i.to_string().as_str()) {
                            counter.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                });
            }
        });
        return counter.into_inner();
    } else {
        let mut count = 0;
        for i in 10_000_000..=100_000_000 {
            if is_valid(i.to_string().as_str()) {
                count += 1;
            }
        }
        return count;
    }
}

// Mac 2.3GHz Dual-Core; Intel core i5 8GB RAM
// Before Update
// LUHN Atomic SeqCst Count from 10M -> 100M: 9000000, took 81.903745037s
// LUHN Atomic Relaxed Count from 10M -> 100M: 9000000, took 84.897919392s
// LUHN Mutex Count [wrapping_add] from 10M -> 100M: 9000000, took 165.93981166s
// LUHN Mutex (Vec.push->then->[sum(vec) at end]) Count from 10M -> 100M: 9000000, took 166.499636825s
// LUHN Sync Count from 10M -> 100M: 9000000, took 175.75783541s
// LUHN Mutex Count [+=] from 10M -> 100M: 9000000, took 205.261413525s
//
// After update
// LUHN Atomic AcqRel Count from 10M -> 100M: 9000000, took 76.81689533s
// LUHN Atomic SeqCst Count from 10M -> 100M: 9000000, took 77.77335478s
// LUHN Atomic Relaxed Count from 10M -> 100M: 9000000, took 85.556084287s
// LUHN Mutex Count from 10M -> 100M: 9000000, took 161.433053824s
// LUHN Sync Count from 10M -> 100M: 9000000, took 163.808592942s

// AMD Ryzen 7 5700G 3.8GHZ 8 Cores * 2 (Threads) 32GB RAM
// Before Update
//
// After Update
//
