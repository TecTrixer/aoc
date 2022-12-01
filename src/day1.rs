use cp_rs::io::*;
fn main() {
    // Custom scanner like I/O handler
    let mut io = Io::from_file("day1.txt");

    // Vector with the sums of calories per Elf
    let mut sums: Vec<usize> = vec![];

    // Iterate through every Elf
    for mut block in io.blocks() {
        // Count the calories per Elf
        let mut res: usize = 0;
        for num in block.nums::<usize>() {
            res += num;
        }
        // Add the count to the sums list
        sums.push(res);
    }

    // Sort the list in reverse order -> most calories will be at first
    sums.sort();
    sums.reverse();

    io.write("Part 1: ");
    // Print most calories
    io.writeln(sums[0]);

    io.write("Part 2: ");
    // Print sum of top 3 calories
    io.writeln(sums[0..3].iter().sum::<usize>());
}
