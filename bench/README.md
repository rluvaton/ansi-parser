# Benchmarks

For mapping
```bash
hyperfine --warmup 10 './bench/mapping/run-with-mapping.sh' './bench/mapping/run-without-mapping.sh' 
```

For split
```bash
hyperfine --warmup 10 './bench/split/run-without-split.sh' './bench/split/run-split.sh'
```

## Sync vs Async bench

sync bench last commit is `e742f7aeb53f7842906db88ed6cdd07a250439d6`

this is the result of all the benchmarks
```bash
$ hyperfine --warmup 10 './bench/split/run-split.sh' './bench/split/run-split-sync.sh'
Benchmark 1: ./bench/split/run-split.sh
  Time (mean ± σ):      57.7 ms ±   4.3 ms    [User: 34.1 ms, System: 19.2 ms]
  Range (min … max):    50.5 ms …  66.6 ms    50 runs
 
Benchmark 2: ./bench/split/run-split-sync.sh
  Time (mean ± σ):     826.8 ms ±   4.2 ms    [User: 522.3 ms, System: 284.0 ms]
  Range (min … max):   821.9 ms … 834.4 ms    10 runs
 
Summary
  './bench/split/run-split.sh' ran
   14.32 ± 1.06 times faster than './bench/split/run-split-sync.sh'
   
   
$ hyperfine --warmup 10 './bench/split/run-without-split.sh' './bench/split/run-without-split-sync.sh'
Benchmark 1: ./bench/split/run-without-split.sh
  Time (mean ± σ):      22.7 ms ±   3.3 ms    [User: 10.7 ms, System: 10.9 ms]
  Range (min … max):    15.4 ms …  34.3 ms    94 runs
 
Benchmark 2: ./bench/split/run-without-split-sync.sh
  Time (mean ± σ):     457.0 ms ±   4.8 ms    [User: 303.0 ms, System: 135.8 ms]
  Range (min … max):   449.3 ms … 463.9 ms    10 runs
 
Summary
  './bench/split/run-without-split.sh' ran
   20.15 ± 2.97 times faster than './bench/split/run-without-split-sync.sh'


############################# TODO - investigate why the sync is faster than async
$ hyperfine --warmup 10 './bench/mapping/run-with-mapping.sh' './bench/mapping/run-with-mapping-sync.sh'
Benchmark 1: ./bench/mapping/run-with-mapping.sh
  Time (mean ± σ):      1.620 s ±  0.011 s    [User: 1.011 s, System: 1.194 s]
  Range (min … max):    1.599 s …  1.634 s    10 runs
 
Benchmark 2: ./bench/mapping/run-with-mapping-sync.sh
  Time (mean ± σ):     761.2 ms ±   7.4 ms    [User: 480.8 ms, System: 258.5 ms]
  Range (min … max):   746.8 ms … 770.6 ms    10 runs
 
Summary
  './bench/mapping/run-with-mapping-sync.sh' ran
    2.13 ± 0.03 times faster than './bench/mapping/run-with-mapping.sh'
    
$ hyperfine --warmup 10 './bench/mapping/run-without-mapping.sh' './bench/mapping/run-without-mapping-sync.sh'
Benchmark 1: ./bench/mapping/run-without-mapping.sh
  Time (mean ± σ):      44.2 ms ±   3.5 ms    [User: 27.9 ms, System: 9.1 ms]
  Range (min … max):    33.9 ms …  51.6 ms    64 runs
 
Benchmark 2: ./bench/mapping/run-without-mapping-sync.sh
  Time (mean ± σ):      1.659 s ±  0.043 s    [User: 1.152 s, System: 0.435 s]
  Range (min … max):    1.618 s …  1.762 s    10 runs
 
Summary
  './bench/mapping/run-without-mapping.sh' ran
   37.50 ± 3.10 times faster than './bench/mapping/run-without-mapping-sync.sh'

```


----

Running flame graph:
```sh
cargo flamegraph --root --open -- mapping create --input ../examples/fixtures/huge.ans --output ../examples/fixtures/huge.ans.mapping 
```
