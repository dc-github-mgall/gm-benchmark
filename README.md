# gm-benchmark

## Requirements

* Unix-like OS
* gcc, pypy, python, nodejs, cargo, rustc

## Results

```
gm-benchmark on  master
❯ ./bench.sh
   Compiling gm-benchmark-runner v0.1.0 (/home/riey/repos/gm-benchmark/runner)
    Finished release [optimized] target(s) in 2.32s
     Running `target/release/gm-benchmark-runner -b ../build -t ../brainfuck`
Start BrainFuck...
Benchmark C++[gcc] elapsed: 0.063901528s
Benchmark C++[gcc] elapsed: 0.063957348s
Benchmark C++[gcc] elapsed: 0.063740138s
Benchmark C++[gcc] elapsed: 0.063688488s
Benchmark C++[gcc] elapsed: 0.064069808s
Benchmark C++[gcc] done! average: 0.063871462s
Benchmark Rust[rustc] elapsed: 0.068082133s
Benchmark Rust[rustc] elapsed: 0.067884513s
Benchmark Rust[rustc] elapsed: 0.067707642s
Benchmark Rust[rustc] elapsed: 0.067583512s
Benchmark Rust[rustc] elapsed: 0.067681823s
Benchmark Rust[rustc] done! average: 0.067787924s
Benchmark JavaScript[node] elapsed: 0.840754547s
Benchmark JavaScript[node] elapsed: 0.765493274s
Benchmark JavaScript[node] elapsed: 0.755845372s
Benchmark JavaScript[node] elapsed: 0.756327594s
Benchmark JavaScript[node] elapsed: 0.756336613s
Benchmark JavaScript[node] done! average: 0.77495148s
Benchmark Python[pypy] elapsed: 0.351760779s
Benchmark Python[pypy] elapsed: 0.33596478s
Benchmark Python[pypy] elapsed: 0.337378942s
Benchmark Python[pypy] elapsed: 0.336640391s
Benchmark Python[pypy] elapsed: 0.325316407s
Benchmark Python[pypy] done! average: 0.337412259s
```
