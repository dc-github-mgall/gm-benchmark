# gm-benchmark

## Results

```
gm-benchmark on  multiple-impls [✘!?] took 3s
❯ ./bench.sh
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/gm-benchmark-runner ../brainfuck`
Start compile bench BrainFuck...
    Finished release [optimized] target(s) in 0.00s
Compile BrainFuck done!
Start BrainFuck...
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.889752686s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.848662976s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.832343066s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.775069625s
Benchmark JavaScript Implementation(JavaScript[node]) elapsed: 0.858709478s
Benchmark JavaScript Implementation(JavaScript[node]) done! average: 0.840907566s
Benchmark Simple Python(Python[python]) elapsed: 8.022284781s
Benchmark Simple Python(Python[python]) elapsed: 7.591535606s
Benchmark Simple Python(Python[python]) elapsed: 7.618236379s
Benchmark Simple Python(Python[python]) elapsed: 7.855750728s
Benchmark Simple Python(Python[python]) elapsed: 7.697984146s
Benchmark Simple Python(Python[python]) done! average: 7.757158328s
Benchmark Simple Python(Python[pypy]) elapsed: 0.349152016s
Benchmark Simple Python(Python[pypy]) elapsed: 0.342552838s
Benchmark Simple Python(Python[pypy]) elapsed: 0.354748113s
Benchmark Simple Python(Python[pypy]) elapsed: 0.332871137s
Benchmark Simple Python(Python[pypy]) elapsed: 0.336964111s
Benchmark Simple Python(Python[pypy]) done! average: 0.343257643s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085471514s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085375804s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085275785s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.085325644s
Benchmark Idiomatic Rust(Rust[rustc]) elapsed: 0.086561085s
Benchmark Idiomatic Rust(Rust[rustc]) done! average: 0.085601966s


```
