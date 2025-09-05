//
// This is only a SKELETON file for the 'Raindrops' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const convert = (n) => {
  let result = '';
  if (n % 3 === 0) result += 'Pling';
  if (n % 5 === 0) result += 'Plang';
  if (n % 7 === 0) result += 'Plong';
  return result || String(n);
};
