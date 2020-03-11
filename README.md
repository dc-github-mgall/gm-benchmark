# gm-benchmark

## Requirements

* Unix-like OS
* gcc, pypy, python, nodejs, cargo, rustc

## Results

```
gm-benchmark on  master [!]
❯ ./bench.sh
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/gm-benchmark-runner ../brainfuck`
Start compile bench BrainFuck...
    Finished release [optimized] target(s) in 0.00s
Compile BrainFuck done!
Start BrainFuck...
Benchmark Simple C++(C++[gcc]) elapsed: 0.064105349s
Benchmark Simple C++(C++[gcc]) elapsed: 0.063968628s
Benchmark Simple C++(C++[gcc]) elapsed: 0.064016828s
Benchmark Simple C++(C++[gcc]) elapsed: 0.063770248s
Benchmark Simple C++(C++[gcc]) elapsed: 0.063762208s
Benchmark Simple C++(C++[gcc]) done! average: 0.063924652s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067341922s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067497752s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067864763s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067325082s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067392833s
Benchmark Idiomatic Rust(Rust[rustc]) done! average: 0.06748447s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.887617023s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.757905745s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.760841079s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.796391932s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.82760022s
Benchmark JavaScript Implementation(JavaScript[node]) done! average: 0.806071199s
Benchmark Simple Python(Python[pypy]) elapsed: 0.343001929s
Benchmark Simple Python(Python[pypy]) elapsed: 0.337090302s
Benchmark Simple Python(Python[pypy]) elapsed: 0.325513337s
Benchmark Simple Python(Python[pypy]) elapsed: 0.32790132s
Benchmark Simple Python(Python[pypy]) elapsed: 0.337721472s
Benchmark Simple Python(Python[pypy]) done! average: 0.334245672s
Benchmark Simple Python(Python[python]) elapsed: 6.80762358s
Benchmark Simple Python(Python[python]) elapsed: 7.016276094s
Benchmark Simple Python(Python[python]) elapsed: 6.869123954s
Benchmark Simple Python(Python[python]) elapsed: 7.046543021s
Benchmark Simple Python(Python[python]) elapsed: 6.826493952s
Benchmark Simple Python(Python[python]) done! average: 6.91321212s
```
