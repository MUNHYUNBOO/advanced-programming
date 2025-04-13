trait Animal {
    fn speak(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn speak(&self) -> &str {
        "Meow!"
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn create_animal(animal_type: &str) -> Option<Box<dyn Animal>> {
        match animal_type {
            "dog" => Some(Box::new(Dog)),
            "cat" => Some(Box::new(Cat)),
            _ => None,
        }
    }
}

fn main() {
    let dog = AnimalFactory::create_animal("dog").unwrap();
    let cat = AnimalFactory::create_animal("cat").unwrap();

    println!("Dog says: {}", dog.speak());
    println!("Cat says: {}", cat.speak());
}
