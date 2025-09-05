export const COLORS = [
  'black', 'brown', 'red', 'orange', 'yellow',
  'green', 'blue', 'violet', 'grey', 'white'
];


export const decodedValue = ([first, second]) =>
  Number(`${COLORS.indexOf(first)}${COLORS.indexOf(second)}`);
