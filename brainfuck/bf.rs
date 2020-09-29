use std::env;
use std::io::{self, BufWriter, Write};
use std::iter;

struct Program<W: Write> {
    source: Vec<u8>,
    bracket_pc: Vec<usize>,
    out: W,
}

impl<W: Write> Program<W> {
    pub fn new(source: &[u8], out: W) -> Self {
        let mut bf_source = Vec::with_capacity(source.len());
        let mut bracket_pc: Vec<usize> = iter::repeat(0).take(source.len()).collect();
        let mut stack = Vec::with_capacity(10);

        for s in source.iter() {
            match s {
                b'>' | b'<' | b'+' | b'-' | b',' | b'.' => {}
                b'[' => {
                    stack.push(bf_source.len());
                    bf_source.push(*s);
                }
                b']' => {
                    let left = stack.pop().unwrap();
                    let right = bf_source.len();
                    bf_source.push(*s);
                    bracket_pc[left] = right;
                    bracket_pc[right] = left;
                }
                _ => continue,
            }
            bf_source.push(*s);
        }

        Self {
            out,
            source: bf_source,
            bracket_pc,
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        let mut ptr = 0;
        let mut tape = vec![0; 8096];
        while let Some(byte) = self.source.get(pc).copied() {
            match byte {
                b'>' => {
                    ptr += 1;
                    if ptr >= tape.len() {
                        tape.push(0);
                    }
                }
                b'<' => {
                    ptr -= 1;
                }
                b'+' => {
                    tape[ptr] += 1;
                }
                b'-' => {
                    tape[ptr] -= 1;
                }
                b'.' => {
                    self.out.write(&[tape[ptr]]).unwrap();
                }
                b'[' => {
                    if tape[ptr] == 0 {
                        pc = self.bracket_pc[pc];
                    }
                }
                b']' => {
                    if tape[ptr] != 0 {
                        pc = self.bracket_pc[pc];
                    }
                }
                _ => {}
            }

            pc += 1;
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::with_capacity(8196, stdout);

    let source_file = env::var("GM_BF_FILE").unwrap();
    let source = std::fs::read(source_file).unwrap();

    let mut program = Program::new(&source, &mut stdout);

    program.run();
}
