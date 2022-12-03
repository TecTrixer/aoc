use cp_rs::io::*;
use std::collections::HashSet;
fn main() {
    let mut io = Io::from_file("day3.txt");
    let mut res1 = 0;
    let mut res2 = 0;
    for lines in io.line_io().collect::<Vec<_>>().chunks_mut(3) {
        let first = lines[0].chars();
        let sec = lines[1].chars();
        let third = lines[2].chars();
        let words = vec![first.clone(), sec.clone(), third.clone()];
        // First part for every line
        for i in 0..3 {
            let word = &words[i];
            let mid = word.len() / 2;
            let (first, sec) = word.split_at(mid);
            let mut set = HashSet::new();
            for c in sec {
                set.insert(c);
            }
            let mut same = '0';
            for c in first {
                if set.contains(c) {
                    same = *c;
                    break;
                }
            }
            res1 += convert(same);
        }
        // Second part for every 3 lines
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        for c in sec {
            set1.insert(c);
        }
        for c in third {
            set2.insert(c);
        }
        let mut same = '0';
        for c in first {
            if set1.contains(&c) && set2.contains(&c) {
                same = c;
                break;
            }
        }
        res2 += convert(same);
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}

fn convert(c: char) -> usize {
    if c.is_lowercase() {
        (c as u8 - 'a' as u8 + 1) as usize
    } else {
        (c as u8 - 'A' as u8 + 27) as usize
    }
}
