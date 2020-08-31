const lib = require('../pkg/ssvm_nodejs_archive.js')

console.log(lib.int_add(1, 123))
console.log(lib.int_multi_option(0, 100))
console.log(lib.int_multi_option(2, 2))
console.log(lib.int_multi_result(0, 100))
console.log(lib.int_multi_result(2, 2))
console.log(lib.str_concat("hello", "world"))
console.log(lib.str_concat(
  new TextEncoder().encode("hello"),
  new TextEncoder().encode("world")
))
