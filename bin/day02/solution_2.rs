use std::{fs, vec};

fn main() {
    let file_path = String::from("input.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut incrementer = 0;

    let contents_iterator = contents.lines().map(|curr_string| curr_string.split(" "));
    for curr_line in contents_iterator {
        let mut curr_list = Vec::<i32>::new();
        for curr_word in curr_line {
            curr_list.push(curr_word.parse::<i32>().unwrap());
        }

        let mut list_of_lists = Vec::<Vec<i32>>::new();
        list_of_lists.push(curr_list.clone());

        let curr_list_len = curr_list.len();

        for kick_index in 0..curr_list_len {
            let mut copy_list = curr_list.clone();
            copy_list.remove(kick_index);
            list_of_lists.push(copy_list);
        }

        for list_to_check in list_of_lists {
            if check(&list_to_check) {
                incrementer += 1;
                break;
            }
        }
    }
    println!("incrementer {}", incrementer);
}

fn check(list: &Vec<i32>) -> bool {
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
