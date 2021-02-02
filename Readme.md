```
λ cargo run --release
   Compiling hello v0.1.0 (/home/matklad/tmp/hello)
    Finished release [optimized] target(s) in 0.37s
     Running `target/release/hello`

build std::collections::HashSet: 4.32s
build rustc_hash::FxHashSet:     3.94s

lookup std::collections::HasSet: 5.32s
lookup rustc_hash::FxHashSet:    1.93s
```
