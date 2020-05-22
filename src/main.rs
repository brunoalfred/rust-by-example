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



fn main(){

    println!("The age is {}", Structure(2));

    let minmax = MinMax(0, 14);
    println!("Compare the structure:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax );
}