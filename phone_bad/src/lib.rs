pub fn get_args() -> Vec<String> {
    use std::env;
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        panic!("Usage: phone_bad <path_to_phone_file> <path_to_good_file> <path_to_book_file> <path_to_bad_file>")
    }

    args
}

pub fn open_file(pathlit: String) -> Vec<String> {
    use std::{fs, path::Path}; 

    let path = Path::new(&pathlit);
    let file = fs::read_to_string(path).expect("Unable to read file");

    file.lines().map(|x| x.to_string()).collect()
}


pub fn first_letter_to_upper_case(s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn get_index(i: usize, len: usize) -> usize {
    let mut iteration = i.clone();

    while len < iteration {
       iteration = iteration - len;
    }

    iteration
}

pub fn process_vec(vec: Vec<String>, order: i32) -> Vec<String> {
    match order {
        1 => vec.into_iter().map(|x| first_letter_to_upper_case(x).trim().to_string()).collect(), 
        _ => vec.into_iter().map(|x| x.to_lowercase().trim().to_string()).collect(),
    }
}

pub fn get_vector_rank(v1: &Vec<String>, v2: &Vec<String>, v3: &Vec<String>, v4: &Vec<String>, rank: i32) -> Vec<String> {
    let mut lengths = vec![v1.len(), v2.len(), v3.len(), v4.len()];
    lengths.sort();
    lengths.reverse();
    
    fn get_rank(vector: &Vec<String>, lengths: &Vec<usize>) -> i32 {
        match vector.len() {
            _ if vector.len() == lengths[0] => 1,
            _ if vector.len() == lengths[1] => 2,
            _ if vector.len() == lengths[2] => 3,
            _ if vector.len() == lengths[3] => 4,
            _ => panic!(),
        }
    }
    
    if get_rank(v1, &lengths) == rank {
        v1.clone()
    } else if get_rank(v2, &lengths) == rank {
        v2.clone()
    } else if get_rank(v3, &lengths) == rank {
        v3.clone()
    } else if get_rank(v4, &lengths) == rank {
        v4.clone()
    } else {
        panic!()
    }
}
