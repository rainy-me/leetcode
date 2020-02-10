var letterCasePermutation = function (S) {
  const chars = [...S];
  let ret = [""];
  for (const char in chars) {
    if (/[A-Za-z]/.test(char)) {
      ret = [
        ...ret.map(c => c + char.toLowerCase()),
        ...ret.map(c => c + char.toUpperCase())
      ]
    } else {
      ret = ret.map(c => c + char)
    }
  }
  return ret;
};
