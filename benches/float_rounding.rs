use criterion::{criterion_group, criterion_main, Criterion};
use simdeez::prelude::*;

// --- f32 floor ---

simd_unsafe_generate_all!(
    fn bench_f32_floor(data: &[f32], out: &mut [f32]) {
        let chunks = data.len() / S::Vf32::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vf32::WIDTH;
            let v = S::Vf32::load_from_slice(&data[offset..]);
            let result = v.floor();
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- f32 ceil ---

simd_unsafe_generate_all!(
    fn bench_f32_ceil(data: &[f32], out: &mut [f32]) {
        let chunks = data.len() / S::Vf32::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vf32::WIDTH;
            let v = S::Vf32::load_from_slice(&data[offset..]);
            let result = v.ceil();
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- f64 floor ---

simd_unsafe_generate_all!(
    fn bench_f64_floor(data: &[f64], out: &mut [f64]) {
        let chunks = data.len() / S::Vf64::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vf64::WIDTH;
            let v = S::Vf64::load_from_slice(&data[offset..]);
            let result = v.floor();
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

// --- f64 ceil ---

simd_unsafe_generate_all!(
    fn bench_f64_ceil(data: &[f64], out: &mut [f64]) {
        let chunks = data.len() / S::Vf64::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vf64::WIDTH;
            let v = S::Vf64::load_from_slice(&data[offset..]);
            let result = v.ceil();
            result.copy_to_slice(&mut out[offset..]);
        }
    }
);

fn criterion_benchmark(c: &mut Criterion) {
    let size = 32768;

    let f32_data: Vec<f32> = (0..size).map(|i| (i as f32) * 0.1 - 1000.0).collect();
    let mut f32_out = vec![0.0f32; size];

    let f64_data: Vec<f64> = (0..size).map(|i| (i as f64) * 0.1 - 1000.0).collect();
    let mut f64_out = vec![0.0f64; size];

    // f32 floor
    c.bench_function("f32 floor sse2", |b| {
        b.iter(|| unsafe { bench_f32_floor_sse2(&f32_data, &mut f32_out) })
    });
    c.bench_function("f32 floor sse41", |b| {
        b.iter(|| unsafe { bench_f32_floor_sse41(&f32_data, &mut f32_out) })
    });
    c.bench_function("f32 floor avx2", |b| {
        b.iter(|| unsafe { bench_f32_floor_avx2(&f32_data, &mut f32_out) })
    });

    // f32 ceil
    c.bench_function("f32 ceil sse2", |b| {
        b.iter(|| unsafe { bench_f32_ceil_sse2(&f32_data, &mut f32_out) })
    });
    c.bench_function("f32 ceil sse41", |b| {
        b.iter(|| unsafe { bench_f32_ceil_sse41(&f32_data, &mut f32_out) })
    });
    c.bench_function("f32 ceil avx2", |b| {
        b.iter(|| unsafe { bench_f32_ceil_avx2(&f32_data, &mut f32_out) })
    });

    // f64 floor
    c.bench_function("f64 floor sse2", |b| {
        b.iter(|| unsafe { bench_f64_floor_sse2(&f64_data, &mut f64_out) })
    });
    c.bench_function("f64 floor sse41", |b| {
        b.iter(|| unsafe { bench_f64_floor_sse41(&f64_data, &mut f64_out) })
    });
    c.bench_function("f64 floor avx2", |b| {
        b.iter(|| unsafe { bench_f64_floor_avx2(&f64_data, &mut f64_out) })
    });

    // f64 ceil
    c.bench_function("f64 ceil sse2", |b| {
        b.iter(|| unsafe { bench_f64_ceil_sse2(&f64_data, &mut f64_out) })
    });
    c.bench_function("f64 ceil sse41", |b| {
        b.iter(|| unsafe { bench_f64_ceil_sse41(&f64_data, &mut f64_out) })
    });
    c.bench_function("f64 ceil avx2", |b| {
        b.iter(|| unsafe { bench_f64_ceil_avx2(&f64_data, &mut f64_out) })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
