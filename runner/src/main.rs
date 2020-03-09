use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Instant;

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LangType {
    Rust,
}

impl LangType {
    pub fn compile(
        self,
        path: PathBuf,
        bin: &str,
        ret: &mut Vec<(Child, PathBuf)>,
    ) -> anyhow::Result<()> {
        ret.push(match self {
            LangType::Rust => (
                Command::new("cargo")
                    .current_dir(&path)
                    .env("RUSTFLAGS", "-Ctarget-cpu=native")
                    .args(&["build", "--release"])
                    .spawn()?,
                path.join("target").join("release").join(bin),
            ),
        });

        Ok(())
    }
}

impl fmt::Display for LangType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangType::Rust => write!(f, "Rust"),
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
    pub fn compile(&self, path: &str, ret: &mut Vec<(Child, PathBuf)>) -> anyhow::Result<()> {
        let path = Path::new(path).join(&self.path);
        println!(
            "Compile {}({}) from {}",
            self.name,
            self.lang,
            path.display()
        );
        self.lang.compile(path, &self.bin, ret)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ProgramStdinType {
    File,
    Text,
}

impl ProgramStdinType {
    pub fn get_bytes(&self, path: &str, content: &str) -> anyhow::Result<Vec<u8>> {
        match self {
            ProgramStdinType::File => Ok(fs::read(Path::new(path).join(content))?),
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
    pub fn bench(&self, path: &str) -> anyhow::Result<()> {
        let mut compile_processes = Vec::with_capacity(self.programs.len() * 2);

        println!("Start compile bench {}...", self.name);

        for program in self.programs.iter() {
            program.compile(path, &mut compile_processes)?;
        }

        let stdin_content: Vec<u8> = self
            .stdin
            .as_ref()
            .map(|stdin| stdin.get_bytes(path))
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

        for (process, _) in compile_processes.iter_mut() {
            let status = process.wait()?;
            assert!(status.success());
        }

        println!("Compile bench {} done!", self.name);
        println!("Start bench {}...", self.name);

        for (i, (_, bin_path)) in compile_processes.iter().enumerate() {
            let program = &self.programs[i];
            println!(
                "Start benchmark [{}] in {}...",
                program.name, bin_path.display()
            );

            let start = Instant::now();
            let mut bench_process = Command::new(bin_path)
                .current_dir(path)
                .args(&args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?;

            bench_process
                .stdin
                .as_mut()
                .unwrap()
                .write_all(&stdin_content)?;

            let output = bench_process.wait_with_output()?;

            let elapsed = start.elapsed();

            assert!(output.status.success());

            println!(
                "Benchmark [{}] done! elapsed: {}s",
                program.name,
                elapsed.as_secs_f64()
            );

            assert_eq!(self.stdout.as_bytes(), output.stdout.as_slice());
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
