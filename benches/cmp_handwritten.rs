//! In this benchmark we compare the macro generated code for
//! setters and getters to some hand-written code for the same
//! data structure.
//!
//! We do a performance analysis for the getter and setter of
//! all fields of both structs.
//!
//! Also we test here that our hand-written code and the macro
//! generated code actually are semantically equivalent.
//! This allows us to further enhance the hand-written code
//! and to eventually come up with new optimization tricks
//! for the macro generated code while staying correct.

mod handwritten;
mod utils;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use handwritten::{
    Generated,
    Handwritten,
};
use utils::repeat;

criterion_group!(
    bench_get,
    bench_get_a,
    bench_get_b,
    bench_get_c,
    bench_get_d,
    bench_get_e,
    bench_get_f,
);
criterion_group!(
    bench_set,
    bench_set_a,
    bench_set_b,
    bench_set_c,
    bench_set_d,
    bench_set_e,
    bench_set_f,
);
criterion_main!(bench_get, bench_set);

macro_rules! generate_cmp_benchmark_for {
    (
        test($test_name_get:ident, $test_name_set:ident) {
            fn $fn_get:ident($name_get:literal);
            fn $fn_set:ident($name_set:literal);
        }
    ) => {
        fn $test_name_get(c: &mut Criterion) {
            let mut g = c.benchmark_group($name_get);
            g.bench_function("generated", |b| {
                let input = black_box(Generated::new());
                b.iter(|| {
                    repeat(|| {
                        black_box(input.$fn_get());
                    })
                });
            });
            g.bench_function("handwritten", |b| {
                let input = Handwritten::new();
                b.iter(|| {
                    repeat(|| {
                        black_box(input.$fn_get());
                    })
                });
            });
        }

        fn $test_name_set(c: &mut Criterion) {
            let mut g = c.benchmark_group($name_set);
            g.bench_function("generated", |b| {
                let mut input = Generated::new();
                b.iter(|| {
                    repeat(|| {
                        black_box(black_box(&mut input).$fn_set(1));
                    })
                });
            });
            g.bench_function("handwritten", |b| {
                let mut input = Handwritten::new();
                b.iter(|| {
                    repeat(|| {
                        black_box(black_box(&mut input).$fn_set(1));
                    })
                });
            });
        }
    };
}
generate_cmp_benchmark_for!(
    test(bench_get_a, bench_set_a) {
        fn a("generated_vs_handwritten/get_a");
        fn set_a("generated_vs_handwritten/set_a");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_b, bench_set_b) {
        fn b("generated_vs_handwritten/get_b");
        fn set_b("generated_vs_handwritten/set_b");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_c, bench_set_c) {
        fn c("generated_vs_handwritten/get_c");
        fn set_c("generated_vs_handwritten/set_c");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_d, bench_set_d) {
        fn d("generated_vs_handwritten/get_d");
        fn set_d("generated_vs_handwritten/set_d");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_e, bench_set_e) {
        fn e("generated_vs_handwritten/get_e");
        fn set_e("generated_vs_handwritten/set_e");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_f, bench_set_f) {
        fn f("generated_vs_handwritten/get_f");
        fn set_f("generated_vs_handwritten/set_f");
    }
);
