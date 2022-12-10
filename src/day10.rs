use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day10.txt");
    let mut res = 0;
    let mut cycle: usize = 1;
    let mut reg: isize = 1;
    let mut display = [['.'; 40]; 6];
    for mut line in io.line_io() {
        if cycle % 40 == 20 {
            res += cycle as isize * reg;
        }
        draw(&mut display, reg, cycle);
        match &line.read::<String>() as &str {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 1;
                if cycle % 40 == 20 {
                    res += cycle as isize * reg;
                }
                draw(&mut display, reg, cycle);
                let val = line.read::<isize>();
                reg += val;
                cycle += 1;
            }
            _ => unreachable!(),
        }
    }
    draw(&mut display, reg, cycle);
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2:");
    io.nl();
    for row in display {
        for pxl in row {
            io.write(pxl);
        }
        io.nl();
    }
}

fn draw(display: &mut [[char; 40]; 6], reg: isize, cycle: usize) {
    let row = (cycle - 1) / 40;
    let col = (cycle - 1) % 40;
    if col == reg as usize || col == reg as usize + 1 || col == reg as usize - 1 {
        display[row][col] = '#';
    }
}
