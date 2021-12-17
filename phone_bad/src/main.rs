fn get_args() -> Vec<String> {
    use std::env;
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 && args.len() != 2 {
        panic!("Usage: phone_bad <path_to_phone_file> <path_to_good_file> <path_to_book_file> <path_to_bad_file>")
    }

    args
}

fn open_file(pathlit: String) -> Vec<String> {
    use std::{fs, path::Path}; 

    let path = Path::new(&pathlit);
    let file = fs::read_to_string(path).expect("Unable to read file");

    file.lines().map(|x| x.to_string()).collect()
}


fn first_letter_to_upper_case (s1: &String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let args = get_args();

    let phone = open_file(args[1].clone()); 
    let book = open_file(args[3].clone());  
    let good = open_file(args[2].clone()); 
    let bad = open_file(args[4].clone());

    for pitem in &phone {
        for bitem in &book {
            for baitem in &bad {
                for gitem in &good {
                    println!("{} {}, {} {}", first_letter_to_upper_case(pitem).trim(), baitem.to_lowercase().trim(), bitem.to_lowercase().trim(), gitem.to_lowercase().trim());
                }
            }
        }
    }
}

