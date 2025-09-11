pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let amino_acids: Vec<&str> = (0..rna.len())
        .step_by(3)
        .filter_map(|i| rna.get(i..i + 3))
        .map_while(|codon| match codon {
            "AUG" => Some("Methionine"),
            "UUU" | "UUC" => Some("Phenylalanine"),
            "UUA" | "UUG" => Some("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
            "UAU" | "UAC" => Some("Tyrosine"),
            "UGU" | "UGC" => Some("Cysteine"),
            "UGG" => Some("Tryptophan"),
            "UAA" | "UAG" | "UGA" => None,
            _ => Some("UNK"),
        })
        .collect();

    if amino_acids.contains(&"UNK")
        || rna.len() % 3 != 0 && !["UAA", "UAG", "UGA"].iter().any(|stop| rna.contains(stop))
    {
        None
    } else {
        Some(amino_acids)
    }
}
