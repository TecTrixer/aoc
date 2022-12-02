use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day2.txt");
    let mut res1 = 0;
    let mut res2 = 0;
    for mut line in io.line_io() {
        let (opp, me): (String, String) = line.tuple();
        res1 += play1(&opp, &me);
        res2 += play2(&opp, &me);
    }
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}

fn play1(opp: &str, me: &str) -> usize {
    match opp {
        "A" => match me {
            "X" => 3 + 1,
            "Y" => 6 + 2,
            "Z" => 0 + 3,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => 6 + 1,
            "Y" => 0 + 2,
            "Z" => 3 + 3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn play2(opp: &str, me: &str) -> usize {
    match opp {
        "A" => match me {
            "X" => 0 + 3,
            "Y" => 3 + 1,
            "Z" => 6 + 2,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => 0 + 2,
            "Y" => 3 + 3,
            "Z" => 6 + 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
