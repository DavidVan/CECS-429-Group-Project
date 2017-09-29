use std::io::{stdin, stdout, Write};

pub fn read_input() -> String {

    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let mut string = s.split_whitespace();
    let first = string.nth(0).expect("Not a valid input");
    first.to_string()
}

pub fn read_input_line() -> String {
    
    let mut input=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    return input;
}
