struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name)
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
        }
    }
    drop(_a);
    println!("a was dropped")
}
