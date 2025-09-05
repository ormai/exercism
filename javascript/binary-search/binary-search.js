export const find = (haystack, needle) => {
  let lo = 0, hi = haystack.length - 1;
  while (lo <= hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (haystack[mid] > needle) {
      hi = mid - 1;
    } else if (haystack[mid] < needle) {
      lo = mid + 1
    } else return mid;
  }
  throw new Error('Value not in array');
};
