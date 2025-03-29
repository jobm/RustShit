use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::<char, usize>::new();
    }
    let word_count = Arc::new(Mutex::new(HashMap::<char, usize>::new()));
    let chunked_inputs = input.chunks((input.len()).div_ceil(worker_count));
    thread::scope(|s| {
        for chunk_input in chunked_inputs {
            s.spawn(|| {
                let word_count = Arc::clone(&word_count);
                chunk_input.iter().for_each(|word| {
                    word.to_lowercase()
                        .chars()
                        .filter(|char_| !char_.is_ascii_punctuation() && !char_.is_ascii_digit())
                        .for_each(|char_| {
                            let mut word_count = word_count.lock().unwrap();
                            if word_count.get(&char_).is_none() {
                                word_count.insert(char_, 1);
                            } else {
                                *word_count.get_mut(&char_).unwrap() += 1;
                            }
                        })
                });
            });
        }
    });
    return word_count.lock().unwrap().to_owned();
}

// func input
// let v = &["abcd"; 100];
// let y = &[
//     "aiba",
//     "aaaak",
//     "dsdstrtkf",
//     "bdstkf",
//     "aiba",
//     "aaaak",
//     "dsdstrtkf",
//     "bdstkfüüü",
//     "82uUopü",
// ];
// // v.insert("1213");
// println!("WC:: {:?}", frequency(v, 2));
// println!("WC:: {:?}", frequency(y, 1));

// const ODE_AN_DIE_FREUDE: [&str; 8] = [
//     "Freude schöner Götterfunken",
//     "Tochter aus Elysium,",
//     "Wir betreten feuertrunken,",
//     "Himmlische, dein Heiligtum!",
//     "Deine Zauber binden wieder",
//     "Was die Mode streng geteilt;",
//     "Alle Menschen werden Brüder,",
//     "Wo dein sanfter Flügel weilt.",
// ];
// Dutch national anthem
// const WILHELMUS: [&str; 8] = [
//     "Wilhelmus van Nassouwe",
//     "ben ik, van Duitsen bloed,",
//     "den vaderland getrouwe",
//     "blijf ik tot in den dood.",
//     "Een Prinse van Oranje",
//     "ben ik, vrij, onverveerd,",
//     "den Koning van Hispanje",
//     "heb ik altijd geëerd.",
// ];
// American national anthem
// const STAR_SPANGLED_BANNER: [&str; 8] = [
//     "O say can you see by the dawn's early light,",
//     "What so proudly we hailed at the twilight's last gleaming,",
//     "Whose broad stripes and bright stars through the perilous fight,",
//     "O'er the ramparts we watched, were so gallantly streaming?",
//     "And the rockets' red glare, the bombs bursting in air,",
//     "Gave proof through the night that our flag was still there;",
//     "O say does that star-spangled banner yet wave,",
//     "O'er the land of the free and the home of the brave?",
// ];
// let mut v = Vec::new();
// for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
//     for line in anthem.iter() {
//         v.push(*line);
//     }
// }
// let freqs = freq::frequency(&v[..], 7);
// println!("WC:: {:?}", freqs);
// println!("WC:: a -> {:?}", freqs.get(&'a'));
// println!("WC:: t -> {:?}", freqs.get(&'t'));
// println!("WC:: ü -> {:?}", freqs.get(&'ü'));
// println!("WC:: a -> {:?}", frequency(&["aA"], 2).get(&'a'));
// println!("WC:: &[] -> {:?}", frequency(&[], 4));
