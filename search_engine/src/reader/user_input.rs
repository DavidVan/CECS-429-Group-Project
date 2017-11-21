use std::io::{stdin, stdout, Write};

/*
 * Reads an input from the user (No whitespace)
 *
 * # Returns
 *
 * String representation of User Input
 *
 */
pub fn read_input() -> String {

    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect(
        "Did not enter a correct string",
    );
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let mut string = s.split_whitespace();
    let first = string.nth(0).expect("Not a valid input");
    first.to_string()
}

/*
 * Reads an input from the user 
 *
 * # Returns
 *
 * String representation of User Input
 *
 */
pub fn read_input_line() -> String {

    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect(
        "Did not enter a correct string",
    );
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    return input;
}

pub fn read_number() -> i32 {

    let mut input_text = String::new();
    stdin().read_line(&mut input_text) .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => panic!("Error! Not an integer")
    };
}

pub fn read_number_range(low: i32, high: i32) -> i32 {

    loop {
        let input : i32 = read_number();

        if input >= low && input <= high {
            return input; 
        }
        println!("Input not in range");
    }

}

pub fn read_yes_no() -> bool {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read from stdin");

        let final_input= (input.trim()).to_lowercase();
        
        if final_input == "y" {
            return true; 
        } else if final_input == "n" {
            return false; 
        }
        println!("Invalid input, try again");
    }
}
