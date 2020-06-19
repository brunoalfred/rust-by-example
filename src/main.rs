

fn give_commoner(gift: Option<&str>){
    //Specifying a course of action for each case of input.
    match gift {
        Some("snake") => println!("You are so courageous!"),
        Some(inner) => println!("{}? How nice", inner),
        None => println!("No gift?, what a shock"),

        _ => {}
    }
}


fn give_princess(gift: Option<&str>){
    let inside = gift.unwrap();
    if inside == "snake" {panic!("Aaaaaaaaaaaaah Mambo gani haya");}
    else { println!("I love {}", inside); }

}

fn main(){

    let food = Some("cabbage");
    let snake = Some("Ugali");
    let nothing = None;
    give_commoner(food);
    give_commoner(snake );

}