export const hey = (message) => {
  if (/^\s*$/.test(message)) {
    return 'Fine. Be that way!';
  }
  if (/^[\sA-Z]+\?$/.test(message)) {
    return "Calm down, I know what I'm doing!";
  }
  if (/\?\s*$/.test(message)) {
    return 'Sure.';
  }
  if (/^[A-Z0-9\W]+$/.test(message) && /[A-Z]/.test(message)) {
    return 'Whoa, chill out!';
  }
  return 'Whatever.';
};
