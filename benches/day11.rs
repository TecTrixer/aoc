use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::VecDeque;

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut monkeys1 = vec![];
        for mut line in io.blocks() {
            line.skipn(4);
            let items: VecDeque<usize> = Io::from_str(&line.read_line()).nums().into();
            line.skipn(4);
            let op = line.read_char() as u8;
            let op_part = match &line.read::<String>() as &str {
                "old" => Op::Old,
                num => Op::Num(num.parse::<usize>().unwrap()),
            };
            line.skipn(3);
            let div_num = line.read::<usize>();
            line.skipn(5);
            let true_num = line.read::<usize>();
            line.skipn(5);
            let false_num = line.read::<usize>();
            monkeys1.push(Monkey {
                items,
                op,
                op_part,
                div_num,
                true_num,
                false_num,
                count: 0,
            });
        }
        let modulo: usize = monkeys1.iter().map(|x| x.div_num).product();
        let mut monkeys2 = monkeys1.clone();
        for _ in 0..20 {
            for m in 0..monkeys1.len() {
                for _ in 0..monkeys1[m].items.len() {
                    monkeys1[m].count += 1;
                    let mut worry_lvl = monkeys1[m].items.pop_front().unwrap();
                    worry_lvl %= modulo;
                    match monkeys1[m].op {
                        b'+' => match &monkeys1[m].op_part {
                            Op::Num(x) => {
                                worry_lvl += x;
                            }
                            Op::Old => {
                                worry_lvl += worry_lvl;
                            }
                        },
                        b'*' => match &monkeys1[m].op_part {
                            Op::Num(x) => {
                                worry_lvl *= x;
                            }
                            Op::Old => {
                                worry_lvl *= worry_lvl;
                            }
                        },
                        _ => unreachable!(),
                    }
                    worry_lvl /= 3;
                    if worry_lvl % monkeys1[m].div_num == 0 {
                        let true_num = monkeys1[m].true_num;
                        monkeys1[true_num].items.push_back(worry_lvl);
                    } else {
                        let false_num = monkeys1[m].false_num;
                        monkeys1[false_num].items.push_back(worry_lvl);
                    }
                }
            }
        }
        for _ in 0..10000 {
            for m in 0..monkeys2.len() {
                for _ in 0..monkeys2[m].items.len() {
                    monkeys2[m].count += 1;
                    let mut worry_lvl = monkeys2[m].items.pop_front().unwrap();
                    worry_lvl %= modulo;
                    match monkeys2[m].op {
                        b'+' => match &monkeys2[m].op_part {
                            Op::Num(x) => {
                                worry_lvl += x;
                            }
                            Op::Old => {
                                worry_lvl += worry_lvl;
                            }
                        },
                        b'*' => match &monkeys2[m].op_part {
                            Op::Num(x) => {
                                worry_lvl *= x;
                            }
                            Op::Old => {
                                worry_lvl *= worry_lvl;
                            }
                        },
                        _ => unreachable!(),
                    }
                    if worry_lvl % monkeys2[m].div_num == 0 {
                        let true_num = monkeys2[m].true_num;
                        monkeys2[true_num].items.push_back(worry_lvl);
                    } else {
                        let false_num = monkeys2[m].false_num;
                        monkeys2[false_num].items.push_back(worry_lvl);
                    }
                }
            }
        }
        let mut counts1: Vec<usize> = monkeys1.into_iter().map(|x| x.count).collect();
        counts1.sort();
        counts1.reverse();
        let mut counts2: Vec<usize> = monkeys2.into_iter().map(|x| x.count).collect();
        counts2.sort();
        counts2.reverse();
        io.write("Part 1: ");
        io.writedln(counts1[0] * counts1[1]);
        io.write("Part 2: ");
        io.writedln(counts2[0] * counts2[1]);
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day11", |b| {
        b.iter(|| bench(black_box(include_str!("../day11.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: u8,
    op_part: Op,
    div_num: usize,
    true_num: usize,
    false_num: usize,
    count: usize,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Num(usize),
    Old,
}
