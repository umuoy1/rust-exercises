#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("{:?}", 12); // 12
    println!("{1:?} {0:?} {a:?}", "AA", "BB", a = "CC"); // "BB" "AA" "CC"
    println!("{:?}", Structure(123)); // Structure(123)
    println!("{:#?} ", Deep(Structure(234)));
    /*
        Deep(
            Structure(
                234,
            ),
        )
    */
}
