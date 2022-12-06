use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day6.txt");
    let mut last = vec![];
    let mut res1 = usize::MAX;
    let mut res2 = usize::MAX;
    for (i, c) in io.chars().iter().enumerate() {
        for idx in 0..last.len() {
            if last[idx] == c {
                for _ in 0..=idx {
                    last.remove(0);
                }
                break;
            }
        }
        last.push(c);
        if last.len() >= 4 && i + 1 < res1 {
            res1 = i + 1;
        }
        if last.len() >= 14 {
            res2 = i + 1;
            break;
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}
