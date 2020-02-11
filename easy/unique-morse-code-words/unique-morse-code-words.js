/**
 * @param {string[]} words
 * @return {number}
 */
var uniqueMorseRepresentations = function (words) {
  const d = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."]
  return [...new Set(words.map(w => {
    return [...w].map(i => {
      return d[i.charCodeAt(0) - 'a'.charCodeAt(0)]
    }).join('')
  }))].length
}
module.exports = uniqueMorseRepresentations