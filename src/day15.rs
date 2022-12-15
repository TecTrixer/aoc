use std::collections::HashSet;

use cp_rs::io::*;
#[allow(unused_mut)]
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
    let mut y = 0;
    for i in 0..=SIZE {
        let mut res = 0;
        let mut ranges = vec![];
        for (sx, sy, bx, by) in input.iter() {
            let dist = (bx - sx).abs() + (by - sy).abs();
            let ydist = (i - sy).abs();
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
            let (mut a, mut b) = ranges[idx];
            a = if a > SIZE { SIZE } else { a };
            a = if a < 0 { 0 } else { a };
            b = if b > SIZE { SIZE } else { b };
            b = if b < 0 { 0 } else { b };
            let mut size = b - a + 1;
            if a <= until {
                size -= until - a + 1;
            }
            until = if b > until { b } else { until };
            if size >= 0 {
                res += size;
            }
            idx += 1;
        }
        if res < SIZE + 1 {
            y = i;
            break;
        }
    }
    let mut x = 0;
    for i in 0..=SIZE {
        let mut res = 0;
        let mut ranges = vec![];
        for (sx, sy, bx, by) in input.iter() {
            let dist = (bx - sx).abs() + (by - sy).abs();
            let xdist = (i - sx).abs();
            if dist >= xdist {
                let newstart = sy - (dist - xdist);
                let newend = sy + (dist - xdist);
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
            let (mut a, mut b) = ranges[idx];
            a = if a > SIZE { SIZE } else { a };
            a = if a < 0 { 0 } else { a };
            b = if b > SIZE { SIZE } else { b };
            b = if b < 0 { 0 } else { b };
            let mut size = b - a + 1;
            if a <= until {
                size -= until - a + 1;
            }
            until = if b > until { b } else { until };
            if size >= 0 {
                res += size;
            }
            idx += 1;
        }
        if res < SIZE + 1 {
            x = i;
            break;
        }
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(x * 4000000 + y);
}
