use std::{env, process};


fn main() {
    let  args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("error parsing arguments");
        process::exit(1)
    }

    let pattern = args[1].clone();
    let str = args[2].clone();
    
    let result = regex_checker(&pattern, &str);
    println!("{}", result)
}

fn is_valid_pattern(pattern: &str) -> bool {
    // collect chars
    let chars: Vec<char> = pattern.chars().collect();
    if chars.is_empty() {
        return true
    }
    for i in 0..chars.len() {
        if chars[i] == '+' || chars[i] == '*' {
            if i == 0 || chars[i - 1] == '+' || chars[i - 1] == '*' {
                return false
            }
        } 
    }
    return true;
}

fn regex_checker(pattern: &String, str: &String) -> bool {
    if pattern.len() != str.len() {
        return false;
    }
    if !is_valid_pattern(pattern) {
        return false
    }
    for (p_char, str_char) in pattern.chars().zip(str.chars()) {
        if p_char != str_char && p_char != '.' {
            return false;
        }
    }
    return true;
}
