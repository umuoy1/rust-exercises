use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _vec = &self.0;
        write!(f, "[")?;

        for (count, v) in _vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![2,3,4]);
    println!("{}", v)
}
