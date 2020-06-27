pub mod structs;
pub mod traits;

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[10] + v[11]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[11]);
}

fn new_vector() -> Vec<i32> {
    Vec::new()
}

// fn main(){
//     let mut v = Vec::new();
//     for i in 1..20 {
//         v.push(i);
//     }
//     v = re(v);

//     borrow1(&v);

// }
