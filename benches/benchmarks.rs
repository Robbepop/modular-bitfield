#![allow(dead_code)]

mod utils;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use modular_bitfield::{
    bitfield,
    specifiers::{
        B12,
        B13,
        B16,
        B3,
        B32,
        B36,
        B4,
        B6,
        B7,
        B8,
        B9,
    },
};
use utils::repeat;

criterion_group!(bench_get, bench_get_variants);
criterion_group!(bench_set, bench_set_variants);
criterion_main!(bench_get, bench_set);

#[bitfield]
pub struct Color {
    r: B8,
    g: B8,
    b: B8,
    a: B8,
}

#[bitfield]
pub struct SingleBitsInSingleByte {
    b0: bool,
    b1: bool,
    b2: bool,
    b3: bool,
    b4: bool,
    b5: bool,
    b6: bool,
    b7: bool,
}

#[bitfield]
pub struct TwoHalfBytes {
    h0: B4,
    h1: B4,
}

#[bitfield]
pub struct SingleBitAndRest {
    head: bool,
    rest: B7,
}

#[bitfield]
pub struct B7B1 {
    b7: B7,
    b1: bool,
}

#[bitfield]
pub struct B3B1B4 {
    b3: B3,
    b1: bool,
    b4: B4,
}

#[bitfield]
pub struct TwoHalfWords {
    fst: B16,
    snd: B16,
}

#[bitfield]
pub struct B6B12B6 {
    front: B6,
    middle: B12,
    back: B6,
}

#[bitfield]
pub struct B6B36B6 {
    front: B6,
    middle: B36,
    back: B6,
}

#[bitfield]
pub struct Complex {
    a: B9,  // 0th and 1st
    b: B6,  // 1st
    c: B13, // 1st, 2nd, 3rd
    d: B4,  // 3rd
    e: B32, // 4th, .., 7th
}

fn bench_set_variants(c: &mut Criterion) {
    let mut g = c.benchmark_group("set_variants");
    g.bench_function("Color", |b| {
        let mut input = Color::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_r(1);
                black_box(&mut input).set_g(1);
                black_box(&mut input).set_b(1);
                black_box(&mut input).set_a(1);
            })
        });
    });
    g.bench_function("SingleBitsInSingleByte", |b| {
        let mut input = SingleBitsInSingleByte::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_b0(true);
                black_box(&mut input).set_b1(true);
                black_box(&mut input).set_b2(true);
                black_box(&mut input).set_b3(true);
                black_box(&mut input).set_b4(true);
                black_box(&mut input).set_b5(true);
                black_box(&mut input).set_b6(true);
                black_box(&mut input).set_b7(true);
            })
        });
    });
    g.bench_function("TwoHalfBytes", |b| {
        let mut input = TwoHalfBytes::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_h0(1);
                black_box(&mut input).set_h1(1);
            })
        });
    });
    g.bench_function("SingleBitAndRest", |b| {
        let mut input = SingleBitAndRest::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_head(true);
                black_box(&mut input).set_rest(1);
            })
        });
    });
    g.bench_function("B7B1", |b| {
        let mut input = B7B1::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_b7(1);
                black_box(&mut input).set_b1(true);
            })
        });
    });
    g.bench_function("B3B1B4", |b| {
        let mut input = B3B1B4::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_b3(1);
                black_box(&mut input).set_b1(true);
                black_box(&mut input).set_b4(1);
            })
        });
    });
    g.bench_function("TwoHalfWords", |b| {
        let mut input = TwoHalfWords::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_fst(1);
                black_box(&mut input).set_snd(1);
            })
        });
    });
    g.bench_function("B6B12B6", |b| {
        let mut input = B6B12B6::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_front(1);
                black_box(&mut input).set_middle(1);
                black_box(&mut input).set_back(1);
            })
        });
    });
    g.bench_function("B6B36B6", |b| {
        let mut input = B6B36B6::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_front(1);
                black_box(&mut input).set_middle(1);
                black_box(&mut input).set_back(1);
            })
        });
    });
    g.bench_function("Complex", |b| {
        let mut input = Complex::new();
        b.iter(|| {
            repeat(|| {
                black_box(&mut input).set_a(1);
                black_box(&mut input).set_b(1);
                black_box(&mut input).set_c(1);
                black_box(&mut input).set_d(1);
                black_box(&mut input).set_e(1);
            })
        });
    });
}

fn bench_get_variants(c: &mut Criterion) {
    let mut g = c.benchmark_group("get");
    g.bench_function("Color", |b| {
        let input = Color::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.r());
                black_box(input.g());
                black_box(input.b());
                black_box(input.a());
            })
        });
    });
    g.bench_function("SingleBitsInSingleByte", |b| {
        let input = SingleBitsInSingleByte::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.b0());
                black_box(input.b1());
                black_box(input.b2());
                black_box(input.b3());
                black_box(input.b4());
                black_box(input.b5());
                black_box(input.b6());
                black_box(input.b7());
            })
        });
    });
    g.bench_function("TwoHalfBytes", |b| {
        let input = TwoHalfBytes::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.h0());
                black_box(input.h1());
            })
        });
    });
    g.bench_function("SingleBitAndRest", |b| {
        let input = SingleBitAndRest::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.head());
                black_box(input.rest());
            })
        });
    });
    g.bench_function("B7B1", |b| {
        let input = B7B1::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.b7());
                black_box(input.b1());
            })
        });
    });
    g.bench_function("B3B1B4", |b| {
        let input = B3B1B4::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.b3());
                black_box(input.b1());
                black_box(input.b4());
            })
        });
    });
    g.bench_function("TwoHalfWords", |b| {
        let input = TwoHalfWords::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.fst());
                black_box(input.snd());
            })
        });
    });
    g.bench_function("B6B12B6", |b| {
        let input = B6B12B6::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.front());
                black_box(input.middle());
                black_box(input.back());
            })
        });
    });
    g.bench_function("B6B36B6", |b| {
        let input = B6B36B6::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.front());
                black_box(input.middle());
                black_box(input.back());
            })
        });
    });
    g.bench_function("Complex", |b| {
        let input = Complex::new();
        b.iter(|| {
            repeat(|| {
                black_box(input.a());
                black_box(input.b());
                black_box(input.c());
                black_box(input.d());
                black_box(input.e());
            })
        });
    });
}
