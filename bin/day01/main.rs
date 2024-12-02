use std::env;
use std::fs;

fn main() {
    // --snip--
    let file_path = String::from("bin/day01/input.txt");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_iterator = contents.lines().map(|curr_string| curr_string.split("   "));

    let mut list_left = Vec::<i32>::new();
    let mut list_right = Vec::<i32>::new();

    for mut curr_line in contents_iterator {
        let left_word = curr_line.next().unwrap().parse::<i32>().unwrap();
        let right_word = curr_line.next().unwrap().parse::<i32>().unwrap();
        list_left.push(left_word);
        list_right.push(right_word);
    }

    list_left.sort();
    list_right.sort();

    assert_eq!(list_left.len(), list_right.len());

    let mut dist_tot: i32 = 0;

    for i in 0..list_left.len() {
        dist_tot += (list_left[i] - list_right[i]).abs();
    }

    println!("{}", dist_tot);
}
