const DISCOUNT = [0, 0, .05, .1, .2, .25];

export function cost(books) {
  const unique = new Set(books).size;
  const total = 800 * (books.length - DISCOUNT[unique] * unique);
  return total;
};
