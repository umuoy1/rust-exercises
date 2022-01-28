use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("{:?}", long_tuple);
    let t = (123i32, true);
    println!("{:?}", reverse(t));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{:?}", matrix);
    println!("{}", transpose(&matrix));
    println!("{:?}", transpose(&matrix));
}
