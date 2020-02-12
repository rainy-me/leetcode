/**
 * @param {number} left
 * @param {number} right
 * @return {number[]}
 */
var selfDividingNumbers = function (left, right) {
  let ret = [];
  let head = left;
  while (head !== right + 1) {
    let ok = true
    for (let c of [...head.toString()]) {

      if (c == 0) {
        ok = false;
        break
      }
      if ((head % c) !== 0) {
        ok = false;
        break
      }
    }
    if (ok) { ret.push(head) }
    head++
  }
  return ret
};
module.exports = selfDividingNumbers