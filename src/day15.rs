use cp_rs::io::*;
use std::collections::HashSet;
const SIZE: isize = 20;
fn main() {
    // let mut io = Io::from_file("day15.txt");
    let mut io = Io::from_file("test.txt");
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
        let ydist = (SIZE / 2 - sy).abs();
        if by == &(SIZE / 2) {
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

    let mut tls = vec![];
    let mut trs = vec![];
    let mut brs = vec![];
    let mut bls = vec![];
    let mut tops = vec![];
    let mut bottoms = vec![];
    for (sx, sy, bx, by) in input {
        let dist = (bx - sx).abs() + (by - sy).abs();
        let sensor = Sensor {
            mx: sx,
            my: sy,
            r: dist,
        };
        tls.push(sensor.tl());
        bls.push(sensor.bl());
        brs.push(sensor.br());
        trs.push(sensor.tr());
    }
    for i in 0..bls.len() {
        let bl = &bls[i];
        for y in i..brs.len() {
            let br = &brs[y];
            tops.push(bl.intersect(br));
        }
    }
    for i in 0..tls.len() {
        let tl = &tls[i];
        for y in i..trs.len() {
            let tr = &trs[y];
            if tr.sx == 8 {
                dbg!(tr, tl, tl.intersect(tr));
            }
            bottoms.push(tl.intersect(tr));
        }
    }

    tops.sort_by(|a, b| a.partial_cmp(b).unwrap() );
    bottoms.sort_by(|a, b| a.partial_cmp(b).unwrap() );
    let mut idxt = 0;
    let mut idxb = 0;
    io.writedln(&bottoms);
    while idxt < tops.len() && idxb < bottoms.len() {
        let top = &tops[idxt];
        if !top.is_in_bound() {
            idxt += 1;
            continue;
        }
        let bottom = &bottoms[idxb];
        if !bottom.is_in_bound() {
            idxb += 1;
            continue;
        }
        if top.is_over(bottom) {
            io.writed(top);
            io.write(" - ");
            io.writedln(bottom);
        } else {
        }
    }
    io.write("Part 2: ");
    // io.writeln(x * 4000000 + y);
}

struct Sensor {
    mx: isize,
    my: isize,
    r: isize,
}

impl Sensor {
    fn tl(&self) -> Edge {
        let sx = self.mx - self.r;
        let sy = self.my;
        let ex = self.mx;
        let ey = self.my + self.r;
        Edge { sx, sy, ex, ey }
    }
    fn bl(&self) -> Edge {
        let sx = self.mx - self.r;
        let sy = self.my;
        let ex = self.mx;
        let ey = self.my - self.r;
        Edge { sx, sy, ex, ey }
    }
    fn tr(&self) -> Edge {
        let sx = self.mx;
        let sy = self.my + self.r;
        let ex = self.mx + self.r;
        let ey = self.my;
        Edge { sx, sy, ex, ey }
    }
    fn br(&self) -> Edge {
        let sx = self.mx;
        let sy = self.my - self.r;
        let ex = self.mx + self.r;
        let ey = self.my;
        Edge { sx, sy, ex, ey }
    }
}

#[derive(Debug)]
struct Edge {
    sx: isize,
    sy: isize,
    ex: isize,
    ey: isize,
}

impl Edge {
    fn intersect(&self, o: &Edge) -> Point {
        let x1 = self.sx;
        let y1 = self.sy;
        let x2 = self.ex;
        let y2 = self.ey;
        let xa = o.sx;
        let ya = o.sy;
        let xb = o.ex;
        let yb = o.ey;

        let m1 = (y2 - y1) as f64 / (x2 - x1) as f64;
        let b1 = y1 as f64 - m1 * x1 as f64;
        let ma = (yb - ya) as f64 / (xb - xa) as f64;
        let ba = ya as f64 - ma * xa as f64;
        let x = (ba - b1) / (m1 - ma);
        let y = m1 * x + b1;
        Point { x, y }
    }
}

#[derive(Debug, Copy, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn is_in_bound(&self) -> bool {
        self.x >= 0.0 && self.x <= SIZE as f64 && self.y >= 0.0 && self.y <= SIZE as f64
    }
    fn is_over(&self, o: &Point) -> bool {
        self.x == o.x && self.y + 2.0 == o.y
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.y < other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else {
            self.x.partial_cmp(&other.x)
        }
    }
}

#[test]
fn intersect() {
    let e1 = Edge {
        sx: 0,
        sy: 0,
        ex: 5,
        ey: 5,
    };
    let e2 = Edge {
        sx: 0,
        sy: 5,
        ex: 5,
        ey: 0,
    };
    let e3 = Edge {
        sx: -1,
        sy: 7,
        ex: 7,
        ey: -1,
    };
    let pr = Point { x: 3.0, y: 3.0 };
    assert!(e1.intersect(&e2) != pr);
    assert!(e1.intersect(&e3) == pr);
}
