const fs = require('fs')
const babel = require('@babel/core')
const generate = require('@babel/generator').default

const generateTest = async () => {
  const [_, __, dir] = process.argv
  if (!dir) { console.error("no dir"); return }
  const [___, title] = dir.split('/')
  const filename = `${title}.js`
  const { code: sourceCode } = await babel.transformFileAsync(`${dir}/${filename}`);
  const res = await babel.parseAsync(sourceCode);
  const funcName = res.program.body[0].declarations[0].id.name

  const testCodeAST = await babel.parseAsync(`
    const ${funcName} = require('./${filename}')
    test('${funcName}', () => {
      expect(${funcName}(0)).toBe(0);
    })
  `)
  const { code: testCode } = generate(testCodeAST)
  fs.writeFileSync(`${dir}/${title}.test.js`, testCode)
}

generateTest();