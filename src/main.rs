use std::time::Instant;

fn main() {
    let n = 50_000_000;
    let hm1 = {
        let _t = timeit("build std::collections::HashSet");
        numbers(n).collect::<std::collections::HashSet<_>>()
    };
    let hm2 = {
        let _t = timeit("build rustc_hash::FxHashSet");
        numbers(n).collect::<rustc_hash::FxHashSet<_>>()
    };
    assert!(hm1.len() == hm2.len());

    {
        let _t = timeit("lookup std::collections::HasSet");
        assert!(numbers(n).all(|it| hm1.contains(&it)))
    }
    {
        let _t = timeit("lookup rustc_hash::FxHashSet");
        assert!(numbers(n).all(|it| hm2.contains(&it)))
    }
}

fn numbers(n: u32) -> impl Iterator<Item = u32> {
    (0..n).map(|i| i.wrapping_mul(i) ^ i)
}

fn timeit(label: &'static str) -> impl Drop {
    struct Timer(&'static str, Instant);
    impl Drop for Timer {
        fn drop(&mut self) {
            eprintln!("{}: {:.2?}", self.0, self.1.elapsed());
        }
    }
    Timer(label, Instant::now())
}
