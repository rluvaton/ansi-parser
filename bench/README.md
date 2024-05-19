# Benchmarks

For mapping
```bash
hyperfine --warmup 10 './bench/mapping/run-with-mapping.sh' './bench/mapping/run-without-mapping.sh' 
```

For split
```bash
hyperfine --warmup 10 './bench/split/run-without-split.sh' './bench/split/run-split.sh'
```
