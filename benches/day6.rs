use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let string = io.chars();
        let mut last = vec![];
        let mut res1 = usize::MAX;
        let mut res2 = usize::MAX;
        for (i, c) in string.iter().enumerate() {
            let mut idx = 0;
            while idx < last.len() {
                if last[idx] == c {
                    for _ in 0..=idx {
                        last.remove(0);
                    }
                    break;
                }
                idx += 1;
            }
            last.push(c);
            if last.len() >= 4 && i + 1 < res1 {
                res1 = i + 1;
            }
            if last.len() >= 14 {
                res2 = i + 1;
                break;
            }
        }
        io.write("Part 1: ");
        io.writeln(res1);
        io.write("Part 2: ");
        io.writeln(res2);
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day6", |b| {
        b.iter(|| bench(black_box(include_str!("../day6.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
