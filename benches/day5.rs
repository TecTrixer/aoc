use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut blocks = io.blocks().into_iter();
        let mut stacks = blocks.next().unwrap();
        let mut commands = blocks.next().unwrap();
        let mut crates = vec![];
        for _ in 0..9 {
            crates.push(vec![]);
        }
        for line in stacks.lines() {
            let chars = line.chars().collect::<Vec<char>>();
            if chars[0] == '1' {
                break;
            }
            let mut idx = 1;
            while idx < chars.len() {
                let c = chars[idx];
                let stack_num = (idx - 1) / 4;
                if c != ' ' {
                    crates[stack_num].push(c);
                }
                idx += 4;
            }
        }
        for stack in crates.iter_mut() {
            stack.reverse();
        }
        let mut crates2 = crates.clone();
        for mut line in commands.line_io() {
            let (_, n, _, f, _, t): (String, usize, String, usize, String, usize) = line.tuple();
            // Part 1
            for _ in 0..n {
                let elem = crates[f - 1].pop().unwrap();
                crates[t - 1].push(elem);
            }

            // Part 2
            let mut temp = Vec::new();
            for _ in 0..n {
                let elem: char = crates2[f - 1].pop().unwrap();
                temp.push(elem);
            }
            for _ in 0..n {
                let elem = temp.pop().unwrap();
                crates2[t - 1].push(elem);
            }
        }
        io.write("Part 1: ");
        for i in 0..9 {
            io.write(crates[i].pop().unwrap());
        }
        io.nl();
        io.write("Part 2: ");
        for i in 0..9 {
            io.write(crates2[i].pop().unwrap());
        }
        io.nl();
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day5", |b| {
        b.iter(|| bench(black_box(include_str!("../day5.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
