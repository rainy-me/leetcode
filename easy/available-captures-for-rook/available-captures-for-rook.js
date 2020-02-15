/**
 * @param {character[][]} board
 * @return {number}
 */
var numRookCaptures = function (board) {
  let ret = 0;
  let top = Array(8);

  const getR = () => {
    for (let i = 0; i < 8; i++) {
      let left = "";
      for (let j = 0; j < 8; j++) {
        const cell = board[i][j]
        if (cell === 'R') {
          if (left && /[a-z]/.test(left)) {
            ret++
          }
          if (top[j] && /[a-z]/.test(top[j])) {
            ret++
          }
          return [i, j]
        } else if (cell !== '.') {
          top[j] = cell
          left = cell
        }
      }
    }

  }
  const [i, j] = getR()
  for (let m = j + 1; m < 8; m++) {
    const cell = board[i][m]
    if (/[A-Z]/.test(cell)) {
      break;
    }
    if (cell && /[a-z]/.test(cell)) {
      ret++
      break;
    }
  }

  for (let n = i + 1; n < 8; n++) {
    const cell = board[n][j]

    if (/[A-Z]/.test(cell)) {
      break;
    }
    if (cell && /[a-z]/.test(cell)) {
      ret++
      break;
    }
  }
  return ret;
};

module.exports = numRookCaptures