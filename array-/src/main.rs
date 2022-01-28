use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element: {}", slice[0]);
}

fn main() {
    let xs: [i32; 4] = [1, 2, 3, 4];

    let ys: [i32; 500] = [0; 500];

    println!("{}", xs[1]);

    println!("{}", xs.len());

    println!("{}", mem::size_of_val(&xs));

    analyze_slice(&ys);

    analyze_slice(&ys[1..3]);
}
