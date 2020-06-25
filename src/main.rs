use std::mem;


struct Object {
    width: u32,
    lenght: u32,
}

fn area(obj: &Object) -> u32{

    obj.lenght * obj.width

}

fn main() { 

    let o = Object{
        width: 40,
        lenght: 20,
    };

    let mut a = area(&o);

    println!("The product of {}x{} is: {}", o.width, o.lenght, a);



}


