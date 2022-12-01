use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day1.txt");

    let mut nums = vec![];
    for mut block in io.blocks() {
        let mut res: isize = 0;
        for num in block.nums::<isize>() {
            res += num;
        }
        nums.push(res);
    }
    nums.sort();
    let len = nums.len();
    io.write("Part 1: ");
    io.writeln(nums[len - 1]);
    io.write("Part 2: ");
    io.writeln(nums[len - 1] + nums[len - 2] + nums[len -3]);
}
