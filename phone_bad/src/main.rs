mod lib; 
use lib::*;

fn main() {
    let args = get_args();

    let phone = process_vec(open_file(args[1].clone()), 1); 
    let book = process_vec(open_file(args[3].clone()), 3);  
    let good = process_vec(open_file(args[4].clone()), 4); 
    let bad = process_vec(open_file(args[2].clone()), 2);

    let phonelen = phone.len();
    let booklen = book.len();
    let goodlen = good.len();
    let badlen = bad.len();

    for i in 0..phonelen {
        for i2 in 0..badlen {
            for i3 in 0..booklen {
                for i4 in 0..goodlen {
                    println!("{} {}, {} {}", phone[get_index(i, phonelen)], bad[get_index(i2, badlen)], book[get_index(i3, booklen)], good[get_index(i4, goodlen)]);
                }
            }
        }
    }
}
