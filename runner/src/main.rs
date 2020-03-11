use ansi_term::Color;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use structopt::StructOpt;

pub fn unknown_impl<T>(implementation: &str) -> anyhow::Result<T> {
    Err(anyhow::anyhow!(
        "Unknown implementation: {}",
        implementation
    ))
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum LangType {
    Cpp,
    JavaScript,
    Python,
    Rust,
}

impl LangType {
    pub fn compile(
        self,
        opt: &Opt,
        program_path: &Path,
        implementation: &str,
    ) -> anyhow::Result<()> {
        let command = match self {
            LangType::Rust => match implementation {
                "rustc" => Some(
                    Command::new("rustc")
                        .current_dir(&opt.build_output)
                        .stderr(Stdio::inherit())
                        .arg("-Ctarget-cpu=native")
                        .arg("-Copt-level=2")
                        .arg(program_path)
                        .spawn()?,
                ),

                other => return unknown_impl(other),
            },
            LangType::Cpp => match implementation {
                "g++" | "clang++" => Some(
                    Command::new(implementation)
                        .current_dir(&opt.build_output)
                        .stderr(Stdio::inherit())
                        .arg("-o")
                        .arg(program_path.file_stem().unwrap())
                        .arg("-march=native")
                        .arg("-O3")
                        .arg(program_path)
                        .spawn()?,
                ),
                other => return unknown_impl(other),
            },
            LangType::JavaScript | LangType::Python => None,
        };

        if let Some(mut command) = command {
            let status = command
                .wait()
                .with_context(|| format!("Failed to compile with {}", implementation))?;
            assert!(status.success(), "Compile process failed!");
        }

        Ok(())
    }

    pub fn bench_command(
        self,
        opt: &Opt,
        program_path: &Path,
        implementation: &str,
    ) -> anyhow::Result<Command> {
        match self {
            LangType::JavaScript => {
                let mut com = Command::new(match implementation {
                    "node" => "node",
                    other => return unknown_impl(other),
                });
                com.arg(program_path);
                Ok(com)
            }
            LangType::Python => {
                let mut com = Command::new(match implementation {
                    "pypy" => "pypy",
                    "python" => "python",
                    other => return unknown_impl(other),
                });
                com.arg(program_path);
                Ok(com)
            }
            LangType::Cpp | LangType::Rust => Ok(Command::new(
                opt.build_output.join(program_path.file_stem().unwrap()),
            )),
        }
    }
}

impl fmt::Display for LangType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangType::JavaScript => write!(f, "{}", Color::Yellow.paint("JavaScript")),
            LangType::Python => write!(f, "{}", Color::Blue.paint("Python")),
            LangType::Cpp => write!(f, "{}", Color::Cyan.paint("C++")),
            LangType::Rust => write!(f, "{}", Color::Red.paint("Rust")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    lang: LangType,
    #[serde(rename = "impl")]
    implementations: Vec<String>,
    idiomatic: bool,
    path: String,
}

impl Program {
    pub fn bench(
        &self,
        opt: &Opt,
        args: &[String],
        stdin_content: &[u8],
        expect_stdout: &[u8],
    ) -> anyhow::Result<()> {
        let program_path = opt.target.join(&self.path);

        for implementation in &self.implementations {
            let impl_color = Color::Purple.paint(implementation);
            println!("Start compile {} with {}", self.lang, impl_color);
            let compile_start = Instant::now();

            self.lang
                .compile(opt, &program_path, implementation)
                .with_context(|| format!("Compile failed with {}", implementation))?;

            println!(
                "Compile with {} complete! elapsed: {}s",
                impl_color,
                Color::Yellow.paint(compile_start.elapsed().as_secs_f64().to_string())
            );
            let mut sum = Duration::new(0, 0);

            const BENCH_COUNT: u32 = 5;

            for _ in 0..BENCH_COUNT {
                let mut command = self
                    .lang
                    .bench_command(opt, &program_path, implementation)?;
                command
                    .current_dir(&opt.build_output)
                    .args(args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit());

                let start = Instant::now();
                let mut bench_process = command
                    .spawn()
                    .with_context(|| format!("Benchmark command failed with {}", implementation))?;

                bench_process
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(stdin_content)?;

                let output = bench_process.wait_with_output()?;

                let elapsed = start.elapsed();

                assert!(output.status.success());

                println!(
                    "Benchmark {}[{}] elapsed: {}s",
                    self.lang,
                    impl_color,
                    Color::Yellow.paint(elapsed.as_secs_f64().to_string())
                );

                sum += elapsed;

                assert_eq!(expect_stdout, output.stdout.as_slice());
            }

            let average = sum / BENCH_COUNT;

            println!(
                "Benchmark {}[{}] done! average: {}s",
                self.lang,
                impl_color,
                Color::Yellow.paint(average.as_secs_f64().to_string()),
            );
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ProgramStdinType {
    File,
    Text,
}

impl ProgramStdinType {
    pub fn get_bytes(&self, target_path: &Path, content: &str) -> anyhow::Result<Vec<u8>> {
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
    pub fn get_bytes(&self, path: &Path) -> anyhow::Result<Vec<u8>> {
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
    pub fn bench(&self, opt: &Opt) -> anyhow::Result<()> {
        let bench_name = Color::Cyan.paint(&self.name);

        println!("Start {}...", bench_name);

        let stdin_content: Vec<u8> = self
            .stdin
            .as_ref()
            .map(|stdin| stdin.get_bytes(&opt.target))
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

        for program in self.programs.iter() {
            program.bench(opt, &args, &stdin_content, self.stdout.as_bytes())?;
        }

        Ok(())
    }
}

#[derive(StructOpt)]
#[structopt(name = "gm_benchmark_runner", about = "Benchmark Runner")]
pub struct Opt {
    #[structopt(short = "b", about = "Path for store build output")]
    build_output: PathBuf,
    #[structopt(short = "t", about = "Path where bench.yml exists")]
    target: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    if !opt.build_output.exists() {
        std::fs::create_dir_all(&opt.build_output)?;
    }

    let bench: Bench = serde_yaml::from_reader(fs::File::open(opt.target.join("bench.yml"))?)?;

    bench
        .bench(&opt)
        .with_context(|| format!("Failed to benchmark {}", bench.name))
}
