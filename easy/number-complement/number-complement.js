var findComplement = function (num) {
  return num ^ parseInt(num.toString(2).replace(/\d/g, '1'), 2);
};