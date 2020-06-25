struct Object{
    base: u32,
    height: u32,
}

impl Object{
    fn area(&self) -> u32{
        self.base * self.height
    }

    fn show(&self){
        println!("the area of: {}x{} is = {}", self.base, self.height, self.area())
    }
}

impl  Object{

    fn new(base: u32, height: u32) -> Object{
        Object{
            base: base,
            height: height,
        }
    }
}

fn main() {

    let triangle_one = Object::new(20, 40);
    triangle_one.show();
    


}