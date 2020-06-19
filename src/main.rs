fn main() {
    'outer: loop {
        println!("Entered the outer loop!");

        'inner: loop {
            println!("Entered the inner loop!");

            break 'outer;
        }

        println!("Is this point ever reached! ");
    }


    println!("Exited the outer loop!");
}