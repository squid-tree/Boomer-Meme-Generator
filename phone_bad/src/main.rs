mod lib; 
use lib::*;

fn main() {
    let args = get_args();

    let phone = open_file(args[1].clone()); 
    let book = open_file(args[3].clone());  
    let good = open_file(args[4].clone()); 
    let bad = open_file(args[2].clone());

    let first = get_vector_rank(&phone, &bad, &book, &good, 1);
    let second = get_vector_rank(&phone, &bad, &book, &good, 2);
    let third = get_vector_rank(&phone, &bad, &book, &good, 3);
    let fourth = get_vector_rank(&phone, &bad, &book, &good, 4);

    println!("{}, {}, {}, {}", first.len(), second.len(), third.len(), fourth.len());
    for i in 0..phone.len() {
        for i2 in 0..bad.len() {
            for i3 in 0..book.len() {
                for i4 in 0..good.len() {
                    println!("{} {}, {} {}", first_letter_to_upper_case(&phone[get_index(i, phone.clone().len())]).trim(), &bad[get_index(i2, bad.clone().len())].to_lowercase().trim(), &book[get_index(i3, book.clone().len())].to_lowercase().trim(), &good[get_index(i4, good.clone().len())].to_lowercase().trim());
                }
            }
        }
    }
}
