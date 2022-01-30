use std::fmt::Display;

fn main() {
    let mut traffic_light = TrafficLight::new();
    let house_light = HouseLight::new();
    print_state(&house_light);

    print_state(&traffic_light);
    traffic_light.turn_color(TrafficLightColor::Yellow);
    print_state(&traffic_light);

    traffic_light.turn_color(TrafficLightColor::Green);
    print_state(&traffic_light);

    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}

#[derive(Debug)]
struct TrafficLight {
    color: TrafficLightColor,
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }

    pub fn turn_color(&mut self, color: TrafficLightColor) {
        self.color = color
    }
}

impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_string = match self {
            TrafficLightColor::Green => "green",
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
        };
        write!(f, "{}", color_string)
    }
}

#[derive(Debug)]
struct HouseLight {
    on: bool,
}

impl Display for HouseLight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HouseLight is {}", if self.on { "on" } else { "off" })
    }
}

impl HouseLight {
    pub fn new() -> Self {
        Self { on: false }
    }
}
/**
 * part 2
 */
struct Sheep {
    naked: bool,
    name: &'static str,
}

// fn print_state(TrafficLightColor | HouseLight) -> bool{}

trait Light {
    fn get_name(&self) -> &str;
    // state 有不同类型，使用 Debug 打印
    fn get_state(&self) -> &dyn std::fmt::Debug;
}

impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House Light"
    }

    // Rust Complier 不知道 dyn std::fmt::Debug的大小，但知道 &dyn std::fmt::Debug 的大小
    // 引用是有大小的
    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.on
    }
}

impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic Light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.color
    }
}

// 接受一个实现了 Trait Light 的参数
fn print_state(light: &impl Light) {
    println!("{} is {:?}", light.get_name(), light.get_state());
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaah?"
        } else {
            "baaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked", self.name());
        } else {
            println!("{} gets a haircut!", self.name());

            self.naked = true;
        }
    }
}
