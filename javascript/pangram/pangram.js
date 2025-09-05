export const isPangram = phrase => {
  const lowered = phrase.toLowerCase();
  return [...'abcdefghijklmnopqrstuvwxyz'].every(letter => lowered.includes(letter));
};
