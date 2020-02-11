const freqAlphabets = require('./decrypt-string-from-alphabet-to-integer-mapping')

test('freqAlphabets', () => {
  expect(freqAlphabets('10#11#12')).toBe('jkab');
  expect(freqAlphabets('1326#')).toBe('acz');
  expect(freqAlphabets('25#')).toBe('y');
  expect(freqAlphabets('12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#')).toBe('abcdefghijklmnopqrstuvwxyz');
})