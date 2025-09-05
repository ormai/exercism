//
// This is only a SKELETON file for the 'Perfect Numbers' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const classify = (number) => {
  if (number <= 0) throw new Error('Classification is only possible for natural numbers.');

  let sum = 0;
  for (let f = 1; f < number; f++) {
    if (number % f === 0) {
      sum += f;
    }
  }

  if (number === sum) return 'perfect';
  if (number < sum) return 'abundant';
  return 'deficient';
};
