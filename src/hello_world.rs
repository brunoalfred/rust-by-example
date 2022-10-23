
fn main() {

    #[allow(dead_code)]
    #[derive(Display)]
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct Printable(UnPrintable);

    println!("This struct `{:?}` won't print...", Printable(UnPrintable(3)));

}
