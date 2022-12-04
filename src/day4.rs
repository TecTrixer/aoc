use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day4.txt");

    let mut res1 = 0;
    let mut res2 = 0;
    for line in io.lines() {
        let mut split = line.split(',');
        let first = split.next().unwrap();
        let mut range1 = first.split('-');
        let t1 = range1.next().unwrap().parse::<usize>().unwrap();
        let t2 = range1.next().unwrap().parse::<usize>().unwrap();
        let sec = split.next().unwrap();
        let mut range2 = sec.split('-');
        let t3 = range2.next().unwrap().parse::<usize>().unwrap();
        let t4 = range2.next().unwrap().parse::<usize>().unwrap();

        if t1 <= t3 && t2 >= t4 || t3 <= t1 && t4 >= t2 {
            res1 += 1;
        }

        if t2 >= t4 {
            if t1 <= t4 {
                res2 += 1;
            }
        } else if t2 >= t3 {
            res2 += 1;
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}
