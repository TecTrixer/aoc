use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);

        let mut res = 0;
        let mut cycle: usize = 1;
        let mut reg: isize = 1;
        let mut display = [['.'; 40]; 6];
        for mut line in io.line_io() {
            if cycle % 40 == 20 {
                res += cycle as isize * reg;
            }
            draw(&mut display, reg, cycle);
            match &line.read::<String>() as &str {
                "noop" => cycle += 1,
                "addx" => {
                    cycle += 1;
                    if cycle % 40 == 20 {
                        res += cycle as isize * reg;
                    }
                    draw(&mut display, reg, cycle);
                    let val = line.read::<isize>();
                    reg += val;
                    cycle += 1;
                }
                _ => unreachable!(),
            }
        }
        draw(&mut display, reg, cycle);
        io.write("Part 1: ");
        io.writeln(res);
        io.write("Part 2:");
        io.nl();
        for row in display {
            for pxl in row {
                io.write(pxl);
            }
            io.nl();
        }
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day10", |b| {
        b.iter(|| bench(black_box(include_str!("../day10.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
fn draw(display: &mut [[char; 40]; 6], reg: isize, cycle: usize) {
    let row = (cycle - 1) / 40;
    let col = (cycle - 1) % 40;
    if col == reg as usize || col == reg as usize + 1 || col == reg as usize - 1 {
        display[row][col] = '#';
    }
}
