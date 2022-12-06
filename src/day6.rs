use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day6.txt");
    let string = io.chars();
    let mut last = 0;
    let mut res1 = usize::MAX;
    let mut res2 = usize::MAX;
    for (i, c) in string.iter().enumerate() {
        for idx in last..i {
            if string[idx] == *c {
                last = idx + 1;
                break;
            }
        }
        let len = i - last + 1;
        if len >= 4 && i + 1 < res1 {
            res1 = i + 1;
        }
        if len >= 14 {
            res2 = i + 1;
            break;
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}
