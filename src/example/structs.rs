pub mod profile {

    #[derive(Debug)]
struct Student {
    // name: char,
    age: u32,
    sex: char,
}

impl Student {   
    fn new(age: u32, sex: char) -> Student {
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


}

