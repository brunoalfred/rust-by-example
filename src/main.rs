
fn get_name(name: Option<&str>){
    match name {
        Some("Bruno") => println!(" The name is "),
        None => panic!("Please give a name"),
        _ => {}
    }
}





fn main(){

    let student_one = Some("Bruno");
    get_name(student_one);

}