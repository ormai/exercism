export const format = (name, number) => {
  let ordinal = number.toString();
  if ([11, 12, 13].includes(number % 100)) {
    ordinal += "th";
  } else {
    ordinal += ["st", "nd", "rd"][(number % 10) - 1] ?? "th";
  }
  return `${name}, you are the ${ordinal} customer we serve today. Thank you!`;
};
