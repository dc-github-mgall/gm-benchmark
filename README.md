# gm-benchmark

## Requirements

* Unix-like OS
* gcc, pypy, python, nodejs, cargo, rustc

## Results

```
gm-benchmark on  master [⇡!]
❯ ./bench.sh
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/gm-benchmark-runner ../brainfuck`
Start compile bench BrainFuck...
    Finished release [optimized] target(s) in 0.00s
Compile BrainFuck done!
Start BrainFuck...
Benchmark Simple C++(C++[gcc]) elapsed: 0.078690906s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078649896s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078208255s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078808326s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078708366s
Benchmark Simple C++(C++[gcc]) done! average: 0.078613149s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085542484s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085512234s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085154384s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085168385s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.084888823s
Benchmark Idiomatic Rust(Rust[rustc]) done! average: 0.085253262s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.874875448s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.826776389s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.916614599s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.878772623s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.887537563s
Benchmark JavaScript Implementation(JavaScript[node]) done! average: 0.876915324s
Benchmark Simple Python(Python[pypy]) elapsed: 0.365353246s
Benchmark Simple Python(Python[pypy]) elapsed: 0.326188738s
Benchmark Simple Python(Python[pypy]) elapsed: 0.347586814s
Benchmark Simple Python(Python[pypy]) elapsed: 0.336369801s
Benchmark Simple Python(Python[pypy]) elapsed: 0.33590485s
Benchmark Simple Python(Python[pypy]) done! average: 0.342280689s
Benchmark Simple Python(Python[python]) elapsed: 6.879554697s
Benchmark Simple Python(Python[python]) elapsed: 6.816932411s
Benchmark Simple Python(Python[python]) elapsed: 6.881144388s
Benchmark Simple Python(Python[python]) elapsed: 6.788808936s
Benchmark Simple Python(Python[python]) elapsed: 6.787304334s
Benchmark Simple Python(Python[python]) done! average: 6.830748953s


```
