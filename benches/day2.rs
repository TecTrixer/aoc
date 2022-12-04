use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn play1(opp: &str, me: &str) -> usize {
    match opp {
        "A" => match me {
            "X" => 3 + 1,
            "Y" => 6 + 2,
            "Z" => 0 + 3,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => 6 + 1,
            "Y" => 0 + 2,
            "Z" => 3 + 3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn play2(opp: &str, me: &str) -> usize {
    match opp {
        "A" => match me {
            "X" => 0 + 3,
            "Y" => 3 + 1,
            "Z" => 6 + 2,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => 0 + 2,
            "Y" => 3 + 3,
            "Z" => 6 + 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut res1 = 0;
        let mut res2 = 0;
        for mut line in io.line_io() {
            let (opp, me): (String, String) = line.tuple();
            res1 += play1(&opp, &me);
            res2 += play2(&opp, &me);
        }
        io.write("Part 1: ");
        io.writeln(res1);
        io.write("Part 2: ");
        io.writeln(res2);
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day2", |b| {
        b.iter(|| bench(black_box(include_str!("../day2.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
