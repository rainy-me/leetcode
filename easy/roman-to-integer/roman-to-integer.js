/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function (s) {
  const map = new Map([
    ['I', 1],
    ['V', 5],
    ['X', 10],
    ['L', 50],
    ['C', 100],
    ['D', 500],
    ['M', 1000],
  ])
  return [...s].reduce((sum, char, i) => {
    sum += map.get(char);
    if (i > 0 && map.get(s[i - 1]) < map.get(char)) {
      sum -= 2 * map.get(s[i - 1]);
    }
    return sum;
  }, 0);
};

var romanToInt2 = function (s) {
  [
    ['IV', '+4'],
    ['IX', '+9'],
    ['XL', '+40'],
    ['XC', '+90'],
    ['CD', '+400'],
    ['CM', '+900'],
    ['I', '+1'],
    ['V', '+5'],
    ['X', '+10'],
    ['L', '+50'],
    ['C', '+100'],
    ['D', '+500'],
    ['M', '+1000'],
  ].map(([k, v]) => {
    s = s.replace(new RegExp(k, 'g'), v)
  })
  return eval(s)
};

module.exports = romanToInt
