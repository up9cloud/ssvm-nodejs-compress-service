const lib = require('../pkg/ssvm_nodejs_archive.js')

const util = require('util')
const http = require('http')

const debug = util.debuglog('app')
const Busboy = require('busboy')

const hostname = '0.0.0.0'
const port = 3000

const help = `Please use command:
curl http://${hostname}:${port}/ -F 'file=@path/to/local/file' > file.7z
`
function eEnd (res, e = help, code = 400) {
  res.writeHead(code, { 'content-type': 'application/json' })
  res.end(JSON.stringify({
    error: e.toString()
  }))
}
function compressFile (file) {
  return lib.compress(file.buffer, 9)
}
function endCompressFiles (files) {
  for (const file of files) {
    const o = lib.decompress(file.buffer)
  }
  // TODO:
  res.writeHead(200, { 'content-type': 'application/x-lzma' })
  res.end(Buffer.from(""))
}
function endCompressFile (res, file) {
  const o = compressFile(file)
  res.writeHead(200, { 'content-type': 'application/x-lzma' })
  res.end(Buffer.from(o))
}
const server = http.createServer((req, res) => {
  if (req.method.toLowerCase() !== 'post') {
    return eEnd(res, "Method Not Allowed", 405)
  }
  if (req.url !== '/') {
    return eEnd(res, "Not Found", 404)
  }
  let files = []
  const busboy = new Busboy({ headers: req.headers });
  busboy.on('file', function(field, file, filename, encoding, mimeType) {
    let buffer = Buffer.alloc(0)
    file.on('data', function(chunk) {
      buffer = Buffer.concat([buffer, chunk])
    })
    file.on('end', function() {
      files.push({
        field, filename, encoding, mimeType,
        buffer
      })
      debug('receive %s: finished', field)
    })
  })
  busboy.on('field', function(field, val, fieldNameTruncated, valTruncated, encoding, mimeType) {
    debug('[%s] %s = %o', field, val)
  })
  busboy.on('finish', function() {
    if (files.length > 1) {
      endCompressFiles(res, files)
    } else {
      endCompressFile(res, files[0])
    }
  })
  return req.pipe(busboy)
})

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
