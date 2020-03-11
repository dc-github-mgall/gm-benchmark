# gm-benchmark

## Requirements

* Unix-like OS
* gcc, pypy, python, nodejs, cargo, rustc

## Results

```
gm-benchmark on  master
❯ ./bench.sh
   Compiling gm-benchmark-runner v0.1.0 (/home/riey/repos/gm-benchmark/runner)
    Finished release [optimized] target(s) in 2.28s
     Running `target/release/gm-benchmark-runner -b ../build -t ../brainfuck -c 20`
Start BrainFuck...
Start compile C++ with g++
Compile with g++ complete! elapsed: 0.468922542s
Benchmark C++[g++] 0/20 elapsed: 0.064352149s
Benchmark C++[g++] 5/20 elapsed: 0.063907538s
Benchmark C++[g++] 10/20 elapsed: 0.063817828s
Benchmark C++[g++] 15/20 elapsed: 0.063910948s
Benchmark C++[g++] done! average: 0.064002978s
Start compile C++ with clang++
Compile with clang++ complete! elapsed: 0.586371246s
Benchmark C++[clang++] 0/20 elapsed: 0.06576287s
Benchmark C++[clang++] 5/20 elapsed: 0.06575673s
Benchmark C++[clang++] 10/20 elapsed: 0.065752121s
Benchmark C++[clang++] 15/20 elapsed: 0.06595576s
Benchmark C++[clang++] done! average: 0.065668244s
Start compile Rust with rustc
Compile with rustc complete! elapsed: 0.396574034s
Benchmark Rust[rustc] 0/20 elapsed: 0.067885692s
Benchmark Rust[rustc] 5/20 elapsed: 0.067982193s
Benchmark Rust[rustc] 10/20 elapsed: 0.067802252s
Benchmark Rust[rustc] 15/20 elapsed: 0.067944223s
Benchmark Rust[rustc] done! average: 0.068089419s
Start compile JavaScript with node
Compile with node complete! elapsed: 0.00000005s
Benchmark JavaScript[node] 0/20 elapsed: 0.892223349s
Benchmark JavaScript[node] 5/20 elapsed: 0.774663456s
Benchmark JavaScript[node] 10/20 elapsed: 0.756950884s
Benchmark JavaScript[node] 15/20 elapsed: 0.758659316s
Benchmark JavaScript[node] done! average: 0.796712597s
Start compile Python with pypy
Compile with pypy complete! elapsed: 0.00000007s
Benchmark Python[pypy] 0/20 elapsed: 0.341406577s
Benchmark Python[pypy] 5/20 elapsed: 0.337021411s
Benchmark Python[pypy] 10/20 elapsed: 0.339060183s
Benchmark Python[pypy] 15/20 elapsed: 0.326501988s
Benchmark Python[pypy] done! average: 0.337167788s
```
