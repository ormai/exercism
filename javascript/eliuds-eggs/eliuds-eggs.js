export const eggCount = (displayValue) => {
  let popcount = 0;
  for (let i = displayValue; i > 0; i >>= 1) {
    popcount += i & 1;
  }
  return popcount;
};
