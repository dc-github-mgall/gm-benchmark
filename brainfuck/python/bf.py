import sys

class Program():
    def __init__(self, source):
        self.code = ""
        self.bracket_map = {}
        stack = []

        for char in source:
            if char == '[':
                stack.append(len(self.code))
            elif char == ']':
                left = stack.pop()
                right = len(self.code)
                self.bracket_map[left] = right
                self.bracket_map[right] = left
            elif not char in ('<', '>', '+', '-', ',', '.'):
                continue

            self.code += char


    def run(self):
        tape = [0]
        pc = 0
        ptr = 0

        while pc < len(self.code):
            char = self.code[pc]
            if char == '+':
                tape[ptr] += 1
            elif char == '-' and tape[ptr] > 0:
                tape[ptr] -= 1
            elif char == '>':
                ptr += 1
                if len(tape) == ptr:
                    tape.append(0)
            elif char == '<' and ptr > 0:
                ptr -= 1
            elif char == ',':
                tape[ptr] = sys.stdin.read(1)
            elif char == '.':
                sys.stdout.write(chr(tape[ptr]))
            elif char == '[' and tape[ptr] == 0:
                pc = self.bracket_map[pc]
            elif char == ']' and tape[ptr] != 0:
                pc = self.bracket_map[pc]
            pc += 1

length = int(sys.argv[1])
source = sys.stdin.read(length)
Program(source).run()
sys.stdout.flush()

