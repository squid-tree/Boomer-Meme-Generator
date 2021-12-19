fn get_args() -> Vec<String> {
    use std::env;
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
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


fn first_letter_to_upper_case(s1: &String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn greatest_vec(v1: &Vec<String>, v2: &Vec<String>, v3: &Vec<String>, v4: &Vec<String>) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    use std::collections::HashMap;
    let hashmap = hashmap::New();

    hashmap.insert(
        "v1";
        v1.len();
    );
    hashmap.insert(
        "v2";
        v1.len();
    );
    hashmap.insert(
        "v3";
        v3.len();
    );
    hashmap.insert(
        "v4";
        v4.len();
    );

}

fn get_index(i: usize, len: usize) -> usize {
    let mut iteration: i32 = i.try_into().unwrap();
    let length: i32 = len.try_into().unwrap();

    while length < iteration {
       iteration = iteration - length;
    }

    let index: i32 = iteration - 1;

    index.try_into().unwrap()
}


fn main() {
    let args = get_args();

    let phone = open_file(args[1].clone()); 
    let book = open_file(args[3].clone());  
    let good = open_file(args[4].clone()); 
    let bad = open_file(args[2].clone());
    let largest = greatest_vec(&phone, &bad, &book, &good);

    for i in 1..largest.len() + 1 {
        println!("{} {}, {} {}", first_letter_to_upper_case(&phone[get_index(i, phone.clone().len())]).trim(), &bad[get_index(i, bad.clone().len())].to_lowercase().trim(), &book[get_index(i, book.clone().len())].to_lowercase().trim(), &good[get_index(i, good.clone().len())].to_lowercase().trim());
    }
}
