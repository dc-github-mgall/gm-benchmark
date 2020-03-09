length = parseInt(process.argv[2])
process.stdin.setEncoding('utf8')

let f = source => {
  let code = ''
  let bracket_map = {}
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

      bracket_map[left] = right
      bracket_map[right] = left
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
    } else if (char == ',') {
      tape[ptr] = sys.stdin.read(1)
    } else if (char == '.') {
      process.stdout.write(String.fromCharCode(tape[ptr]))
    } else if (char == '[' && tape[ptr] == 0) {
      pc = bracket_map[pc]
    } else if (char == ']' && tape[ptr] != 0) {
      pc = bracket_map[pc]
    }

    pc += 1
  }
}

process.stdin.on('readable', () => {
  var input = process.stdin.read()

  if (input !== null) {
    f(input)
  }
})
