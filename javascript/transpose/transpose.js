export const transpose = (text) => {
  const [height, peak] = text.reduce(
    (a, b, i) => (b.length > a[0] ? [b.length, i] : a),
    [0, 0],
  );
  const transposed = Array(height);
  for (let c = 0; c < height; c++) {
    let row = "";
    for (let r = 0; r < text.length; r++) {
      const char = text[r]?.charAt(c);
      if (char === "") {
        if (
          r < peak ||
          c <
            text.slice(r + 1).find((line) => line.length > text[r].length)
              ?.length
        ) {
          row += " ";
        }
      } else {
        row += char;
      }
    }
    transposed[c] = row;
  }
  return transposed;
};
