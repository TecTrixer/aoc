use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        // Vector with the sums of calories per Elf
        let mut sums: Vec<usize> = vec![];

        // Iterate through every Elf
        for mut block in io.blocks() {
            // Count the calories per Elf
            let mut res: usize = 0;
            for num in block.nums::<usize>() {
                res += num;
            }
            // Add the count to the sums list
            sums.push(res);
        }

        // Sort the list in reverse order -> most calories will be at first
        sums.sort();
        sums.reverse();

        io.write("Part 1: ");
        // Print most calories
        io.writeln(sums[0]);

        io.write("Part 2: ");
        // Print sum of top 3 calories
        io.writeln(sums[0..3].iter().sum::<usize>());
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        b.iter(|| bench(black_box(include_str!("../day1.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
