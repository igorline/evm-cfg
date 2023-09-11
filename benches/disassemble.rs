use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use evm_cfg::cfg_gen::dasm::disassemble;

pub fn criterion_benchmark(c: &mut Criterion) {
    let simple = hex::decode("6011609961000c90610013565b5f5260205ff35b6001019056").unwrap();
    c.bench_function("simple", |b| b.iter(|| disassemble(black_box(&simple))));

    let simple_broken =
        hex::decode("6011609961000c90610013565b5f5260205ff35b600101909156").unwrap();
    c.bench_function("simple_broken", |b| {
        b.iter(|| disassemble(black_box(&simple_broken)))
    });

    let april = fs::read_to_string("./examples/april.evm").expect("Should have april.evm");
    let april_hex = hex::decode(april).expect("Could not decode april.evm");
    c.bench_function("april", |b| b.iter(|| disassemble(black_box(&april_hex))));

    let curta12 = fs::read_to_string("./examples/curta12.evm").expect("Should have curta12.evm");
    let curta12_hex = hex::decode(curta12).expect("Could not decode curta12.evm");
    c.bench_function("curta12", |b| {
        b.iter(|| disassemble(black_box(&curta12_hex)))
    });

    let weth9 = fs::read_to_string("./examples/weth9.evm").expect("Should have weth9.evm");
    let weth9_hex = hex::decode(weth9).unwrap();
    c.bench_function("weth9", |b| b.iter(|| disassemble(black_box(&weth9_hex))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
