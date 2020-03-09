use ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LangType {
    Rust,
    Python,
}

impl LangType {
    pub fn compile(self, program_path: PathBuf, ret: &mut Vec<Child>) -> anyhow::Result<()> {
        match self {
            LangType::Rust => {
                ret.push(
                    Command::new("cargo")
                        .current_dir(&program_path)
                        .env("RUSTFLAGS", "-Ctarget-cpu=native")
                        .args(&["build", "--release"])
                        .spawn()?,
                );
            }
            LangType::Python => {}
        }

        Ok(())
    }

    pub fn get_command(self, program_path: PathBuf, bin: &str) -> Command {
        match self {
            LangType::Python => {
                let mut com = Command::new("python");
                com.arg(program_path.join(bin));
                com
            }
            LangType::Rust => Command::new(program_path.join("target").join("release").join(bin)),
        }
    }
}

impl fmt::Display for LangType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangType::Rust => write!(f, "{}", Color::Red.paint("Rust")),
            LangType::Python => write!(f, "{}", Color::Blue.paint("Python")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    lang: LangType,
    name: String,
    idiomatic: bool,
    path: String,
    bin: String,
}

impl Program {
    pub fn compile(&self, target_path: &str, ret: &mut Vec<Child>) -> anyhow::Result<()> {
        let program_path = Path::new(target_path).join(&self.path);
        self.lang.compile(program_path, ret)
    }

    pub fn get_command(&self, target_path: &str) -> Command {
        let program_path = Path::new(target_path).join(&self.path);
        self.lang.get_command(program_path, &self.bin)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ProgramStdinType {
    File,
    Text,
}

impl ProgramStdinType {
    pub fn get_bytes(&self, target_path: &str, content: &str) -> anyhow::Result<Vec<u8>> {
        match self {
            ProgramStdinType::File => Ok(fs::read(Path::new(target_path).join(content))?),
            ProgramStdinType::Text => Ok(content.as_bytes().to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProgramStdin {
    #[serde(rename = "type")]
    ty: ProgramStdinType,
    content: String,
}

impl ProgramStdin {
    pub fn get_bytes(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        self.ty.get_bytes(path, &self.content)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Bench {
    name: String,
    args: Vec<String>,
    stdin: Option<ProgramStdin>,
    stdout: String,
    programs: Vec<Program>,
}

impl Bench {
    pub fn bench(&self, target_path: &str) -> anyhow::Result<()> {
        let mut compile_processes = Vec::with_capacity(self.programs.len() * 2);
        let bench_name = Color::Cyan.paint(&self.name);

        println!("Start compile bench {}...", bench_name);

        for program in self.programs.iter() {
            program.compile(target_path, &mut compile_processes)?;
        }

        let stdin_content: Vec<u8> = self
            .stdin
            .as_ref()
            .map(|stdin| stdin.get_bytes(target_path))
            .transpose()?
            .unwrap_or_default();

        let args: Vec<String> = self
            .args
            .iter()
            .map(|arg| match arg.as_str() {
                "$CONTENT_LENGTH$" => stdin_content.len().to_string(),
                arg => arg.to_string(),
            })
            .collect();

        for process in compile_processes.iter_mut() {
            let status = process.wait()?;
            assert!(status.success());
        }

        println!("Compile {} done!", bench_name);
        println!("Start {}...", bench_name);

        for program in self.programs.iter() {
            let mut sum = Duration::new(0, 0);
            let program_name = Color::Green.paint(&program.name);

            const BENCH_COUNT: u32 = 5;

            for _ in 0..BENCH_COUNT {
                let mut command = program.get_command(target_path);
                command
                    .current_dir(target_path)
                    .args(&args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit());

                let start = Instant::now();
                let mut bench_process = command.spawn()?;

                bench_process
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(&stdin_content)?;

                let output = bench_process.wait_with_output()?;

                let elapsed = start.elapsed();

                assert!(output.status.success());

                println!(
                    "Benchmark {}({}) elapsed: {}s",
                    program_name,
                    program.lang,
                    Color::Yellow.paint(elapsed.as_secs_f64().to_string())
                );

                sum += elapsed;

                assert_eq!(self.stdout.as_bytes(), output.stdout.as_slice());
            }

            let average = sum / BENCH_COUNT;

            println!(
                "Benchmark {}({}) done! average: {}s",
                program_name,
                program.lang,
                average.as_secs_f64(),
            );
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let target_dir = env::args().skip(1).next().unwrap();

    let bench: Bench =
        serde_yaml::from_reader(fs::File::open(Path::new(&target_dir).join("bench.yml"))?)?;

    bench.bench(&target_dir)
}
