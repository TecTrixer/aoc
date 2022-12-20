use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day20.txt");
    let mut values = vec![];
    let mut indexes = vec![];
    let mut values2 = vec![];
    let mut indexes2 = vec![];
    for (i, mut line) in io.line_io().enumerate() {
        let value = line.read::<isize>();
        values.push(value);
        indexes.push(i as isize);
        values2.push(value * 811589153);
        indexes2.push(i as isize);
    }
    let len = values.len() as isize;
    let mut i = 0;
    while i < len {
        let from = indexes.iter().position(|&x| x == i).unwrap();
        if values[from] == 0 {
            i += 1;
            continue;
        }
        let to_init = values[from];
        let to: isize;
        to = (from as isize + to_init).rem_euclid(len - 1);
        let val = values.remove(from);
        values.insert(to as usize, val);
        let idx = indexes.remove(from);
        indexes.insert(to as usize, idx);
        i += 1;
    }
    for _ in 0..10 {
        let mut i = 0;
        while i < len {
            let from = indexes2.iter().position(|&x| x == i).unwrap();
            if values2[from] == 0 {
                i += 1;
                continue;
            }
            let to_init = values2[from];
            let to: isize;
            to = ((from as isize + to_init) % (len - 1) + len - 1) % (len - 1);
            let val = values2.remove(from);
            values2.insert(to as usize, val);
            let idx = indexes2.remove(from);
            indexes2.insert(to as usize, idx);
            i += 1;
        }
    }
    let zero_pos = values.iter().position(|&x| x == 0).unwrap();
    let idx1 = (zero_pos + 1000) % len as usize;
    let idx2 = (zero_pos + 2000) % len as usize;
    let idx3 = (zero_pos + 3000) % len as usize;
    let res = values[idx1] + values[idx2] + values[idx3];
    let zero_pos2 = values2.iter().position(|&x| x == 0).unwrap();
    let idx21 = (zero_pos2 + 1000) % len as usize;
    let idx22 = (zero_pos2 + 2000) % len as usize;
    let idx23 = (zero_pos2 + 3000) % len as usize;
    let res2 = values2[idx21] + values2[idx22] + values2[idx23];
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res2);
}
