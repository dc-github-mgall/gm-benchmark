let f = source => {
  let code = ''
  let bracket_pc = new Array(source.length).fill(0)
  let stack = []

  let split = source.split('')
  let len = split.length
  for (let i = 0; i < len; i++) {
    const char = split[i]

    if (char == '[') {
      stack.push(code.length)
    } else if (char == ']') {
      let left = stack.pop()
      let right = code.length

      bracket_pc[left] = right
      bracket_pc[right] = left
    } else if (!['<', '>', '+', '-', ',', '.'].filter(v => v == char).length) {
      continue
    }

    code += char
  }

  let tape = [0]
  let pc = 0
  let ptr = 0

  while (pc < code.length) {
    char = code[pc]

    if (char == '+') {
      tape[ptr] += 1
    } else if (char == '-' && tape[ptr] > 0) {
      tape[ptr] -= 1
    } else if (char == '>') {
      ptr += 1
      if (tape.length == ptr) tape.push(0)
    }
    if (char == '<' && ptr > 0) {
      ptr -= 1
    } else if (char == '.') {
      process.stdout.write(String.fromCharCode(tape[ptr]))
    } else if (char == '[' && tape[ptr] == 0) {
      pc = bracket_pc[pc]
    } else if (char == ']' && tape[ptr] != 0) {
      pc = bracket_pc[pc]
    }

    pc += 1
  }
}

const fs = require("fs");

fs.readFile(process.env["GM_BF_FILE"], (err, data) => {
  if (err) throw err

  f(data.toString())
})
