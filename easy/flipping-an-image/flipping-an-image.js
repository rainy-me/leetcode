/**
 * @param {number[][]} A
 * @return {number[][]}
 */
var flipAndInvertImage = function (A) {
  return A.map(row => {
    return row.reverse().map(cell => cell ^ 1)
  })
};

/**
 * @param {number[][]} A
 * @return {number[][]}
 */

// no reverse
//
// var flipAndInvertImage = function (A) {
//   const size = A[0].length - 1
//   return A.map(row => {
//     let count = 0;
//     while (count < (size + 1) / 2) {
//       // flip
//       [row[size - count], row[count]] = [row[count], row[size - count]]
//       // invert
//       count++
//     }
//     return row.map(cell => cell ^ 1)
//   })
// };