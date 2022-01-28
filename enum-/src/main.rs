#![allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 123, y: 234 };

    inspect(pressed);
    inspect(load);
    inspect(click);

    let a: i32 = 123;
    let b: i32 = 234;

    println!(
        "a + b = {}",
        VeryVerboseEnumOfThingsToDoWithNumbers::Add.run(a, b)
    );
    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
}
