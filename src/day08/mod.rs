use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, _: bool) {
    let mut area = Vec::<Vec<u32>>::new();

    for line in lines {
        let l = line.expect("Failed to read line");

        let mut row = Vec::<u32>::new();

        for c in l.chars() {
            row.push(c.to_digit(10).expect("char is not a digit"));
        }

        area.push(row)
    }

    let grid_size = area.len();
    let mut visible_trees = 0;
    let mut max_scenic_score = 0;

    // The trees at the edge will always have a score of 0, so we do not even have to consider them
    // in part 2, therefore we can share one loop
    for row_idx in 1..grid_size - 1 {
        for col_idx in 1..grid_size - 1 {
            if visible_in_row(&area, row_idx, col_idx) || visible_in_column(&area, col_idx, row_idx) {
                visible_trees += 1;
            }

            let cur_score = score(&area, row_idx, col_idx);

            max_scenic_score = max(cur_score, max_scenic_score);
        }
    }

    println!("\tVisible trees inside the grid: {}", visible_trees);
    println!("\tVisible trees in total: {}", visible_trees + grid_size * 4 - 4);
    println!("\tHighest scenic score possible: {}", max_scenic_score);
}

fn score(area: &Vec<Vec<u32>>, row_idx: usize, column_idx: usize) -> u32 {
    let tree_height = area[row_idx][column_idx];

    let up = vec_up(area, row_idx, column_idx);
    let down = vec_down(area, row_idx, column_idx);
    let left = vec_left(area, row_idx, column_idx);
    let right = vec_right(area, row_idx, column_idx);

    return viewing_distance(&up, tree_height) *
        viewing_distance(&down, tree_height) *
        viewing_distance(&left, tree_height) *
        viewing_distance(&right, tree_height);
}

fn vec_up(area: &Vec<Vec<u32>>, row_idx: usize, column_idx: usize) -> Vec<u32> {
    let mut vec = Vec::<u32>::new();

    if row_idx == 0 {
        return vec;
    }

    for i in 0..row_idx {
        vec.insert(0, area[i][column_idx]);
    }

    return vec;
}

fn vec_down(area: &Vec<Vec<u32>>, row_idx: usize, column_idx: usize) -> Vec<u32> {
    let mut vec = Vec::<u32>::new();

    if row_idx + 1 == area.len() {
        return vec;
    }

    for i in row_idx + 1..area.len() {
        vec.push(area[i][column_idx]);
    }

    return vec;
}

fn vec_left(area: &Vec<Vec<u32>>, row_idx: usize, column_idx: usize) -> Vec<u32> {
    let mut vec = Vec::<u32>::new();

    if column_idx == 0 {
        return vec;
    }

    for i in 0..column_idx {
        vec.insert(0, area[row_idx][i]);
    }

    return vec;
}

fn vec_right(area: &Vec<Vec<u32>>, row_idx: usize, column_idx: usize) -> Vec<u32> {
    let mut vec = Vec::<u32>::new();

    if column_idx + 1 == area.len() {
        return vec;
    }

    for i in column_idx + 1..area.len() {
        vec.push(area[row_idx][i]);
    }

    return vec;
}

fn viewing_distance(vec: &Vec<u32>, tree_height: u32) -> u32 {
    for (i, other_tree_height) in vec.iter().enumerate() {
        if other_tree_height >= &tree_height {
            return (i + 1) as u32;
        }
    }

    return vec.len() as u32;
}

fn visible_in_row(area: &Vec<Vec<u32>>, row_idx: usize, tree_index: usize) -> bool {
    let row = area.get(row_idx).expect("row not in area");

    return visible_in_vec(&row, tree_index);
}

fn visible_in_column(area: &Vec<Vec<u32>>, column_idx: usize, tree_index: usize) -> bool {
    let mut column = Vec::<u32>::new();

    for rows in area {
        column.push(rows.get(column_idx).expect("row does not contain given column").to_owned())
    }

    return visible_in_vec(&column, tree_index);
}

fn visible_in_vec(vec: &Vec<u32>, tree_idx: usize) -> bool {
    let tree_size = vec[tree_idx];

    let mut visible = true;

    for (i, tree) in vec.iter().enumerate() {
        if i == tree_idx {
            break;
        }

        if tree >= &tree_size {
            visible = false
        }
    }

    if visible {
        return true;
    }

    let mut row = vec.clone();
    row.reverse();

    let mut visible = true;
    let reversed_tree_idx = row.len() - 1 - tree_idx;

    for (i, tree) in row.iter().enumerate() {
        if i == reversed_tree_idx {
            break;
        }

        if tree >= &tree_size {
            visible = false
        }
    }

    return visible;
}
