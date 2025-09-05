type Color = typeof COLORS[number];

const COLORS = [
  'black', 'brown', 'red', 'orange', 'yellow',
  'green', 'blue', 'violet', 'grey', 'white',
] as const;

export function decodedValue(colors: Color[]): number {
  return COLORS.indexOf(colors[0]) * 10 + COLORS.indexOf(colors[1]);
}
