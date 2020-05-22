use std::fmt;


struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.0 )
    }
}


//for fmt::Debug
#[derive(Debug)]
struct MinMax(i64, i64);

//for fmt::Display
impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

//for fmt::Debug
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

//for fmt::Display
impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main(){

    println!("The age is {}", Structure(2));

    let minmax = MinMax(0, 14);
    println!("Compare the structure:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax );


    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big}, the small range is {small}",
    big = big_range,
    small = small_range
    );


    let point = Point2D{x: 2.5, y: 4.3};
    println!("Compare the points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

}