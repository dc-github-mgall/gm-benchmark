use std::collections::BTreeMap;
use std::env;
use std::io::{self, BufReader, BufWriter, Bytes, Read, Write};

struct Program<R: Read, W: Write> {
    source: Vec<u8>,
    bracket_pair: BTreeMap<usize, usize>,
    out: W,
    input: Bytes<R>,
}

impl<R: Read, W: Write> Program<R, W> {
    pub fn new(source: &[u8], input: R, out: W) -> Self {
        let mut bf_source = Vec::with_capacity(source.len());
        let mut pairs = BTreeMap::new();
        let mut stack = Vec::new();

        for s in source.iter() {
            match s {
                b'>' | b'<' | b'+' | b'-' | b',' | b'.' => {
                    bf_source.push(*s);
                }
                b'[' => {
                    stack.push(bf_source.len());
                    bf_source.push(*s);
                }
                b']' => {
                    let left = stack.pop().unwrap();
                    let right = bf_source.len();
                    bf_source.push(*s);
                    pairs.insert(left, right);
                    pairs.insert(right, left);
                }
                _ => {}
            }
        }

        Self {
            out,
            input: input.bytes(),
            source: bf_source,
            bracket_pair: pairs,
        }
    }

    fn run_byte(
        &mut self,
        pc: &mut usize,
        ptr: &mut usize,
        tape: &mut Vec<u8>,
        byte: u8,
    ) -> anyhow::Result<()> {
        match byte {
            b'>' => {
                *ptr += 1;
                if *ptr >= tape.len() {
                    tape.push(0);
                }
            }
            b'<' => {
                *ptr = ptr.saturating_sub(1);
            }
            b'+' => {
                tape[*ptr] += 1;
            }
            b'-' => {
                tape[*ptr] -= 1;
            }
            b'.' => {
                self.out.write(&[tape[*ptr]])?;
            }
            b',' => {
                tape[*ptr] = self.input.next().unwrap()?;
            }
            b'[' => {
                if tape[*ptr] == 0 {
                    *pc = self.bracket_pair[pc];
                    return Ok(());
                }
            }
            b']' => {
                if tape[*ptr] != 0 {
                    *pc = self.bracket_pair[pc];
                    return Ok(());
                }
            }
            _ => {}
        }

        *pc += 1;

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut pc = 0;
        let mut ptr = 0;
        let mut tape = vec![0; 8096];
        while let Some(byte) = self.source.get(pc).copied() {
            self.run_byte(&mut pc, &mut ptr, &mut tape, byte)?;
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

    let source_length = env::args().skip(1).next().unwrap().parse()?;
    let mut source = vec![0u8; source_length];
    stdin.read(&mut source)?;

    let mut program = Program::new(&source, &mut stdin, &mut stdout);

    program.run()?;

    stdout.flush()?;

    Ok(())
}
