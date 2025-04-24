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

fn regex_checker(pattern: &String, str: &String) -> bool {
    if pattern.len() != str.len() {
        return false;
    }
    return false;
}
