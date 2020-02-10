/**
 * @param {number} x
 * @param {number} y
 * @return {number}
 */
var hammingDistance = function (x, y) {
  let value = x ^ y;
  let counter = 0;

  while (value) {
    if (value & 1)
      ++counter
    value = value >> 1
  }
  return counter
};