use std::mem;

fn main() {


    let t  = (2, 'd', false);
    println!("{:?} {}", t, mem::size_of_val(&t));

    let s = "Bruno has arrived".to_string();
    println!("{}", s);




}


