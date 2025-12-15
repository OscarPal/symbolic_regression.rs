use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use symbolic_regression::bench::{
    bfgs_quadratic_n16, constant_opt_linear_env, run_constant_opt_linear,
};

fn bench_bfgs_quadratic(c: &mut Criterion) {
    c.bench_function("optim/bfgs_quadratic_n16", |b| {
        b.iter(|| {
            std::hint::black_box(bfgs_quadratic_n16());
        })
    });
}

fn bench_constant_optimization(c: &mut Criterion) {
    let env = constant_opt_linear_env();
    c.bench_function("optim/optimize_constants_linear_c0x_plus_c1", |b| {
        b.iter_batched(
            || (),
            |_| {
                std::hint::black_box(run_constant_opt_linear(&env));
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_bfgs_quadratic, bench_constant_optimization);
criterion_main!(benches);
