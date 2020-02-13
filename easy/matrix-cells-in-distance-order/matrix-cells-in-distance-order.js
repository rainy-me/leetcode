/**
 * @param {number} R
 * @param {number} C
 * @param {number} r0
 * @param {number} c0
 * @return {number[][]}
 */
var allCellsDistOrder = function (R, C, r0, c0) {
  return [...Array(R).keys()].reduce((a, r) => {
    [
      ...Array(C).keys()
    ].forEach((c) => {
      a.push([r, c])
    })
    return a
  }, [])
    .sort((l1, l2) => {
      const d1 = Math.abs(r0 - l1[0]) + Math.abs(c0 - l1[1])
      const d2 = Math.abs(r0 - l2[0]) + Math.abs(c0 - l2[1])
      return d1 - d2
    })
};

module.exports = allCellsDistOrder