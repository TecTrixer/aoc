use cp_rs::io::*;
use std::collections::HashSet;
const SIZE: isize = 4000000;
fn main() {
    let mut io = Io::from_file("day15.txt");
    let mut input = vec![];
    for nums in io.nums().chunks(4) {
        let (sx, sy, bx, by): (isize, isize, isize, isize) = (nums[0], nums[1], nums[2], nums[3]);
        input.push((sx, sy, bx, by));
    }
    let mut res1 = 0;
    let mut beacons = HashSet::new();
    let mut ranges = vec![];
    for (sx, sy, bx, by) in input.iter() {
        let dist = (bx - sx).abs() + (by - sy).abs();
        let ydist = (2000000 - sy).abs();
        if by == &2000000 {
            beacons.insert(bx);
        }
        if dist >= ydist {
            let newstart = sx - (dist - ydist);
            let newend = sx + (dist - ydist);
            ranges.push((newstart, newend));
        }
    }
    ranges.sort_by(|(a, b), (c, d)| match a.cmp(c) {
        std::cmp::Ordering::Equal => return b.cmp(d),
        x => return x,
    });
    let mut idx = 0;
    let mut until = ranges[0].0 - 1;
    while idx < ranges.len() {
        let (a, b) = ranges[idx];
        let mut size = b - a + 1;
        if a <= until {
            size -= until - a + 1;
        }
        until = if b > until { b } else { until };
        if size >= 0 {
            res1 += size;
        }
        idx += 1;
    }
    res1 -= beacons.len() as isize;
    io.write("Part 1: ");
    io.writeln(res1);
    let mut x = 0;
    let mut y = 0;
    let mut pos = vec![];
    let mut set = vec![];

    for row in 0..=SIZE {
        pos.clear();
        set.clear();
        for (sx, sy, bx, by) in input.iter() {
            let dist = (bx - sx).abs() + (by - sy).abs();
            let ydist = (row - sy).abs();
            let r = dist - ydist;
            if r < 0 {
                continue;
            }
            let newstart = sx - (dist - ydist);
            let newend = sx + (dist - ydist);
            pos.push((newstart, newend));
            set.push(newstart - 1);
            set.push(newend + 1);
        }
        'outer: for i in set.iter() {
            for (a, b) in pos.iter() {
                if *i < 0 || *i > SIZE || (i >= a && i <= b) {
                    continue 'outer;
                }
            }
            x = *i;
            y = row;
        }
    }
    io.write("Part 2: ");
    io.writeln(x * 4000000 + y);
}
