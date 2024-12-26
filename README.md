# aoc2024

runs part1 + part2 for all days combined in ~227ms, only single-threaded on a laptop with an i5-1135G7:

```txt
$ hyperfine --warmup 3 --runs 100 target/release/aoc2024                                                        

Benchmark 1: target/release/aoc2024
  Time (mean ± σ):     227.0 ms ±   4.2 ms    [User: 220.6 ms, System: 4.5 ms]
  Range (min … max):   221.9 ms … 259.5 ms    100 runs
```
