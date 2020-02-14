/**
 * @param {string[]} A
 * @return {number}
 */
var numSpecialEquivGroups = function (A) {
  return new Set(A.map(a => {
    let e = [];
    let o = [];
    [...a].map((char, index) => {
      if (index & 1) {
        o.push(char)
      } else {
        e.push(char)
      }
    })
    return [...o.sort(), ...e.sort()].join("")
  })).size
};
module.exports = numSpecialEquivGroups