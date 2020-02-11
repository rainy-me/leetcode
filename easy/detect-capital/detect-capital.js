/**
 * @param {string} word
 * @return {boolean}
 */
var detectCapitalUse = function (word) {
  return /^[A-Z]?[a-z]+$|^[A-Z]*$/m.test(word)
};