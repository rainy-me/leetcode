/**
 * @param {string[]} logs
 * @return {string[]}
 */
var reorderLogFiles = function (logs) {
  let textLogs = [];
  let digitLogs = [];
  logs.forEach(log => {
    if (log.match(/.+ [0-9]+/g)) {
      digitLogs.push(log)
      return
    }
    textLogs.push(log)
  })
  return [...textLogs.sort((a, b) => {

    let [idx1, ...rest1] = a.split(' ')
    let [idx2, ...rest2] = b.split(' ')
    rest1 = rest1.join(" ")
    rest2 = rest2.join(" ")
    if (rest1 === rest2) {
      return idx1 > idx2 ? 1 : -1
    }
    return rest1 > rest2 ? 1 : -1
  }), ...digitLogs]
}

module.exports = reorderLogFiles