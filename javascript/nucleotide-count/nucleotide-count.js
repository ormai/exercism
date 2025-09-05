//
// This is only a SKELETON file for the 'Nucleotide Count' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export function countNucleotides(strand) {
  let a = 0, c = 0, g = 0, t = 0;

  for (const nucleotide of strand) {
    switch (nucleotide) {
      case 'A': a++; break;
      case 'C': c++; break;
      case 'G': g++; break;
      case 'T': t++; break;
      default: throw new Error('Invalid nucleotide in strand');
    }
  }

  return `${a} ${c} ${g} ${t}`
}
