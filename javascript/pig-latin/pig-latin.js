export const translate = phrase => phrase.split(' ').map(word => word
  .replace(/^([^aeiou]*qu)(.*)/, '$2$1')
  .replace(/^(?!(?:xr|yt))([^aeiou]+)(.*)/, '$2$1')
  .replace(/^([^aeiou]+)(y.*)/, '$2$1') + 'ay'
).join(' ');
