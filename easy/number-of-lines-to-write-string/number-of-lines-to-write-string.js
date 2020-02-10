/**
 * @param {number[]} widths
 * @param {string} S
 * @return {number[]}
 */
var numberOfLines = function (widths, S) {
  const widthMap = new Map([...'abcdefghijklmnopqrstuvwxyz'].map((char, index) => {
    return [char, widths[index]]
  }))
  return [...S].reduce((acc, char) => {
    const charWidth = widthMap.get(char);
    let tmp = acc[1] + charWidth
    if (tmp > 100) {
      tmp = charWidth
      acc[0]++
    }
    acc[1] = tmp;
    return acc;
  }, [1, 0])
};