export const rows = (n) => {
  if (n === 0) {
    return [];
  }

  const triangle = [[1]];
  for (let i = 1; i < n; ++i) {
    const row = [1];
    for (let j = 1; j <= i; ++j) {
      row[j] = j === i ? 1 : triangle[i - 1][j - 1] + triangle[i - 1][j];
    }
    triangle[i] = row;
  }
  return triangle;
};
