use cp_rs::io::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn bench() {
    let mut io = Io::from_file("day21.txt");
    let mut monkeys: HashMap<String, Operation> = HashMap::new();
    let mut results: HashMap<String, usize> = HashMap::new();
    let mut root_f = String::new();
    let mut root_s = String::new();
    for line in io.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        let first = chars[0..4].iter().collect::<String>();
        if chars[6].is_digit(10) {
            let digit = parse_digit(&chars);
            monkeys.insert(first, Operation::Literal(digit));
        } else {
            let sec = chars[6..10].iter().collect::<String>();
            let third = chars[13..17].iter().collect::<String>();
            if first == "root".to_string() {
                root_f = sec.clone();
                root_s = third.clone();
            }
            match chars[11] {
                '+' => monkeys.insert(first, Operation::Add(sec, third)),
                '-' => monkeys.insert(first, Operation::Subtract(sec, third)),
                '*' => monkeys.insert(first, Operation::Times(sec, third)),
                '/' => monkeys.insert(first, Operation::Divide(sec, third)),
                _ => unreachable!(),
            };
        }
    }
    let root = "root".to_string();
    let res = get_value(&root, &monkeys, &mut results);
    results.clear();
    let (wo_humn, value) = if let Some(x) = get_path_wo_humn(&root_f, &monkeys, &mut results) {
        (root_s, x)
    } else if let Some(x) = get_path_wo_humn(&root_s, &monkeys, &mut results) {
        (root_f, x)
    } else {
        unreachable!();
    };
    results.clear();
    let res2 = get_value2(&wo_humn, &monkeys, &mut results, value);
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res2);
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day21", |b| {
        b.iter(|| bench())
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

#[derive(Debug, Clone, Hash)]
enum Operation {
    Add(String, String),
    Subtract(String, String),
    Times(String, String),
    Divide(String, String),
    Literal(usize),
}

fn parse_digit(chars: &Vec<char>) -> usize {
    let mut val = chars[6] as usize - '0' as usize;
    let mut idx = 7;
    while idx < chars.len() {
        val *= 10;
        val += chars[idx] as usize - '0' as usize;
        idx += 1;
    }
    val
}

fn get_value(
    curr: &String,
    monkeys: &HashMap<String, Operation>,
    results: &mut HashMap<String, usize>,
) -> usize {
    if let Some(res) = results.get(curr) {
        return *res;
    }
    if let Some(op) = monkeys.get(curr) {
        let val = match op {
            Operation::Add(a, b) => {
                let a = get_value(&a, monkeys, results);
                let b = get_value(&b, monkeys, results);
                a + b
            }
            Operation::Subtract(a, b) => {
                let a = get_value(&a, monkeys, results);
                let b = get_value(&b, monkeys, results);
                a - b
            }
            Operation::Times(a, b) => {
                let a = get_value(&a, monkeys, results);
                let b = get_value(&b, monkeys, results);
                a * b
            }
            Operation::Divide(a, b) => {
                let a = get_value(&a, monkeys, results);
                let b = get_value(&b, monkeys, results);
                a / b
            }
            Operation::Literal(x) => *x,
        };
        results.insert(curr.to_string(), val);
        return val;
    }
    unreachable!();
}

fn get_path_wo_humn(
    curr: &String,
    monkeys: &HashMap<String, Operation>,
    results: &mut HashMap<String, usize>,
) -> Option<usize> {
    if curr == "humn" {
        return None;
    }
    if let Some(res) = results.get(curr) {
        return Some(*res);
    }
    if let Some(op) = monkeys.get(curr) {
        let val = match op {
            Operation::Add(a, b) => {
                let a = match get_path_wo_humn(&a, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                let b = match get_path_wo_humn(&b, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                a + b
            }
            Operation::Subtract(a, b) => {
                let a = match get_path_wo_humn(&a, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                let b = match get_path_wo_humn(&b, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                a - b
            }
            Operation::Times(a, b) => {
                let a = match get_path_wo_humn(&a, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                let b = match get_path_wo_humn(&b, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                a * b
            }
            Operation::Divide(a, b) => {
                let a = match get_path_wo_humn(&a, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                let b = match get_path_wo_humn(&b, monkeys, results) {
                    Some(x) => x,
                    None => return None,
                };
                a / b
            }
            Operation::Literal(x) => *x,
        };
        results.insert(curr.to_string(), val);
        return Some(val);
    }
    unreachable!();
}

fn get_value2(
    curr: &String,
    monkeys: &HashMap<String, Operation>,
    results: &mut HashMap<String, usize>,
    value: usize,
) -> usize {
    if curr == "humn" {
        return value;
    }
    if let Some(op) = monkeys.get(curr) {
        match op {
            Operation::Add(a, b) => {
                if let Some(x) = get_path_wo_humn(a, monkeys, results) {
                    return get_value2(b, monkeys, results, value - x);
                } else if let Some(x) = get_path_wo_humn(b, monkeys, results) {
                    return get_value2(a, monkeys, results, value - x);
                }
            }
            Operation::Subtract(a, b) => {
                if let Some(x) = get_path_wo_humn(a, monkeys, results) {
                    return get_value2(b, monkeys, results, x - value);
                } else if let Some(x) = get_path_wo_humn(b, monkeys, results) {
                    return get_value2(a, monkeys, results, value + x);
                }
            }
            Operation::Times(a, b) => {
                if let Some(x) = get_path_wo_humn(a, monkeys, results) {
                    return get_value2(b, monkeys, results, value / x);
                } else if let Some(x) = get_path_wo_humn(b, monkeys, results) {
                    return get_value2(a, monkeys, results, value / x);
                }
            }
            Operation::Divide(a, b) => {
                if let Some(x) = get_path_wo_humn(a, monkeys, results) {
                    return get_value2(b, monkeys, results, x / value);
                } else if let Some(x) = get_path_wo_humn(b, monkeys, results) {
                    return get_value2(a, monkeys, results, value * x);
                }
            }
            Operation::Literal(x) => {
                return *x;
            }
        }
    }
    unreachable!();
}
