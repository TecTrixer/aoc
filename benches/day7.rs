use std::collections::HashMap;

use cp_rs::io::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(input: String) -> String {
    let mut output = Vec::new();
    {
        let mut io = Io::with_reader_and_writer(input.as_bytes(), &mut output);
        let mut map: HashMap<String, Vec<Child>> = HashMap::new();
        let mut curr_dir = String::from("r");
        map.insert(curr_dir.clone(), vec![]);
        let _ = io.read_line();
        for mut line in io.line_io() {
            let first = line.read::<String>();
            let sec = line.read::<String>();
            // is command
            if first == String::from("$") {
                // change directory
                if sec == String::from("cd") {
                    let dir = line.read::<String>();
                    // go back one dir
                    if dir == String::from("..") {
                        curr_dir = curr_dir.rsplit_once('/').unwrap().0.to_string();
                        // go one dir deeper
                    } else {
                        add_dir(&curr_dir, &dir, &mut map);
                        curr_dir = curr_dir + "/" + &dir;
                    }
                    // list contents
                } else {
                    continue;
                }
                // is directory
            } else if first == String::from("dir") {
                add_dir(&curr_dir, &sec, &mut map);
                // is file
            } else {
                let num = first.parse::<usize>().unwrap();
                let name = sec;
                add_file(&curr_dir, &name, &mut map, num);
                update_upper_dirs(&curr_dir, &mut map);
            }
        }
        let mut res = 0;
        let free = 70_000_000 - get_size_of_dir(&mut map, &"r".to_string());
        let needed = 30_000_000 - free;
        let mut res2 = usize::MAX;
        for list in map.into_values() {
            for x in list {
                match x {
                    Child::Dir(_, size) => {
                        if size <= 100000 {
                            res += size
                        }
                        if size >= needed && size < res2 {
                            res2 = size;
                        }
                    }
                    Child::File(_, _) => (),
                }
            }
        }
        io.write("Part 1: ");
        io.writeln(res);
        io.write("Part 2: ");
        io.writeln(res2);
    }
    std::str::from_utf8(output.as_slice()).unwrap().to_string()
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("day7", |b| {
        b.iter(|| bench(black_box(include_str!("../day7.txt").to_string())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

#[derive(Debug, Clone)]
enum Child {
    Dir(String, usize),
    File(String, usize),
}

fn get_size_of_dir(map: &HashMap<String, Vec<Child>>, dir: &String) -> usize {
    let content = map.get(dir).unwrap();
    let mut total = 0;
    for x in content {
        match x {
            Child::Dir(_, size) => total += size,
            Child::File(_, size) => total += size,
        }
    }
    total
}

fn add_dir(curr_dir: &String, new_dir: &String, map: &mut HashMap<String, Vec<Child>>) {
    let full_dir = curr_dir.clone() + "/" + new_dir;
    let content = map.get_mut(curr_dir).unwrap();
    let mut exists = false;
    for x in content.iter() {
        if match x {
            Child::Dir(name, _) => name == &full_dir,
            Child::File(_, _) => false,
        } {
            exists = true;
        }
    }
    if !exists {
        let new_dir = Child::Dir(full_dir.clone(), 0);
        content.push(new_dir);
    }
    if !map.contains_key(&full_dir) {
        map.insert(full_dir, vec![]);
    }
}

fn add_file(
    curr_dir: &String,
    new_file: &String,
    map: &mut HashMap<String, Vec<Child>>,
    size: usize,
) {
    let content = map.get_mut(curr_dir).unwrap();
    let mut exists = false;
    for x in content.iter() {
        if match x {
            Child::Dir(_, _) => false,
            Child::File(name, _) => name == new_file,
        } {
            exists = true;
        }
    }
    if !exists {
        let new_file = Child::File(new_file.clone(), size);
        content.push(new_file);
    }
}

fn update_upper_dirs(curr_dir: &String, map: &mut HashMap<String, Vec<Child>>) {
    if curr_dir.len() < 2 {
        return;
    }
    let (upper_dir, _) = curr_dir.rsplit_once('/').unwrap();
    let size = get_size_of_dir(map, curr_dir);
    let upper_content = map.get_mut(upper_dir).unwrap();
    for x in upper_content {
        match x {
            Child::Dir(name, old_size) => {
                if name == curr_dir {
                    *old_size = size
                }
            }
            Child::File(_, _) => (),
        }
    }
    update_upper_dirs(&upper_dir.to_string(), map);
}
