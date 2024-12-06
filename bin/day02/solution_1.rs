use std::{fs, vec};

fn main() {
    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut incrementer = 0;

    let contents_iterator = contents.lines().map(|curr_string| curr_string.split(" "));
    for mut curr_line in contents_iterator {
        let mut currList = Vec::<i32>::new();
        for mut curr_word in curr_line {
            currList.push(curr_word.parse::<i32>().unwrap());
        }
        if check(currList) {
            incrementer += 1;
        }
    }
    println!("incrementer {}", incrementer);
}

fn check(list: Vec<i32>) -> bool {
    let direction_positive = !(list[0] > list[1]);
    let mut last_int = list[0];
    for curr_int in list.iter().skip(1) {
        let diff = curr_int - last_int;
        if (diff >= 0) != direction_positive {
            return false;
        }
        if diff.abs() > 3 || diff.abs() == 0 {
            return false;
        }
        last_int = *curr_int;
    }
    return true;
}
