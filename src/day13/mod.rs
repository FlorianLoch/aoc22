use std::cmp::{max, Ordering};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str::Chars;
use crate::puzzle;

#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    value: i32,
    is_leaf: bool,
}

impl Node {
    fn new() -> Self {
        return Node { children: vec![], value: 0, is_leaf: false };
    }

    fn new_leaf(value: i32) -> Self {
        let mut leaf = Node::new();

        leaf.value = value;
        leaf.is_leaf = true;

        return leaf;
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn is_leaf(&self) -> bool {
        return self.is_leaf;
    }

    fn compare(&self, other: &Node) -> Ordering {
        // Both are integers
        if self.is_leaf() && other.is_leaf() {
            if self.value < other.value {
                return Ordering::Less;
            }

            if self.value == other.value {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        }

        // One of the nodes is a list and not a leaf/integer
        if self.is_leaf() {
            let mut as_node = Node::new();
            as_node.add_child(self.clone());

            return as_node.compare(other);
        }

        if other.is_leaf() {
            let mut as_node = Node::new();
            as_node.add_child(other.clone());

            return self.compare(&as_node);
        }

        // Both are lists
        let self_len = self.children.len();
        let other_len = other.children.len();

        for i in 0..max(self_len, other_len) {
            if i == self_len {
                if i == other_len {
                    return Ordering::Equal;
                }

                return Ordering::Less;
            }

            if i == other_len {
                return Ordering::Greater;
            }

            match self.children[i].compare(&other.children[i]) {
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => {
                    return Ordering::Greater;
                }
                Ordering::Equal => {
                    // We have to keep comparing...
                }
            }
        }

        return Ordering::Equal;
    }

    fn to_string(&self) -> String {
        if self.is_leaf() {
            return self.value.to_string();
        }

        let mut s = String::from("[");

        for i in 0..self.children.len() {
            s.push_str(&self.children[i].to_string());

            if i + 1 < self.children.len() {
                s.push_str(", ")
            }
        }

        s.push(']');

        return s;
    }
}

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let all_lines = puzzle::read_all_lines(lines);

    solve_1(&all_lines, test);
    solve_2(&all_lines, test);
}

fn solve_1(lines: &Vec<String>, test: bool) {
    let mut list_pairs = Vec::<(Node, Node)>::new();

    let mut lines = lines.iter();

    loop {
        let line_res = lines.next();

        if line_res.is_none() {
            break;
        }

        let line = line_res.unwrap();

        if line.is_empty() {
            continue;
        }

        let a = parse_line(line);

        let line_res = lines.next();
        let line = line_res.unwrap();
        let b = parse_line(line);

        list_pairs.push((a, b));
    }

    let mut in_right_order_sum = 0;

    for (i, list) in list_pairs.iter().enumerate() {
        if is_in_right_order(&list.0, &list.1, test) {
            in_right_order_sum += i + 1;
        }
    }

    println!("Part 1: sum of indices being in right order: {}", in_right_order_sum);
}

fn solve_2(lines: &Vec<String>, _: bool) {
    let mut nodes = Vec::<Node>::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        nodes.push(parse_line(line));
    }

    const DIVIDER_PACKET_1: &str = "[[2]]";
    const DIVIDER_PACKER_2: &str = "[[6]]";

    nodes.push(parse_line(&String::from(DIVIDER_PACKET_1)));
    nodes.push(parse_line(&String::from(DIVIDER_PACKER_2)));

    nodes.sort_by(|a, b| {
        a.compare(b)
    });

    let (mut idx_first, mut idx_second) = (0, 0);

    for (i, node) in nodes.iter().enumerate() {
        let as_str = node.to_string();

        if as_str == DIVIDER_PACKET_1 {
            idx_first = i + 1;
        } else if as_str == DIVIDER_PACKER_2 {
            idx_second = i + 1;
        }

        // if test {
        //     println!("{}: {}", i, node.to_string());
        // }
    }

    println!("Part 2: decoder key: {}", idx_first * idx_second);
}

fn is_in_right_order(left: &Node, right: &Node, _: bool) -> bool {
    let res = left.compare(right);

    // if test {
    //     println!("==>");
    //     println!("L: {}", left.to_string());
    //     println!("R: {}", right.to_string());
    //     println!("Result: {:?}", res);
    //     println!();
    // }

    return res == Ordering::Less;
}

fn parse_line(line: &String) -> Node {
    let mut char_stream = line.chars();

    return parse_list(&mut char_stream).children[0].clone();
}

fn parse_list(char_stream: &mut Chars) -> Node {
    let mut node = Node::new();

    let mut digits = String::new();

    loop {
        let next_res = char_stream.next();

        if next_res.is_none() {
            return node;
        }

        let c = next_res.unwrap();

        if c.is_digit(10) {
            digits.push(c);

            continue;
        }

        if !digits.is_empty() {
            let as_num: i32 = digits.parse().expect("Failed to parse number");

            node.add_child(Node::new_leaf(as_num));

            digits.clear();
        }

        if c == '[' {
            let child = parse_list(char_stream);

            node.add_child(child);

            continue;
        }

        if c == ']' {
            return node;
        }

        // We simply ignore commas and whitespace
    }
}