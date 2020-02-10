/**
 * @param {number[]} arr
 * @return {void} Do not return anything, modify arr in-place instead.
 */
var duplicateZeros = function (arr) {
  const copy = [...arr];
  let addZero = false;
  for (let i = 0; i < arr.length; i++) {
    if (addZero) {
      addZero = false
      arr[i] = 0
      continue
    }
    const head = copy.shift()
    if (head == 0) {
      addZero = true
    }
    arr[i] = head
  }
};