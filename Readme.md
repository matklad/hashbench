```
λ cargo run --release | xclip -selection c
   Compiling hashbench v0.1.0 (/home/matklad/projects/hashbench)
    Finished release [optimized] target(s) in 0.44s
     Running `target/release/hashbench`
build std::collections::HashSet   4.35s
build rustc_hash::FxHashSet       3.76s
build ahash::HashSet              3.92s

lookup std::collections::HasSet   5.33s
lookup rustc_hash::FxHashSet      1.89s
lookup ahash::AHashSet            2.78s
```

I have no idea why for str lookup is costlier than building the hash map.
