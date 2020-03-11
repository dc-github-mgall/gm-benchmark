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
Benchmark Simple C++(C++[gcc]) elapsed: 0.078834506s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078733926s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078931217s
Benchmark Simple C++(C++[gcc]) elapsed: 0.079316907s
Benchmark Simple C++(C++[gcc]) elapsed: 0.078893096s
Benchmark Simple C++(C++[gcc]) done! average: 0.07894193s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067374842s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067518813s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067438592s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067676503s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.067513422s
Benchmark Idiomatic Rust(Rust[rustc]) done! average: 0.067504434s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.822494324s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.889046445s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.892147049s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.841323857s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.815313675s
Benchmark JavaScript Implementation(JavaScript[node]) done! average: 0.85206507s
Benchmark Simple Python(Python[pypy]) elapsed: 0.341308277s
Benchmark Simple Python(Python[pypy]) elapsed: 0.33608888s
Benchmark Simple Python(Python[pypy]) elapsed: 0.338634223s
Benchmark Simple Python(Python[pypy]) elapsed: 0.33559189s
Benchmark Simple Python(Python[pypy]) elapsed: 0.350162707s
Benchmark Simple Python(Python[pypy]) done! average: 0.340357195s
Benchmark Simple Python(Python[python]) elapsed: 6.974152853s
Benchmark Simple Python(Python[python]) elapsed: 6.856582299s
Benchmark Simple Python(Python[python]) elapsed: 6.797042696s
Benchmark Simple Python(Python[python]) elapsed: 7.097980483s
Benchmark Simple Python(Python[python]) elapsed: 6.800355741s
Benchmark Simple Python(Python[python]) done! average: 6.905222814s
```
