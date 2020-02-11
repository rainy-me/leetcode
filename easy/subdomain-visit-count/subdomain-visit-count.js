/**
 * @param {string[]} cpdomains
 * @return {string[]}
 */
var subdomainVisits = function (cpdomains) {
  let m = new Map()
  cpdomains.forEach(p => {
    let [ns, d] = p.split(" ")
    const n = parseInt(ns)
    const [a, b, c] = d.split(".").reverse()
    m.set(a, (m.get(a) || 0) + n)
    if (b) {
      const k = `${b}.${a}`;
      m.set(k, (m.get(k) || 0) + n)
    }
    if (c) {
      m.set(d, (m.get(d) || 0) + n)
    }
  })
  return [...m.entries()].map(([k, v]) => `${v} ${k}`)
};

console.log(
  subdomainVisits(["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"])
)