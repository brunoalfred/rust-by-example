mod example;
mod control;


fn main(){

    let result = control::sum(2, 3);
    println!("{}", result);

    let std_one = example::structs::new(20, 'M');


    let c = example::traits::new(3.4);
    println!("{}", c.area());

}