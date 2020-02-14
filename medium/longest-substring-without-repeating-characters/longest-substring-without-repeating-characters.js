/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function (s) {
  const len = s.length
  const m = new Map()
  let max = 0;
  let lastRepeat = -1;

  for (let i = 0; i < len; i++) {
    const last = m.get(s[i])
    if (last > lastRepeat) {
      lastRepeat = last
    }
    m.set(s[i], i)
    if (max < i - lastRepeat) {
      max = i - lastRepeat
    }
  }
  return max
};
module.exports = lengthOfLongestSubstring