#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn calc_area(rectangle: &Rectangle) -> f32 {
    let Rectangle {
        top_left: top,
        bottom_right: bottom,
    } = rectangle;
    (top.y - bottom.y) * (bottom.x - top.x)
}

fn main() {
    let name = String::from("Peter");
    let age = 50;
    let person = Person { name, age };
    println!("{:?}", person);

    let point1 = Point { x: 1.1, y: 5.2 };
    let rectangle = Rectangle {
        top_left: point1,
        bottom_right: Point { x: 4.1, y: 2.2 },
    };

    println!("{:#?}", rectangle);

    let _unit = Unit;
    let pair = Pair(1, 2.3);

    println!("{:?}", pair.0);
    let Pair(int, dec) = pair;
    println!("{} {}", int, dec);

    println!("{}", calc_area(&rectangle));
}
