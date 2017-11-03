use std::fmt;

fn main() {
    let mut num_vec = Vec::with_capacity(100);
    let mut x = 1;
    while x <= 100 {
        //println!("{:?}", x);
        num_vec.push(x);
        x = x + 1;
    }
    let mut i = 0;
    while i < num_vec.len() {
        let current_num = num_vec[i];
        if current_num % 3 != 0 && current_num % 5 != 0 {
            println!("{:?}", current_num);
        }
        if current_num % 3 == 0 && current_num % 5 == 0 {
            println!("fozzbozz");
        }
        if current_num % 5 == 0 && current_num % 3 != 0 {
            println!("bozz");
        }
        if current_num % 3 == 0 && current_num % 5 != 0 {
            println!("fozz");
        }
        i = i + 1;
    }
}
