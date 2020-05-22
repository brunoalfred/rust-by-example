// struct UnPrintable(i32);

//using `derive` attribute to make structure printable.
#[derive(Debug)]
struct DebugPrintable(i32);

/*  Putting a `DebugPrintable` inside of structure `Deep` to make it
  *Printable
  */
#[derive(Debug)]
struct Deep(DebugPrintable);

fn main() {

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
    "Slater",
        "Christian",
        actor="actor's"
    );

    println!("Now will {:?} print !", DebugPrintable(3));

}