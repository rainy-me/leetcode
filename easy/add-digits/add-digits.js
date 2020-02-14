/**
 * @param {number} num
 * @return {number}
 */
var addDigits = function (num) {
  return 1 + (num - 1) % 9;
};

var addDigits2 = function (num) {
  while (num >= 10) {
    num = [...num.toString()].reduce((acc, n) => acc + parseInt(n), 0)
  }
  return num
};

module.exports = addDigits