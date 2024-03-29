use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");
    
    let mut total = 0;

    for c in contents.lines() {
        let mut num1 = '0';
        let mut num2 = '0';

        let new_line = c.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        for i in new_line.chars() {
            if i.is_numeric() {
                num1 = i;
                break;
            }

        }

        let reverse = new_line.chars().rev().collect::<String>();

        for j in reverse.chars() {
            if j.is_numeric() {
                num2 = j;
                break;
            }
        }

        let final_num = num1.to_string() + &num2.to_string();
        let final_int = final_num.parse::<i32>().unwrap();
        total += final_int;
    }

    println!("Total: {}", total);
}
