use std::env;
use std::io::{self, BufReader, BufWriter, Bytes, Read, Write};

struct Program<'source, R: Read, W: Write> {
    source: &'source [u8],
    out: W,
    input: Bytes<R>,
    prev_open_brace: usize,
    ptr: usize,
    pc: usize,
    tape: Vec<u8>,
}

impl<'source, R: Read, W: Write> Program<'source, R, W> {
    pub fn new(source: &'source [u8], input: R, out: W) -> Self {
        Self {
            source,
            out,
            input: input.bytes(),
            prev_open_brace: 0,
            ptr: 0,
            pc: 0,
            tape: vec![0; 8196],
        }
    }

    fn run_byte(&mut self, byte: u8) -> anyhow::Result<()> {
        self.pc += 1;
        match byte {
            b'>' => {
                self.ptr += 1;
            }
            b'<' => {
                self.ptr -= 1;
            }
            b'+' => {
                self.tape[self.ptr] += 1;
            }
            b'-' => {
                self.tape[self.ptr] -= 1;
            }
            b'.' => {
                self.out.write(&[self.tape[self.ptr]])?;
            }
            b',' => {
                self.tape[self.ptr] = self.input.next().unwrap()?;
            }
            b'[' => {
                self.prev_open_brace = self.pc - 1;
                while self.tape[self.ptr] == 0 {
                    while let Some(&byte) = self.source.get(self.pc) {
                        if byte == b']' {
                            if self.tape[self.ptr] != 0 {
                                continue;
                            } else {
                                self.pc += 1;
                                return Ok(());
                            }
                        } else {
                            self.run_byte(byte)?;
                        }
                    }
                }
            }
            b']' => {
                if self.tape[self.ptr] != 0 {
                    self.pc = self.prev_open_brace;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while let Some(byte) = self.source.get(self.pc) {
            self.run_byte(*byte)?;
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut stdin = BufReader::with_capacity(8196, stdin);
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::with_capacity(8196, stdout);

    let source_file = env::args().skip(1).next().unwrap();
    let source = std::fs::read(&source_file)?;

    let mut program = Program::new(&source, &mut stdin, &mut stdout);

    program.run()?;

    stdout.flush()?;

    Ok(())
}
