use criterion::{criterion_group, criterion_main, Criterion};
use simdeez::prelude::*;

simd_unsafe_generate_all!(
    fn bench_horizontal_add_f32(data: &[f32]) -> f32 {
        let mut sum = 0.0f32;
        let chunks = data.len() / S::Vf32::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vf32::WIDTH;
            let v = S::Vf32::load_from_slice(&data[offset..]);
            sum += v.horizontal_add();
        }
        sum
    }
);

simd_unsafe_generate_all!(
    fn bench_horizontal_add_f64(data: &[f64]) -> f64 {
        let mut sum = 0.0f64;
        let chunks = data.len() / S::Vf64::WIDTH;
        for i in 0..chunks {
            let offset = i * S::Vf64::WIDTH;
            let v = S::Vf64::load_from_slice(&data[offset..]);
            sum += v.horizontal_add();
        }
        sum
    }
);

fn criterion_benchmark(c: &mut Criterion) {
    let f32_data: Vec<f32> = (0..32768).map(|i| (i as f32) * 0.1).collect();
    let f64_data: Vec<f64> = (0..32768).map(|i| (i as f64) * 0.1).collect();

    c.bench_function("horizontal_add f32 sse41", |b| {
        b.iter(|| unsafe { bench_horizontal_add_f32_sse41(&f32_data) })
    });
    c.bench_function("horizontal_add f32 avx2", |b| {
        b.iter(|| unsafe { bench_horizontal_add_f32_avx2(&f32_data) })
    });

    c.bench_function("horizontal_add f64 sse41", |b| {
        b.iter(|| unsafe { bench_horizontal_add_f64_sse41(&f64_data) })
    });
    c.bench_function("horizontal_add f64 avx2", |b| {
        b.iter(|| unsafe { bench_horizontal_add_f64_avx2(&f64_data) })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
