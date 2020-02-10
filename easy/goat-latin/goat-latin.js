var toGoatLatin = function (S) {
  return S.split(" ").map((text, index) => {
    const [all, vowel] = text.match(/([a|e|i|o|u]?)[\S]+/);

    let tmp = "";
    if (vowel) {
      tmp = all + "ma"
    } else {
      tmp = (all + all[0] + "ma").substr(1)
    }
    tmp += "a".repeat(index + 1)
    console.log(tmp)
    return tmp

  }).join(" ")
};