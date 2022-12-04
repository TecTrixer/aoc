use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day4.txt");
    let mut res1 = 0;
    let mut res2 = 0;
    for r in io.pnums::<isize>().chunks(4) {
        if r[0] <= r[2] && r[1] >= r[3] || r[0] >= r[2] && r[1] <= r[3] {
            res1 += 1;
        }
        if r[0] <= r[2] && r[1] >= r[2] || r[0] >= r[2] && r[0] <= r[3] {
            res2 += 1;
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}
