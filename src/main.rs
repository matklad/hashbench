use std::time::Instant;

const N: u32 = 50_000_000;

#[inline(never)]
fn build() -> rustc_hash::FxHashSet<u32> {
    numbers(N).collect()
}

#[inline(never)]
fn lookup(hm: &rustc_hash::FxHashSet<u32>) {
    assert!(numbers(N).all(|it| hm.contains(&it)))
}

fn main() {
    let hm = {
        let _t = timeit("build rustc_hash::FxHashSet");
        build()
    };

    {
        let _t = timeit("lookup rustc_hash::FxHashSet");
        lookup(&hm)
    }
}

fn numbers(n: u32) -> impl Iterator<Item = u32> {
//  On  my machine (5900X) this runs as
// build rustc_hash::FxHashSet       1.90s
// lookup rustc_hash::FxHashSet      5.33s

// Uncommenting any of the to cases gives
// build rustc_hash::FxHashSet       1.76s
// lookup rustc_hash::FxHashSet      1.45s
    (0..n)
        // uncomment rev to make lookup fast
        // .rev()
        .map(|i| i)
        // uncomment rev to make lookup fast
        // .collect::<Vec<u32>>()
        // .into_iter()
}

fn timeit(label: &'static str) -> impl Drop {
    struct Timer(&'static str, Instant);
    impl Drop for Timer {
        fn drop(&mut self) {
            eprintln!("{:<33} {:.2?}", self.0, self.1.elapsed());
        }
    }
    Timer(label, Instant::now())
}
