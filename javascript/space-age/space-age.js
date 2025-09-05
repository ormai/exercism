const PERIOD = {
  'mercury': .2408467,
  'venus': .61519726,
  'earth': 1,
  'mars': 1.8808158,
  'jupiter': 11.862615,
  'saturn': 29.447498,
  'uranus': 84.016846,
  'neptune': 164.79132,
};

export const age = (planet, seconds) =>
  Math.round(seconds / PERIOD[planet] / 86400 / 365.25 * 100) / 100;
