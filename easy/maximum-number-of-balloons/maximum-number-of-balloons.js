/**
 * @param {string} text
 * @return {number}
 */
var maxNumberOfBalloons = function (text) {
  const map = new Map(
    [
      ["b", 0],
      ["a", 0],
      ["l", 0],
      ["o", 0],
      ["n", 0],
    ])
  for (const char of [...text]) {
    const count = map.get(char);
    if (count !== undefined) {
      map.set(char, count + 1)
    }
  }
  return [
    map.get('b'),
    map.get('a'),
    parseInt(map.get('l') / 2),
    parseInt(map.get('o') / 2),
    map.get('n')
  ].sort((a, b) => a - b)[0]
};