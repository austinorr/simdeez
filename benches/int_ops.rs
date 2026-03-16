use criterion::{criterion_group, Criterion};
use simdeez::prelude::*;

// --- i8 multiply ---

simd_unsafe_generate_all!(
    fn bench_i8_mul(a: &[i8], b: &[i8], out: &mut [i8]) {
        let chunks = a.len() / S::Vi8::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vi8::WIDTH;
            let va = S::Vi8::load_from_slice(&a[offset..]);
            let vb = S::Vi8::load_from_slice(&b[offset..]);
            let result = va * vb;
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- i8 less-than ---

simd_unsafe_generate_all!(
    fn bench_i8_lt(a: &[i8], b: &[i8], out: &mut [i8]) {
        let chunks = a.len() / S::Vi8::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vi8::WIDTH;
            let va = S::Vi8::load_from_slice(&a[offset..]);
            let vb = S::Vi8::load_from_slice(&b[offset..]);
            let result = va.cmp_lt(vb);
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- i16 less-than ---

simd_unsafe_generate_all!(
    fn bench_i16_lt(a: &[i16], b: &[i16], out: &mut [i16]) {
        let chunks = a.len() / S::Vi16::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vi16::WIDTH;
            let va = S::Vi16::load_from_slice(&a[offset..]);
            let vb = S::Vi16::load_from_slice(&b[offset..]);
            let result = va.cmp_lt(vb);
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- i32 less-than ---

simd_unsafe_generate_all!(
    fn bench_i32_lt(a: &[i32], b: &[i32], out: &mut [i32]) {
        let chunks = a.len() / S::Vi32::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vi32::WIDTH;
            let va = S::Vi32::load_from_slice(&a[offset..]);
            let vb = S::Vi32::load_from_slice(&b[offset..]);
            let result = va.cmp_lt(vb);
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- i64 less-than ---

simd_unsafe_generate_all!(
    fn bench_i64_lt(a: &[i64], b: &[i64], out: &mut [i64]) {
        let chunks = a.len() / S::Vi64::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vi64::WIDTH;
            let va = S::Vi64::load_from_slice(&a[offset..]);
            let vb = S::Vi64::load_from_slice(&b[offset..]);
            let result = va.cmp_lt(vb);
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

fn criterion_benchmark(c: &mut Criterion) {
    let size = 32768;

    let i8_a: Vec<i8> = (0..size).map(|i| (i % 127) as i8).collect();
    let i8_b: Vec<i8> = (0..size).map(|i| ((i * 3) % 127) as i8).collect();
    let mut i8_out = vec![0i8; size];

    let i16_a: Vec<i16> = (0..size).map(|i| (i % 32000) as i16).collect();
    let i16_b: Vec<i16> = (0..size).map(|i| ((i * 3) % 32000) as i16).collect();
    let mut i16_out = vec![0i16; size];

    let i32_a: Vec<i32> = (0..size).map(|i| i as i32).collect();
    let i32_b: Vec<i32> = (0..size).map(|i| (i as i32).wrapping_mul(3)).collect();
    let mut i32_out = vec![0i32; size];

    let i64_a: Vec<i64> = (0..size).map(|i| i as i64).collect();
    let i64_b: Vec<i64> = (0..size).map(|i| (i as i64).wrapping_mul(3)).collect();
    let mut i64_out = vec![0i64; size];

    let mut g = c.benchmark_group("int_ops");

    // i8 mul
    g.bench_function("i8 mul sse2", |b| {
        b.iter(|| unsafe { bench_i8_mul_sse2(&i8_a, &i8_b, &mut i8_out) })
    });
    g.bench_function("i8 mul sse41", |b| {
        b.iter(|| unsafe { bench_i8_mul_sse41(&i8_a, &i8_b, &mut i8_out) })
    });
    g.bench_function("i8 mul avx2", |b| {
        b.iter(|| unsafe { bench_i8_mul_avx2(&i8_a, &i8_b, &mut i8_out) })
    });

    // i8 lt
    g.bench_function("i8 lt sse2", |b| {
        b.iter(|| unsafe { bench_i8_lt_sse2(&i8_a, &i8_b, &mut i8_out) })
    });
    g.bench_function("i8 lt sse41", |b| {
        b.iter(|| unsafe { bench_i8_lt_sse41(&i8_a, &i8_b, &mut i8_out) })
    });
    g.bench_function("i8 lt avx2", |b| {
        b.iter(|| unsafe { bench_i8_lt_avx2(&i8_a, &i8_b, &mut i8_out) })
    });

    // i16 lt
    g.bench_function("i16 lt sse2", |b| {
        b.iter(|| unsafe { bench_i16_lt_sse2(&i16_a, &i16_b, &mut i16_out) })
    });
    g.bench_function("i16 lt sse41", |b| {
        b.iter(|| unsafe { bench_i16_lt_sse41(&i16_a, &i16_b, &mut i16_out) })
    });
    g.bench_function("i16 lt avx2", |b| {
        b.iter(|| unsafe { bench_i16_lt_avx2(&i16_a, &i16_b, &mut i16_out) })
    });

    // i32 lt
    g.bench_function("i32 lt sse2", |b| {
        b.iter(|| unsafe { bench_i32_lt_sse2(&i32_a, &i32_b, &mut i32_out) })
    });
    g.bench_function("i32 lt sse41", |b| {
        b.iter(|| unsafe { bench_i32_lt_sse41(&i32_a, &i32_b, &mut i32_out) })
    });
    g.bench_function("i32 lt avx2", |b| {
        b.iter(|| unsafe { bench_i32_lt_avx2(&i32_a, &i32_b, &mut i32_out) })
    });

    // i64 lt
    g.bench_function("i64 lt sse2", |b| {
        b.iter(|| unsafe { bench_i64_lt_sse2(&i64_a, &i64_b, &mut i64_out) })
    });
    g.bench_function("i64 lt sse41", |b| {
        b.iter(|| unsafe { bench_i64_lt_sse41(&i64_a, &i64_b, &mut i64_out) })
    });
    g.bench_function("i64 lt avx2", |b| {
        b.iter(|| unsafe { bench_i64_lt_avx2(&i64_a, &i64_b, &mut i64_out) })
    });
}

criterion_group!(benches, criterion_benchmark);
