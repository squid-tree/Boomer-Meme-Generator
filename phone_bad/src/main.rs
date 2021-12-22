mod lib; 
use lib::*;

fn main() {
    let args = get_args();

    let phone = process_vec(open_file(args[1].clone()), 1); 
    let book = process_vec(open_file(args[3].clone()), 3);  
    let good = process_vec(open_file(args[4].clone()), 4); 
    let bad = process_vec(open_file(args[2].clone()), 2);

    let first = get_vector_rank(&phone, &bad, &book, &good, 1);
    let second = get_vector_rank(&phone, &bad, &book, &good, 2);
    let third = get_vector_rank(&phone, &bad, &book, &good, 3);
    let fourth = get_vector_rank(&phone, &bad, &book, &good, 4);

    for i in 0..phone.len() {
        for i2 in 0..bad.len() {
            for i3 in 0..book.len() {
                for i4 in 0..good.len() {
                    println!("{} {}, {} {}", &phone[get_index(i, phone.len())], &bad[get_index(i2, bad.len())], &book[get_index(i3, book.len())], &good[get_index(i4, good.len())]);
                }
            }
        }
    }
}
