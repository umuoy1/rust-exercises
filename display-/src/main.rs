use std::fmt;

// part1
struct Structure(i32, i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

//part2
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} + {}i", self.real, self.imag);
    }
}

fn main() {
    println!("{}", Structure(123, 234));

    let min_max = MinMax(0, 14);
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let point = Point2D { x: 3.3, y: 4.4 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
