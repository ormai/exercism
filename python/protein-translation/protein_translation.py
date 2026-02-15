def proteins(strand: str) -> list[str]:
    """Converts an RNA sequence into the repective codons."""

    amino_acids = []
    for codon in [strand[i : i + 3] for i in range(0, len(strand), 3)]:
        match codon:
            case "AUG":
                amino_acids.append("Methionine")
            case "UUU" | "UUC":
                amino_acids.append("Phenylalanine")
            case "UUA" | "UUG":
                amino_acids.append("Leucine")
            case "UCU" | "UCC" | "UCA" | "UCG":
                amino_acids.append("Serine")
            case "UAU" | "UAC":
                amino_acids.append("Tyrosine")
            case "UGU" | "UGC":
                amino_acids.append("Cysteine")
            case "UGG":
                amino_acids.append("Tryptophan")
            case "UAA" | "UAG" | "UGA":
                break
    return amino_acids
