const DISCOUNT = [0, 10, 30, 80, 125];

export function cost(books) {
  let c = {};
  for (const book of books) {
    if (!c[book]) {
      c[book] = 0;
    }
    c[book] += 1;
  }
  const counts = Object.values(c).sort((a, b) => b - a);

  for (let i = 0; i < counts.length - 1; i++) {
    counts[i] -= counts[i + 1];
  }

  if (counts.length >= 5) {
    let m = Math.min(counts[2], counts[4]);
    counts[2] -= m;
    counts[4] -= m;
    counts[3] += 2 * m;
  }

  let deduction = 0;
  for (let i = 0; i < counts.length; i++) {
    deduction += counts[i] * DISCOUNT[i];
  }
  return (books.length * 100 - deduction) * 8;
}
