use cp_rs::geometry::*;
use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashSet;

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut visited1: HashSet<Point2D<i32>> = HashSet::new();
        let mut visited2: HashSet<Point2D<i32>> = HashSet::new();
        let mut tail_pos1 = vec![Point2D::new(0, 0); 2];
        let mut tail_pos2 = vec![Point2D::new(0, 0); 10];
        for mut line in io.line_io() {
            let (dir, n): (String, i32) = line.tuple();
            match &dir as &str {
                "R" => tail_pos1 = move_dir(n, &mut visited1, tail_pos1, Point2D::new(1, 0)),
                "U" => tail_pos1 = move_dir(n, &mut visited1, tail_pos1, Point2D::new(0, 1)),
                "L" => tail_pos1 = move_dir(n, &mut visited1, tail_pos1, Point2D::new(-1, 0)),
                "D" => tail_pos1 = move_dir(n, &mut visited1, tail_pos1, Point2D::new(0, -1)),
                _ => unreachable!(),
            }
            match &dir as &str {
                "R" => tail_pos2 = move_dir(n, &mut visited2, tail_pos2, Point2D::new(1, 0)),
                "U" => tail_pos2 = move_dir(n, &mut visited2, tail_pos2, Point2D::new(0, 1)),
                "L" => tail_pos2 = move_dir(n, &mut visited2, tail_pos2, Point2D::new(-1, 0)),
                "D" => tail_pos2 = move_dir(n, &mut visited2, tail_pos2, Point2D::new(0, -1)),
                _ => unreachable!(),
            }
        }
        io.write("Part 1: ");
        io.writeln(visited1.len());
        io.write("Part 2: ");
        io.writeln(visited2.len());
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day9", |b| {
        b.iter(|| bench(black_box(include_str!("../day9.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn move_dir(
    n: i32,
    visited: &mut HashSet<Point2D<i32>>,
    mut tail_pos: Vec<Point2D<i32>>,
    dir: Point2D<i32>,
) -> Vec<Point2D<i32>> {
    for _ in 0..n {
        tail_pos[0] += dir;

        for i in 0..tail_pos.len() - 1 {
            if (tail_pos[i].clone() - tail_pos[i + 1].clone()).len() > 1.5 {
                tail_pos[i + 1] = move_tail(tail_pos[i], tail_pos[i + 1]);
            }
        }
        visited.insert(tail_pos[tail_pos.len() - 1]);
    }
    tail_pos
}

fn move_tail(head_pos: Point2D<i32>, mut tail_pos: Point2D<i32>) -> Point2D<i32> {
    if head_pos.x > tail_pos.x {
        tail_pos.x += 1;
    } else if head_pos.x < tail_pos.x {
        tail_pos.x -= 1;
    }
    if head_pos.y > tail_pos.y {
        tail_pos.y += 1;
    } else if head_pos.y < tail_pos.y {
        tail_pos.y -= 1;
    }
    tail_pos
}
