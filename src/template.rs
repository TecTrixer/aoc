use cp_rs::io::*;
#[allow(unused_mut)]
fn main() {
    let mut io = Io::from_file("dayX.txt");
    let mut res = 0;
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res);
}
