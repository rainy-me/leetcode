/**
 * @param {string} S
 * @return {string}
 */
var removeDuplicates = function (S) {
  let stack = [];
  [...S].forEach(v => {
    if (stack[stack.length - 1] === v) {
      stack.pop()
    } else {
      stack.push(v)
    }
  })
  return stack.join("")
};
module.exports = removeDuplicates