// mod slow_palindrome_prods;
// use slow_palindrome_prods::palindrome_products;
// mod isogram;
// use isogram::check_bits;
// mod permutations;
// use permutations::find_permutations;
// mod repeated_letter;
// use repeated_letter::first_repeated;
// mod accumulate;
// mod luhn;
// mod panagram;
// mod phone_number;
// mod allergies;
// mod largest_series_product;
// mod luhn_trait;
// mod luhn_trait2;
// mod matrix;
// mod pig_latin;
// mod proverb;
// mod perfect_numbers;
// mod run_len_encod;
// mod pythagoras_triplet;
// mod saddle_point;
// mod sieve;
// mod tournament;
// mod vec;
// use luhn::is_valid;
// mod array_duplicates;
// mod bitset;
// mod bitset_efficient;
// mod knapsack;
// mod simple_linkedlist;
// mod test;
// mod word_counts;
// mod atomic_and_locks;
// mod convert_bases;
// mod bin_search;
// mod greep;
// mod desimal;
// mod atbash_cipher;
// mod decimal;
// mod hamming;
// mod nucl_count;
// mod scrabble_points;
// mod wordy;
// mod pig_latin;
// mod say;
// mod etl;
// mod lc_sort_nums;
// mod roman_numerals;
mod remove_duplicates;

// use atomic_and_locks::Mutex;
// use atomic_and_locks::RwLock;

// use std::collections::HashSet;

//use bitset::Bitset;
// use array_duplicates::is_array_duplicated;
// use bitset::Bitset;
// use knapsack::{maximum_value_dp, Item};
// use simple_linkedlist::SimpleLinkedList;
// use test::run_thread;
// use word_counts::word_count;
// use atomic_and_locks::SpinLock;
// use atomic_and_locks::{run_channels, run_channels_v2};
// use bin_search::find_in_arr;
// use accumulate::*;
// use atomic_and_locks::Carton;
// use greep::{grep, Flags};
// use luhn::count_valid_luhns;
// use panagram::is_pangram;
// use phone_number::number;
// use sieve::primes_up_to;
// use vec::*;
// use largest_series_product::lsp;
// use matrix::Matrix;//
// use luhn_trait2::*;
// use proverb::build_proverb;
// use tournament::{Game, tally};
// use allergies::Allergies;
// use pig_latin::translate;
// use perfect_numbers::classify;
// use saddle_point::find_saddle_points;
// use convert_bases::convert;
// use run_len_encod::{decode, encode}
// use pythagoras_triplet::find;
// use roman_numerals::Roman;
// use desimal::Decimal;
// use atbash_cipher::{decode, encode};
// use decimal::Decimal;
// use hamming::hamming_distance;
// use nucl_count::*;
// use scrabble_points::score;
// use say::encode;

fn main() {
    // for v in [12, 2322, 23443, 1233211, 8932343] {
    //     assert_eq!(Palindrome::new(v), None);
    // }
    // println!("{:?}", palindrome_products(1, 9));
    // count_down(9);
    // println!("{:?}", first_repeated("bibewoweopp"));
    // println!("{:?}", first_repeated("bbibewoweopp")); //wewe
    // println!("{:?}", first_repeated("wewertt"));
    // println!("{:?}", first_repeated("wewe"));
    // println!("PRODS:: {:?}", find_permutations(&mut [1, 2, 3]));
    // println!("PRODS:: {:?}", find_permutations(&mut [1, 3]));
    // println!("{:?}", call_modifier());
    // println!("{:?}", palindrome_products(1, 10));
    // println!("{:?}", palindrome_products(10, 99));
    // println!("{:?}", palindrome_products(1000, 1));
    // println!("MIN::1002001 -> {:?}", palindrome_products(1000, 9999));
    // println!("{:?}", palindrome_products(1002, 1003));
    // println!("MIN::10201 -> {:?}", palindrome_products(100, 999));
    // println!("is_iso_gram(lumberjacks) ->: {}", check_bits("lumberjacks"));
    // println!("{:?}", is_valid("4539 3195 0343 6467"));
    // println!("{:?}", is_valid("059"));
    // println!("{:?}", is_valid_id("059"));
    // println!("{:?}", is_valid_id("4539 3195 0343 6467"));
    // println!("{:?}", is_valid_id("00")); //055a 444 285
    // let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    // list.push(1);
    // list.push(2);
    // list.push(3);
    // list.push(4);
    // list.push(5);
    // println!("LIST 1: {:?}", list.len());
    // println!("LIST B-POP: {:?}", list);
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.peek());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("POP: {:?}", list.pop());
    // println!("LIST A-POP: {:?}", list);
    // let rev_list = list.rev();
    // println!("LIST REV: {:?}", rev_list);
    // println!("IsEmpty: {:?}", is_empty());
    // let mut array = vec!["1", "2", "3", "4"];
    // let mut list: SimpleLinkedList<_> = array.drain(..).collect();
    // println!("LIST: {:?}", list);
    // let list = list.rev();
    // println!("REV-LIST: {:?}", list);
    // list.pop();
    // list.pop();
    // list.pop();
    // list.pop();
    // println!("LIST: {:?}", list.pop());
    // println!("LIST: {:?}", list.pop());
    // println!("LIST: {:?}", list.pop());
    // println!("LIST: {:?}", list.pop());
    // println!("LIST: {:?}", list.pop());
    // let mut bitset = Bitset::new(100000);
    // bitset.fix(1);
    // bitset.unfix(2);
    // bitset.fix(1);
    // bitset.flip();
    // bitset.fix(1);
    // bitset.unfix(0);
    // bitset.unfix(0);
    // bitset.fix(20);
    // println!("Bitset: {:?}", bitset.to_string());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // println!("Bitset: {:?}", bitset.one());
    // bitset.flip();
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // println!("Bitset: {:?}", bitset.all());
    // bitset.flip();
    // println!("Bitset: {:?}", bitset.count());
    // println!("Bitset: {:?}", "password's".strip_prefix(['\'']));
    // let max_weight = 10;
    // let items = [
    //     Item {
    //         weight: 2,
    //         value: 5,
    //     },
    //     Item {
    //         weight: 2,
    //         value: 5,
    //     },
    //     Item {
    //         weight: 2,
    //         value: 5,
    //     },
    //     Item {
    //         weight: 2,
    //         value: 5,
    //     },
    //     Item {
    //         weight: 10,
    //         value: 21,
    //     },
    // ];
    // println!("Max Profit: {:?}", maximum_value_dp(max_weight, &items));
    // let max_weight = 10;
    // let items = [
    //     Item {
    //         weight: 2,
    //         value: 20,
    //     },
    //     Item {
    //         weight: 2,
    //         value: 20,
    //     },
    //     Item {
    //         weight: 2,
    //         value: 20,
    //     },
    //     Item {
    //         weight: 2,
    //         value: 20,
    //     },
    //     Item {
    //         weight: 10,
    //         value: 50,
    //     },
    // ];
    // println!("Max Profit: {:?}", maximum_value_dp(max_weight, &items));
    // let v = &[1,2,3,3,5];
    // let v2 = &[1,0,2,4,5,9];
    // let v3 = &[0];
    // let v4 = &[1];
    // println!("{:?}", is_array_duplicated(v));
    // println!("{:?}", is_array_duplicated(v2));
    // println!("{:?}", is_array_duplicated(v3));
    // println!("{:?}", is_array_duplicated(v4));
    // println!("THREAD RES:: {:?}", t());
    // run_thread();
    // run_channels();
    // run_channels_v2();
    // let haystack = Vec::from([111, 289, 305, 443, 506, 679]);
    // println!("{:?}", find_in_arr(haystack, 443));
    // let flgs = &["-n", "-l", "-x", "-i"];
    // let flags = Flags::new(flgs);
    // println!("{:?}", flags);
    // println!("{:?}", grep("house grown", &flags, &[&"1.txt"]));
    // println!("[]: {:?}", map(vec![1,2,3], |x| x * x));

    // let c = Carton::new(45);
    // println!("{:?}", *c);

    // let x: &str = "bee";
    // let mut y = std::borrow::Cow::from(x);
    // y.to_mut().push('r');
    // println!("{:?}, {:?}", x, y);
    // println!("[+1 (613)-995-0253] -> {:?}", number("+1 (613)-995-0253"));
    // println!("[+1 (023) 456-7890] -> {:?}", number("+1 (023) 456-7890"));
    // println!("[613.995.0253] -> {:?}", number("613.995.0253"));
    // println!("[(023) 456-7890] -> {:?}", number("(023) 456-7890"));
    // println!("[1 (223) 056-7890] -> {:?}", number("1 (223) 056-7890"));
    // println!("[(123) 456-7890] -> {:?}", number("(123) 456-7890"));
    // println!("[12234567890] -> {:?}", number("12234567890"));
    // println!("[223.456.7890] -> {:?}", number("223.456.7890"));
    // println!("[1 (223) 156-7890] -> {:?}", number("1 (223) 156-7890"));
    // println!("[22234567890] -> {:?}", number("22234567890"));
    // let mut set: HashSet<u16> = HashSet::new();
    // let x = "The quick brown fox jumps over the lazy dog"
    //     .to_lowercase()
    //     .chars()
    //     .filter(|c| !c.is_ascii_whitespace())
    //     .for_each(|c| {
    //         set.insert(c as u16);
    //         ()
    //     });
    // println!("{:?}", set.iter().sum::<u16>());
    // let z = "\"Five quacking Zephyrs jolt my wax bed.\"";
    // println!("{z} is panagram: {:?}", is_pangram(z));
    // let z = "abcdefghijklmnopqrstuvwxyz";
    // println!("{z} is panagram: {:?}", is_pangram(z));
    // primes_up_to(11);
    // println!("10 PRMS: {:?}", primes_up_to(10));
    // println!("11 PRMS: {:?}", primes_up_to(11));
    // println!("13 PRMS: {:?}", primes_up_to(13));
    // println!("{}", 0.1 + 0.2);
    // println!(
    //     "{:?}",
    //     lsp("73167176531330624919225119674426574742355349194934", 6)
    // );
    // println!("{:?}", lsp("2U9", 2));
    // let luhn = Luhn::from(46_454_286u32);
    // println!("IS '46_454_286u32' LUHN -> {:?}", luhn.is_valid());
    // let luhn = Luhn::from("046 454 286");
    // println!("IS '046 454 286' LUHN -> {:?}", luhn.is_valid());
    // let luhn = Luhn::from("64_437u16");
    // println!("IS '64_437u16' LUHN -> {:?}", luhn.is_valid());
    // let matrix = Matrix::new("1 2\n10 20");
    // println!("ROW 0 -> {:?}", matrix.row(1));
    // println!("ROW 1 -> {:?}", matrix.row(2));
    // println!("COL 0 -> {:?}", matrix.column(1));
    // println!("COL 1 -> {:?}", matrix.column(2));
    // let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9\n8 7 6");
    // println!("ROW 0 -> {:?}", matrix.row(4));
    // let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
    // println!("ROW 0 -> {:?}", matrix.row(4));
    // println!("ROW 0 -> {:?}", matrix.column(4));
    //
    //println!("{:?}", build_proverb(&["pin", "gun", "soldier", "battle"]));
    // let input: &[&str] = &[
    //     "Allegoric Alaskans;Blithering Badgers;win",
    //     "Allegoric Alaskans;Blithering Badgers;win",
    //     "Allegoric Alaskans;Blithering Badgers;draw",
    //     "Allegoric Alaskans;Blithering Badgers;loss",
    // ];

    // let output = tally(&input.join("\n"));
    // for i in output.lines() {
    //     println!("{:?}", i)
    // }
    // let mut th_wpadding = String::from("                           ");
    // th_wpadding.extend(["Team"]).pad;
    // println!("WON: {:?}", assert_eq!(Game::Win, Game::Win));
    // let input = &[vec![9, 8, 7], vec![3, 5, 2], vec![6, 6, 7]];
    // for i in input {
    //     println!("{:?}", i)
    // }
    // find_saddle_points(input);
    // println!("{:?}", String::from("").valid_luhn());
    // println!("{:?}", 240u8.valid_luhn());
    // println!("{:?}", 64_436u16.valid_luhn());
    // println!("{:?}", 241u8.valid_luhn());
    // println!("{:?}", 64_437u16.valid_luhn())
    // let allergies = Allergies::new(257).allergies();
    // println!("ALLERGY {:?}", allergies);
    // let allergies = Allergies::new(255).allergies();
    // println!("ALLERGY {:?}", allergies);
    // let allergies = Allergies::new(509).allergies();
    // println!("ALLERGY {:?}", allergies);
    // let allergies = Allergies::new(3).allergies();
    // println!("ALLERGY {:?}", allergies);
    // let allergies = Allergies::new(248).allergies();
    // println!("ALLERGY {:?}", allergies);
    //
    // let x = &[vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    // find_saddle_points(x);

    // let input = &[vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    // println!("COORDS [()]: -> {:?}", find_saddle_points(input));

    // let input = &[vec![4, 5, 4], vec![3, 5, 5], vec![1, 5, 4]];
    // println!(
    //     "COORDS [(0, 1), (1, 1), (2, 1)]: -> {:?}",
    //     find_saddle_points(input)
    // );

    // let input = &[vec![6, 7, 8], vec![5, 5, 5], vec![7, 5, 6]];
    // println!("COORDS [(1, 0), (1, 1), (1, 2)]: -> {:?}", find_saddle_points(input));

    // // let input = &[vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
    // // find_saddle_points(input);

    // let inp = &[vec![3, 1, 3], vec![3, 2, 4]];
    // println!("COORDS [(0, 0), (0, 2)]: -> {:?}", find_saddle_points(inp));

    // let input = &[vec![]];
    // println!("COORDS []: -> {:?}", find_saddle_points(input));
    //
    // classify(28);
    // convert(&[1, 0, 1], 2, 10);
    // println!("ENCODED -> {:?}", encode("AABCCCDEEEE"));
    // println!(
    //     "ENCODED -> {:?}",
    //     encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB")
    // );
    // println!("DECODED -> {:?}", decode("12WB12W3B24WB"));
    // println!("ECODED -> {:?}", encode("XYZ"));
    // println!("FIND 108 -> {:?}", find(108));
    // println!("FIND 1000 -> {:?}", find(1000));
    // println!("1996 => (MCMXCVI): {:?}", Roman::parts_num(1996));
    // println!("1992 => (MCMXCII): {:?}", Roman::parts_num(1992));
    // println!("1992 => (MCMXCII): {:?}", Roman::parts_num(1993));
    // println!("1992 => (MCMXCII): {:?}", Roman::from(1992).to_string());
    // println!("1892 => (MCMLXXXII): {:?}", Roman::from(1982).to_string());
    // println!("3549 => (MMMDXLIX): {:?}", Roman::from(3549).to_string());
    // println!("549 => (DXLIX): {:?}", Roman::from(549).to_string());
    // let decimal: Decimal<f64> = Decimal::try_from("0.11").ok().unwrap();
    // let dec1: Decimal<f64> = Decimal::try_from(
    //     "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
    // )
    // .ok()
    // .unwrap();
    // let dec2: Decimal<f64> = Decimal::try_from(
    //     "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
    // )
    // .ok()
    // .unwrap();
    // println!("D: {:?}", dec1 + dec2);
    //
    // fn decimal(input: &str) -> Decimal {
    //     Decimal::try_from(input).expect("That was supposed to be a valid value")
    // }
    // let dec1 = decimal(
    //     "100000000000000000000000000000000000000000000.000000000000000000000000000000000000000000001",
    // );
    // let dec2 = decimal(
    //     "100000000000000000000000000000000000000000000.000000000000000000000000000000000000000000002",
    // );
    // println!("Addi -> [e132e] {:?}", dec1 + dec2);
    // println!(
    //     "Addi -> [0.0099]: {:?}",
    //     decimal("0.01") + decimal("-0.0001")
    // );
    // println!("Addi -> [1.0]: {:?}", decimal("1.1") + decimal("-0.1"));
    // println!("Addi -> [0.991]: {:?}", decimal("1.001") + decimal("-0.01"));
    // println!("Addi -> [0.01]: {:?}", decimal("0.011") + decimal("-0.001"));
    // println!("Addi -> [1.0]: {:?}", decimal("1.1") + decimal("-0.1"));
    // println!("Addi -> [0.99]: {:?}", decimal("1.0") + decimal("-0.01"));
    // println!("Addi -> [-2.0]: {:?}", decimal("-1.99") + decimal("-0.01"));
    // println!("Addi -> [-2.01]: {:?}", decimal("-1.99") + decimal("-0.02"));
    // println!(
    //     "ENCODE [thequickbrownfoxjumpsoverthelazydog]: {:?}",
    //     decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt")
    // );
    // println!("ENCODE [gvhg]: {:?}", decode("test"));
    // println!(
    //     "ENCODE [testing123testing]: {:?}",
    //     decode("gvhgr mt123 gvhgr mt")
    // );
    // println!(
    //     "HAMMING: {:?}",
    //     hamming_distance("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT")
    // );
    // println!("C: {:?}", count('X', "A"));
    //The quick brown fox jumps over the lazy dog.
    // println!("WORD: {:?}", encode(6000192));
    // println!("WORD: {:?}", encode(514521));
    // println!("WORD: {:?}", encode(4521));
    // println!("WORD: {:?}", encode(1234567890));
    // let x = input_from(&[(1, vec!['a', 'b'])]);
    // let y = input_from(&[
    //     (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
    //     (2, vec!['D', 'G']),
    //     (3, vec!['B', 'C', 'M', 'P']),
    //     (4, vec!['F', 'H', 'V', 'W', 'Y']),
    //     (5, vec!['K']),
    //     (8, vec!['J', 'X']),
    //     (10, vec!['Q', 'Z']),
    // ]);
    // etl::transform(&x);
    // etl::transform(&y);
    // println!(
    //     "RES: -> [Phenylalanine, Phenylalanine];; {:?}",
    //     rna::translate("UUCUUCUAAUGGU")
    // );
    // println!(
    //     "RES: -> [Methionine, Phenylalanine, Serine];; {:?}",
    //     rna::translate("AUGUUUUCU")
    // );
    println!(
        "RES: -> [None] {:?}",
        remove_duplicates::remove_duplicates(&mut Vec::from([1, 1, 1, 2, 2, 3, 4, 4, 4]))
    );
    //
    // advent_late::main();
}
// use std::collections::BTreeMap;
// fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
//     v.iter().cloned().collect()
// }

// -1.99
// -0.01
// ------
// 0.
