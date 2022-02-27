#![feature(sort_internals)]

use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};

const INPUT_DATA_FILE_PREFIX: &str = "benches/input_data";

fn read_or_generate_input(len: usize, modulus: i32) -> Vec<i32> {
    let mut input = Vec::with_capacity(len);

    let file_path_str = String::from(format!("input_{}_{}", len, modulus).as_str());
    let file_path = Path::new(INPUT_DATA_FILE_PREFIX).join(file_path_str.as_str());
    if file_path.exists() {
        // Read the generated input data.
        let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
        // 8-byte `len` + (len * 4)-byte i32 data.
        let mut buf = vec![0; 8 + len * 4];
        file.read(&mut buf).unwrap();
        input = bincode::deserialize(&buf[..]).unwrap();
    } else {
        // Generate the input data.
        let mut new_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(file_path)
            .unwrap();
        let mut rng = StdRng::from_entropy();
        for _ in 0..len {
            input.push(rng.gen::<i32>() % modulus);
        }
        let encoded = bincode::serialize(&input).unwrap();
        new_file.write_all(&encoded).unwrap();
    }
    input
}

fn sort_unstable(input: &[i32], tmp: &mut [i32]) {
    // Sort in default order.
    tmp.copy_from_slice(input);
    tmp.sort_unstable();
    // Sort in ascending order.
    tmp.copy_from_slice(input);
    tmp.sort_unstable_by(|a, b| a.cmp(b));
    // Sort in descending order.
    tmp.copy_from_slice(input);
    tmp.sort_unstable_by(|a, b| b.cmp(a));
}

fn benchmark_sort_unstable(c: &mut Criterion) {
    for len in [25, 500] {
        for modulus in [5, 10, 100, 1000] {
            let input = read_or_generate_input(len, modulus);
            let mut tmp = vec![0; input.len()];
            c.bench_with_input(
                BenchmarkId::new(
                    "sort_unstable",
                    format!("length: {}, modulus: {}", len, modulus),
                ),
                &len,
                |b, _| b.iter(|| sort_unstable(&input, &mut tmp)),
            );
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000);
    targets = benchmark_sort_unstable
}
criterion_main!(benches);
