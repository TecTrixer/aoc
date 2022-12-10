use cp_rs::io::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench() -> String {
    let mut output = Vec::new();
    {
        let input = std::fs::File::open("day10.txt").unwrap();
        let mut io = Io::with_reader_and_writer(input, &mut output);
        let all = io.read_all();
        let input: &[u8] = all.as_bytes();
        let mut res = 0;
        let mut cycle: usize = 1;
        let mut reg: isize = 1;
        io.write("Part 2:");
        let mut idx = 0;
        while idx < input.len() {
            if cycle % 40 == 20 {
                res += cycle as isize * reg;
            }
            draw(&mut io, reg, cycle);
            match input[idx] {
                b'n' => {
                    idx += 5;
                    cycle += 1;
                }
                b'a' => {
                    idx += 5;
                    cycle += 1;
                    if cycle % 40 == 20 {
                        res += cycle as isize * reg;
                    }
                    draw(&mut io, reg, cycle);
                    let val = get_digit(input, &mut idx);
                    reg += val;
                    cycle += 1;
                }
                _ => unreachable!(),
            }
        }
        io.nl();
        io.write("Part 1: ");
        io.writeln(res);
    }
    let res = unsafe { std::str::from_utf8_unchecked(output.as_slice()) }.to_string();
    res
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day10", |b| {
        b.iter(|| bench())
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn draw(io: &mut Io<std::fs::File, &mut Vec<u8>>, reg: isize, cycle: usize) {
    let col = (cycle - 1) % 40;
    if col == 0 {
        io.nl();
    }
    if col == reg as usize || col == reg as usize + 1 || col == reg as usize - 1 {
        io.write('#');
    } else {
        io.write(' ');
    }
}

#[inline]
fn get_digit(input: &[u8], idx: &mut usize) -> isize {
    let mut val = 0;
    if input[*idx] == b'-' {
        *idx += 1;
        while input[*idx] != b'\n' {
            val *= 10;
            val += (input[*idx] - b'0') as isize;
            *idx += 1;
        }
        val *= -1;
    } else {
        while input[*idx] != b'\n' {
            val *= 10;
            val += (input[*idx] - b'0') as isize;
            *idx += 1;
        }
    }
    *idx += 1;
    val
}
