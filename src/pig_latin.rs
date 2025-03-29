pub fn translate(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let chars: Vec<char> = input.chars().collect();

    if chars.len() >= 2 && (input.starts_with("xr") || input.starts_with("yt")) {
        // Rule 1: Starts with "xr" or "yt"
        format!("{}ay", input)
    } else if vowels.contains(&chars[0]) {
        // Rule 1: Starts with a vowel
        format!("{}ay", input)
    } else {
        let mut consonant_cluster_end = 0;

        // Find the first vowel or 'y' after consonants
        for (i, &c) in chars.iter().enumerate() {
            if vowels.contains(&c) || (c == 'y' && i != 0) {
                break;
            }
            consonant_cluster_end = i + 1;
        }

        if consonant_cluster_end > 0
            && chars[consonant_cluster_end - 1] == 'q'
            && chars.get(consonant_cluster_end) == Some(&'u')
        {
            // Rule 3: Handle "qu" sequence
            consonant_cluster_end += 1;
        }

        // Split and reassemble the word according to Pig Latin rules
        let prefix: String = chars[..consonant_cluster_end].iter().collect();
        let suffix: String = chars[consonant_cluster_end..].iter().collect();
        format!("{}{}ay", suffix, prefix)
    }
}
