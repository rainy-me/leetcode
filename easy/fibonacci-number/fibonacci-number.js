/**
 * @param {number} N
 * @return {number}
 */
var fib = function (N) {
  let m = new Map([[0, 0], [1, 1]])
  let f = (n) => {
    let cached = m.get(n)
    if (cached !== undefined) return cached
    let ret = f(n - 1) + f(n - 2)
    m.set(n, ret)
    return ret
  }
  return f(N)
};

console.log(fib(2))