//TODO: Convert temperature between Fahrenheit and celsius.
//FORMULA:(32°F − 32) × 5/9 = 0°C


use std::io;



fn main(){

    println!("Welcome!, to convert from Celsius to fahrenheit press 1");
    let mut option = String::new();

    io::stdin().read_line(&mut option)
        .expect("Failed to read the input");


    let option: u32 = option.trim().parse()
        .expect("Please Enter a number!");

    println!("{}", option);

    if option == 1  {


        let mut value = String::new();
        io::stdin().read_line(& mut value)
            .expect("Failed to read the Input!");

        let value = value.trim().parse()
            .expect("Please Enter a number!");

        let result = (9/5)*value + 32;

        println!("The temperature in Fahrenheit is: {}", result);

    }

}




