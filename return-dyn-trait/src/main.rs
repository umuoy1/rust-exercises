struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &str;
}

impl Animal for Sheep {
    fn noise(&self) -> &str {
        "baaah"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &str {
        "moooo!"
    }
}

fn random_animal(random_fac: f64) -> Box<dyn Animal> {
    if random_fac < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let cow = Cow {};
    println!("Cow noise: {}", cow.noise());
    let random_fac = 0.2;
    let animal = random_animal(random_fac);
    println!("{}", animal.noise());
}
