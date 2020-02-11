/**
 * @param {string[]} queries
 * @param {string[]} words
 * @return {number[]}
 */
var numSmallerByFrequency = function (queries, words) {
  const getFrequency = (w) => {
    let c = "zz"
    let f = 0
    for (let char of w) {
      if (char > c) continue
      else if (char === c) { f++ }
      else { f = 1; c = char; }
    }
    return f;
  }
  words = words.map(getFrequency)
  queries = queries.map(getFrequency)

  return queries.map(q => {

    let result = 0
    /**
     * 1st) for ;;
     * 2nd) forEach
     * 3rd) for let
     * 4th) filter(...).length
     */
    for (let i = 0; i < words.length; i++) {
      if (words[i] > q) {
        result++
      }
    }
    return result
  })
};

console.log(numSmallerByFrequency(["bbb", "cc"], ["a", "aa", "aaa", "aaaa"]))
console.log(numSmallerByFrequency(["cbd"], ["zaaaz"]))