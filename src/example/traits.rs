mod structs;

pub trait Shape {
    fn area(&self) -> u32;
}


impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.141 * self.radius * self.radius) as u32
    }
}

impl Rectangle {
   pub fn new(x: u32, y: u32) -> Rectangle {
        Rectangle { x: x, y: y }
    }
}

impl Circle {
  pub  fn new(radius: f64) -> Circle{
        Circle{radius: radius}
    }
}





impl Student {
    pub fn new(age: u32, sex: char) -> Student {
        Student { age: age, sex: sex }
    }
}

impl Student {
    fn intro(&self) {
        println!("My age is: {}", self.age);
        println!("My sex is: {}", self.sex);
    }

    // if  student qualifies to be admited

    fn age_qualification(&self) -> bool {
        let status;
        status = if self.age >= 18 { true } else { false };
        status
    }
}
