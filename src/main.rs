fn main() {
    let array = [2, 3, 4, 5, 7, 7];
    let number = 1..6;


    for value in array.iter() {
        println!("The value is: {}", value);
    }

    for value in number {
        println!("The values are: {}", value);
    }
}