use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day6.txt");
    let string = io.chars();
    let mut last = vec![];
    let mut res1 = usize::MAX;
    let mut res2 = usize::MAX;
    for (i, c) in string.iter().enumerate() {
        let mut idx = 0;
        while idx < last.len() {
            if last[idx] == c {
                for _ in 0..=idx {
                    last.remove(0);
                }
                break;
            }
            idx+= 1;
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
