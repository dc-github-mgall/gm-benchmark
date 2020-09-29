use ansi_term::Color;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;
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
    fn binary_name(self, program_path: &Path, implementation: &str) -> String {
        let stem = program_path.file_stem().unwrap().to_string_lossy();

        match self {
            LangType::Rust => format!("{}-rs-{}", stem, implementation),
            LangType::Cpp => format!("{}-cpp-{}", stem, implementation),
            LangType::Python => format!("{}-python-{}", stem, implementation),
            LangType::JavaScript => format!("{}-js-{}", stem, implementation),
        }
    }

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
                        .arg(format!(
                            "-o{}",
                            self.binary_name(program_path, implementation)
                        ))
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
                        .arg(self.binary_name(program_path, implementation))
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

    pub fn hyperfine_arg(
        self,
        opt: &Opt,
        program_path: &Path,
        implementation: &str,
    ) -> anyhow::Result<String> {
        match self {
            LangType::JavaScript => {
                let runtime = match implementation {
                    "node" => "node",
                    other => return unknown_impl(other),
                };

                Ok(format!("{} {}", runtime, program_path.display()))
            }
            LangType::Python => {
                let runtime = match implementation {
                    "pypy" => "pypy",
                    "python" => "python",
                    other => return unknown_impl(other),
                };

                Ok(format!("{} {}", runtime, program_path.display()))
            }
            LangType::Rust | LangType::Cpp => Ok(opt
                .build_output
                .join(self.binary_name(program_path, implementation))
                .display()
                .to_string()),
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
    pub fn bench(&self, opt: &Opt, hyperfine_args: &mut Vec<String>) -> anyhow::Result<()> {
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

            hyperfine_args.push(
                self.lang
                    .hyperfine_arg(opt, &program_path, implementation)?,
            );
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Bench {
    name: String,
    env: HashMap<String, String>,
    programs: Vec<Program>,
}

impl Bench {
    pub fn bench(&self, opt: &Opt) -> anyhow::Result<()> {
        let bench_name = Color::Cyan.paint(&self.name);
        let mut hyperfine_args = Vec::new();

        println!("Start {}...", bench_name);

        for program in self.programs.iter() {
            program.bench(opt, &mut hyperfine_args)?;
        }

        let bench_exit_status = Command::new("hyperfine")
            .current_dir(&opt.target)
            .args(hyperfine_args)
            .envs(&self.env)
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| "Run benchmark")?
            .wait()?;

        assert!(bench_exit_status.success(), "Bench run failed");

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
