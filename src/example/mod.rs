pub mod vectors{
    



fn re(v: Vec<i32>) -> Vec<i32>{

    println!("{}", v[10] + v[11]);
    v
    
}



fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[11]);
}

 



// fn main(){
    
//     let mut v = Vec::new();
//     for i in 1..20 {
//         v.push(i);
//     }
//     v = re(v);

//     borrow1(&v);



// }





}