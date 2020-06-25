use std::mem;

fn main() {


    let t  = (2, 'd', false);
    println!("{:?} {}", t, mem::size_of_val(&t));

    let s = "Bruno has arrived".to_string();
    println!("{}", s);

    let s1 = " ,he is in the class ";

    let fs = s + &s1;
    println!("{}", fs);




}


