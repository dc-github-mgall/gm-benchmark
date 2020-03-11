#!/bin/bash

cd runner

cargo run --release -- -b ../build -t ../brainfuck -c 20

