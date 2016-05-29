'use strict'
// **Github:** https://github.com/toajs/toa
//
// **License:** MIT

const toa = require('toa')
const ilog = require('ilog')
const toaMorgan = require('toa-morgan')

const app = module.exports = toa(function () {
  this.body = 'Hello, nodepack! -- ' + this.config.poweredBy
})

app.use(toaMorgan('tiny'))
app.listen(3000, () => ilog.info('Start nodepack demo at 3000'))

setTimeout(() => {
  process.exit(0)
}, 60000)
