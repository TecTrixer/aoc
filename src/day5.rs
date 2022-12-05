use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day5.txt");
    let mut blocks = io.blocks().into_iter();
    let mut stacks = blocks.next().unwrap();
    let mut commands = blocks.next().unwrap();
    let mut crates = vec![];
    for _ in 0..9 {
        crates.push(vec![]);
    }
    for line in stacks.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        if chars[0] == '1' {
            break;
        }
        let mut idx = 1;
        while idx < chars.len() {
            let c = chars[idx];
            let stack_num = (idx - 1) / 4;
            if c != ' ' {
                crates[stack_num].push(c);
            }
            idx += 4;
        }
    }
    for stack in crates.iter_mut() {
        stack.reverse();
    }
    let mut crates2 = crates.clone();
    for mut line in commands.line_io() {
        let (_, n, _, f, _, t): (String, usize, String, usize, String, usize) = line.tuple();
        // Part 1
        for _ in 0..n {
            let elem = crates[f - 1].pop().unwrap();
            crates[t - 1].push(elem);
        }

        // Part 2
        let mut temp = Vec::new();
        for _ in 0..n {
            let elem: char = crates2[f - 1].pop().unwrap();
            temp.push(elem);
        }
        for _ in 0..n {
            let elem = temp.pop().unwrap();
            crates2[t - 1].push(elem);
        }
    }
    io.write("Part 1: ");
    for i in 0..9 {
        io.write(crates[i].pop().unwrap());
    }
    io.nl();
    io.write("Part 2: ");
    for i in 0..9 {
        io.write(crates2[i].pop().unwrap());
    }
    io.nl();
}
