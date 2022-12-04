use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day4.txt");
    let mut res1 = 0;
    let mut res2 = 0;
    for r in io.nums::<isize>().chunks(4) {
        let fl = r[0];
        let fr = r[1] * -1;
        let sl = r[2];
        let sr = r[3] * -1;
        if fl <= sl && fr >= sr || fl >= sl && fr <= sr {
            res1 += 1;
        }
        if fl <= sl && fr >= sl || fl >= sl && fl <= sr {
            res2 += 1;
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}
