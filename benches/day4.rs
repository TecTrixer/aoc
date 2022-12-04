use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cp_rs::io::*;

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut res1 = 0;
        let mut res2 = 0;
        for r in io.pnums::<isize>().chunks(4) {
            if r[0] <= r[2] && r[1] >= r[3] || r[0] >= r[2] && r[1] <= r[3] {
                res1 += 1;
            }
            if r[0] <= r[2] && r[1] >= r[2] || r[0] >= r[2] && r[0] <= r[3] {
                res2 += 1;
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
    c.bench_function("day4", |b| b.iter(|| bench(black_box(include_str!("../day4.txt").to_string()))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
