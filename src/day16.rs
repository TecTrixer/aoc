use std::collections::HashMap;

use cp_rs::io::*;
fn main() {
    let mut io = Io::from_file("day16.txt");
    // let mut io = Io::from_file("test.txt");
    // let mut nodes: HashMap<String, (usize, bool)> = HashMap::new();
    // let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut node_map = HashMap::new();
    let mut nodes: Vec<usize> = vec![];
    let mut edges: Vec<Vec<usize>> = vec![];
    let mut edges_temp: Vec<Vec<String>> = vec![];
    let mut node_num = 0;
    for mut line in io.line_io() {
        let (source, rate): (String, usize) = line.tuple();
        let destinations: Vec<String> =
            line.read_all().split(", ").map(|x| x.to_string()).collect();
        node_map.insert(source, node_num);
        nodes.push(rate);
        edges_temp.push(destinations);
        node_num += 1;
    }
    let mut non_zero = vec![];
    for node in nodes.iter() {
        if node > &0 {
            non_zero.push(*node);
        }
    }
    for vec in edges_temp.into_iter() {
        let mut dests = vec![];
        for s in vec.into_iter() {
            if let Some(idxn) = node_map.get(&s) {
                dests.push(*idxn);
            }
        }
        edges.push(dests);
    }

    let mut max = 0;
    let mut map = HashMap::new();
    for i in 100..((2 as usize).pow(non_zero.len() as u32)) {
        dbg!(i);
        let num = num_to_num(i, &non_zero);
        let val = find_max(
            &mut nodes,
            0,
            &mut edges,
            26,
            *node_map.get("AA").unwrap(),
            *node_map.get("AA").unwrap(),
            &mut map,
            num,
        );
        map.clear();
        if val > max {
            max = val;
        }
        dbg!(val);
    }
    let res = 0;
    io.write("Part 1: ");
    io.writeln(max);
    io.write("Part 2: ");
    io.writeln(res);
}

fn num_to_num(compact_set: usize, nodes: &Vec<usize>) -> usize {
    let mut res = 0;
    for i in 0..15 {
        let num = 1 << i;
        if num & compact_set != 0 {
            res += 1 << nodes[i];
        }
    }
    res
}

fn is_in_set(set: usize, idx: usize) -> bool {
    ((1 << idx) & set) != 0
}

fn set_idx_in_set(set: usize, idx: usize) -> usize {
    (1 << idx) | set
}

fn find_max(
    nodes: &Vec<usize>,
    bitset: usize,
    edges: &Vec<Vec<usize>>,
    iter: usize,
    curr_node: usize,
    elephant_node: usize,
    cache: &mut HashMap<(usize, usize, usize, usize), u16>,
    me_open: usize,
) -> u16 {
    if iter == 0 {
        return 0;
    }
    if let Some(res) = cache.get(&(iter, curr_node, elephant_node, bitset)) {
        return *res;
    }
    let mut max = 0;
    // see if activating current node helps
    let can_try = !is_in_set(bitset, curr_node) && nodes[curr_node] != 0;
    // && is_in_set(me_open, curr_node);
    let ecan_try = !is_in_set(bitset, elephant_node) && nodes[elephant_node] != 0;
    // && !is_in_set(me_open, elephant_node);
    if can_try && ecan_try && elephant_node != curr_node {
        let bitset_new = set_idx_in_set(bitset, curr_node);
        let bitset_new = set_idx_in_set(bitset_new, elephant_node);
        let val = find_max(
            nodes,
            bitset_new,
            edges,
            iter - 1,
            curr_node,
            elephant_node,
            cache,
            me_open,
        );
        let add1 = (iter - 1) * nodes[curr_node];
        let add2 = (iter - 1) * nodes[elephant_node];
        if add1 as u16 + add2 as u16 + val > max {
            max = add1 as u16 + add2 as u16 + val;
        }
    }

    if can_try {
        let bitset_new = set_idx_in_set(bitset, curr_node);
        let add = nodes[curr_node] * (iter - 1);
        for edest in edges[elephant_node].iter() {
            let val = find_max(
                nodes,
                bitset_new,
                edges,
                iter - 1,
                curr_node,
                *edest,
                cache,
                me_open,
            );
            if add as u16 + val > max {
                max = add as u16 + val;
            }
        }
    }
    if ecan_try {
        let bitset_new = set_idx_in_set(bitset, elephant_node);
        let add = nodes[elephant_node] * (iter - 1);
        for dest in edges[curr_node].iter() {
            let val = find_max(
                nodes,
                bitset_new,
                edges,
                iter - 1,
                *dest,
                elephant_node,
                cache,
                me_open,
            );
            if add as u16 + val > max {
                max = add as u16 + val;
            }
        }
    }

    // check every possible destination
    for dest in edges[curr_node].iter() {
        for edest in edges[elephant_node].iter() {
            let val = find_max(
                nodes,
                bitset,
                edges,
                iter - 1,
                *dest,
                *edest,
                cache,
                me_open,
            );
            if val > max {
                max = val;
            }
        }
    }
    cache.insert((iter, curr_node, elephant_node, bitset), max);
    if iter > 20 {
        dbg!(iter);
    }
    max
}
