'use strict'
// **Github:** https://github.com/zensh/nodepack
//
// **License:** MIT

console.log('Current working directory:', process.cwd())
console.log('Command line arguments:', process.argv.join(', '))

module.exports = require('./app/')
