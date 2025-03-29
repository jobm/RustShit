pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins: Vec<&str> = vec![];
    rna.chars()
        .collect::<Vec<_>>()
        .as_slice()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .take_while(|codon| {
            match codon.as_str() {
                "AUG" => proteins.push("Methionine"),
                "UGG" => proteins.push("Tryptophan"),
                "UUA" | "UUG" => proteins.push("Leucine"),
                "UAU" | "UAC" => proteins.push("Tyrosine"),
                "UGU" | "UGC" => proteins.push("Cysteine"),
                "UUU" | "UUC" => proteins.push("Phenylalanine"),
                "UCU" | "UCC" | "UCA" | "UCG" => proteins.push("Serine"),
                "UAA" | "UAG" | "UGA" => {
                    return false;
                }
                _ => {
                    return false;
                }
            }
            true
        })
        .for_each(drop);
    if proteins.is_empty() {
        None
    } else {
        Some(proteins)
    }
}

// // Codon	Protein
// // AUG	Methionine -
// // UUU, UUC	Phenylalanine -
// // UUA, UUG	Leucine -
// // UCU, UCC, UCA, UCG	Serine -
// // UAU, UAC	Tyrosine
// // UGU, UGC	Cysteine
// // UGG	Tryptophan
// // UAA, UAG, UGA	STOP
