use std::collections::{HashSet};
use crate::puzzle;

pub struct Day01 {}

impl puzzle::Puzzle for Day01 {
    fn group_n_lines(&self) -> usize {
        1
    }

    fn solve(&self, lines: &Vec<String>) -> i32 {
        todo!()
    }
}

// The former logic:
// fn main() {
//     let mut _sums = Vec::new();
//     let mut _current = 0;
//
//     if let Ok(lines) = read_lines("./supplylist") {
//         for line in lines {
//             if let Ok(l) = line {
//                 if l == "" {
//                     _sums.push(_current);
//
//                     _current = 0;
//
//                     continue;
//                 }
//
//                 let as_int: i32 = l.parse().unwrap();
//
//                 _current += as_int
//             }
//         }
//     }
//
//     _sums.sort();
//     _sums.reverse();
//
//     println!("Max value {}", _sums.first().unwrap());
//     println!("Top 3 {}", sum_up_first_n_entries(&_sums, 3))
// }
//
// fn sum_up_first_n_entries(_v: &Vec<i32>, n: usize) -> i32 {
//     let mut _sum = 0;
//
//     for i in 0..n {
//         let val = _v.get(i);
//
//         if val == None {
//             break;
//         }
//
//         _sum += val.unwrap();
//     }
//
//     return _sum;
// }
