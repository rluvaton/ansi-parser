# Benchmarks

For mapping
```bash
hyperfine --warmup 10 './run-with-mapping.sh' './run-without-mapping.sh' 
```

For split
```bash
hyperfine --warmup 10 './bench/run-without-split.sh' './bench/run-split.sh'
```
