/**
 * @param {number[]} stones
 * @return {number}
 */
var lastStoneWeight = function (stones) {
  while (stones.length > 1) {
    const sorted = stones.sort((a, b) => a - b)
    const [y, x] = [sorted.pop(), sorted.pop()]
    const rest = y - x;
    if (rest) stones.push(rest)
  }
  return stones.length ? stones[0] : 0
};