use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::ptr::null_mut;

pub fn solve(lines: &mut Lines<BufReader<File>>, test: bool) {
    let numbers: Vec<i32> = lines.map(|line| {
        line.expect("Failed to read line").parse().expect("Failed to parse number")
    }).collect();

    let mut node_store = Vec::<NumberNode>::with_capacity(numbers.len()-1);

    let mut head = &NumberNode::new(numbers[0]);

    // for i in 1..numbers.len() {
    //     node_store.push(NumberNode::new(numbers[i]));
    //
    //     let n = &node_store[i-1];
    //
    //     head.append(n);
    //
    //     head = n;
    // }

    if !test {
        return;
    }


}

#[derive(Debug, Copy, Clone)]
struct NumberNode<'list> {
    val: i32,
    processed: bool,
    initial_next: *mut NumberNode<'list>,
    next: *mut NumberNode<'list>,
    previous: *mut NumberNode<'list>,
}

impl<'list> NumberNode<'list> {
    fn new(val: i32) -> NumberNode<'static> {
        NumberNode {
            val,
            processed: false,
            initial_next: null_mut(),
            next: null_mut(),
            previous: null_mut(),
        }
    }

    fn append(&'list mut self, next: *mut NumberNode<'list>) {
        unsafe { (*next).previous = self; }

        self.initial_next = next;
        self.next = next;
    }
}

#[derive(Debug, Copy, Clone)]
struct LinkedList<'list> {
    head: &'list NumberNode<'list>,
}

// #[derive(Debug, Copy, Clone)]
// struct NumberNode<'list> {
//     val: i32,
//     processed: bool,
//     initial_next: Option<&'list NumberNode<'list>>,
//     next: Option<&'list NumberNode<'list>>,
//     previous: Option<&'list NumberNode<'list>>,
// }
//
// impl<'list> NumberNode<'list> {
//     fn new(val: i32) -> NumberNode<'static> {
//         NumberNode{
//             val,
//             processed: false,
//             initial_next: None,
//             next: None,
//             previous: None,
//         }
//     }
//
//     fn append(&'list mut self, next: &'list mut NumberNode<'list>) {
//         next.previous = Some(self);
//
//         self.initial_next = Some(next);
//         self.next = self.initial_next;
//
//     }
// }
//
// #[derive(Debug, Copy, Clone)]
// struct LinkedList<'list> {
//     head: &'list NumberNode<'list>
// }
