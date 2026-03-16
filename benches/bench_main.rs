mod float_rounding;
mod horizontal_add;
mod int_ops;
mod numparse;

use criterion::criterion_main;

criterion_main!(
    float_rounding::benches,
    horizontal_add::benches,
    int_ops::benches,
    numparse::benches,
);
