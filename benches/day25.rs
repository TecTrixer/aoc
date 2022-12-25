use criterion::{criterion_group, criterion_main, Criterion};
use cp_rs::io::*;
const ITERATIONS: u32 = 22;
fn bench() {
    let mut io = Io::from_file("day25.txt");
    let mut res = 0;
    for mut line in io.line_io() {
        let mut val: isize = 0;
        for c in line.chars() {
            val *= 5;
            val += match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
        }
        res += val;
    }
    let mut res2 = String::new();
    let mut five_pot = 5_isize.pow(ITERATIONS);
    let mut rest = 0;
    for j in 0..ITERATIONS {
        rest -= 2 * 5_isize.pow(j);
    }
    for _ in 0..ITERATIONS {
        five_pot /= 5;
        rest += 2 * five_pot;
        if res >= 2 * five_pot + rest {
            res2.push('2');
            res -= 2 * five_pot;
        } else if res >= five_pot + rest {
            res2.push('1');
            res -= five_pot;
        } else if res <= -2 * five_pot - rest {
            res2.push('=');
            res += 2 * five_pot;
        } else if res <= - five_pot - rest {
            res2.push('-');
            res += five_pot;
        } else {
            res2.push('0');
        }
    }
    io.write("Part 1: ");
    io.writeln(res2.trim_start_matches('0'));
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day25", |b| b.iter(|| bench()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
