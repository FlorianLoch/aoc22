use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve(lines: &mut Lines<BufReader<File>>, _: bool) {
    let mut sum = 0;

    for line in lines {
        let chars: Vec<char> = line.expect("Failed to read line").chars().collect();

        let opponent = chars[0];
        let you = chars[2];

        sum += play_round(opponent, you);
    }

    println!("\tSum: {}", sum)
}


fn play_round(opponent: char, you: char) -> i32 {
    // let mut encryption = HashMap::<char, char>::new();
    let mut value = HashMap::<char, i32>::new();

    // A X: Rock
    // B Y: Paper
    // C Z: Scissor

    // encryption.insert('X', 'A');
    // encryption.insert('Y', 'B');
    // encryption.insert('Z', 'C');

    value.insert('A', 1);
    value.insert('B', 2);
    value.insert('C', 3);

    // let decrypted = *encryption.get(&you).unwrap();
    let decrypted = decrypt(opponent, you);
    let value_of_decrypted = *value.get(&decrypted).unwrap();

    if decrypted == opponent {
        return 3 + value_of_decrypted;
    }

    if decrypted == 'A' {
        if opponent == 'B' {
            return 0 + value_of_decrypted;
        }

        if opponent == 'C' {
            return 6 + value_of_decrypted;
        }
    }

    if decrypted == 'B' {
        if opponent == 'A' {
            return 6 + value_of_decrypted;
        }

        if opponent == 'C' {
            return 0 + value_of_decrypted;
        }
    }

    if decrypted == 'C' {
        if opponent == 'A' {
            return 0 + value_of_decrypted;
        }

        if opponent == 'B' {
            return 6 + value_of_decrypted;
        }
    }

    // Should never happen
    return -1;
}

fn decrypt(opponent: char, you: char) -> char {
    // X loose
    // Y draw
    // Z win

    if you == 'X' {
        if opponent == 'A' {
            return 'C';
        }

        if opponent == 'B' {
            return 'A';
        }

        if opponent == 'C' {
            return 'B';
        }
    }

    if you == 'Y' {
        return opponent;
    }

    // you == 'Z'
    if opponent == 'A' {
        return 'B';
    }

    if opponent == 'B' {
        return 'C';
    }

    // opponent == 'C' {
    return 'A';
}