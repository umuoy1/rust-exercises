use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        Number {
            value: self.value + rhs.value,
        }
    }
}

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar)");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo)");
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    let number1 = Number { value: 1 };
    let number2 = Number { value: 2 };
    println!("{:?}", number1 + number2)
}
