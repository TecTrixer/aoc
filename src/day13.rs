use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day13.txt");
    let mut res = 0;
    let mut idx = 0;
    let mut list = vec![];
    for mut block in io.blocks() {
        let first = block.read_line().chars().collect::<Vec<char>>();
        let first_entry = parse(&first[..]).0;
        let sec = block.read_line().chars().collect::<Vec<char>>();
        let sec_entry = parse(&sec[..]).0;
        if let Some(o) = first_entry.partial_cmp(&sec_entry) {
            if o == std::cmp::Ordering::Less {
                res += idx + 1;
            }
        }
        list.push(first_entry);
        list.push(sec_entry);
        idx += 1;
    }
    let div1 = Entry::List(vec![Entry::List(vec![Entry::Num(2)])]);
    let div2 = Entry::List(vec![Entry::List(vec![Entry::Num(6)])]);
    list.push(div1.clone());
    list.push(div2.clone());
    list.sort();
    let mut res2 = 1;
    for (i, e) in list.iter().enumerate() {
        if e == &div1 {
            res2 *= i + 1;
        } else if e == &div2 {
            res2 *= i + 1;
        }
    }
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res2);
}

fn parse(input: &[char]) -> (Entry, usize) {
    let mut element = vec![];
    let mut i = 1;
    let mut num = 0;
    let mut has_num = false;
    while i < input.len() {
        match input[i] {
            '[' => {
                let res = parse(&input[i..]);
                i += res.1;
                element.push(res.0);
            }
            ']' => {
                if has_num {
                    element.push(Entry::Num(num));
                }
                return (Entry::List(element), i);
            }
            ',' => {
                if has_num {
                    element.push(Entry::Num(num));
                    num = 0;
                    has_num = false;
                }
            }
            d => {
                has_num = true;
                num *= 10;
                num += d as usize - '0' as usize;
            }
        }
        i += 1;
    }
    return (Entry::List(element), i);
}

#[derive(Ord, Eq, PartialEq, Debug, Clone)]
enum Entry {
    List(Vec<Entry>),
    Num(usize),
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Entry::List(l) => match other {
                Entry::List(lo) => {
                    let order = l.iter().zip(lo).map(|(a, b)| a.partial_cmp(b)).find(|x| {
                        if let Some(o) = x {
                            if o != &std::cmp::Ordering::Equal {
                                return true;
                            }
                        }
                        false
                    });
                    if let Some(Some(o)) = order {
                        match o {
                            std::cmp::Ordering::Less => {
                                return Some(std::cmp::Ordering::Less);
                            }
                            std::cmp::Ordering::Equal => unreachable!(),
                            std::cmp::Ordering::Greater => {
                                return Some(std::cmp::Ordering::Greater);
                            }
                        }
                    } else {
                        if l.len() < lo.len() {
                            return Some(std::cmp::Ordering::Less);
                        } else if l.len() > lo.len() {
                            return Some(std::cmp::Ordering::Greater);
                        } else {
                            return Some(std::cmp::Ordering::Equal);
                        }
                    }
                }
                Entry::Num(no) => {
                    return self.partial_cmp(&Entry::List(vec![Entry::Num(*no)]));
                }
            },
            Entry::Num(n) => match other {
                Entry::List(_) => {
                    return Entry::List(vec![Entry::Num(*n)]).partial_cmp(other);
                }
                Entry::Num(no) => {
                    return Some(n.cmp(no));
                }
            },
        }
    }
}
