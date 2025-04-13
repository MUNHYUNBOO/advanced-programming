trait Coffee {
    fn cost(&self) -> u32;
}

struct BaseCoffee;

impl Coffee for BaseCoffee {
    fn cost(&self) -> u32 {
        5
    }
}

struct MilkDecorator<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> Coffee for MilkDecorator<T> {
    fn cost(&self) -> u32 {
        self.coffee.cost() + 2
    }
}

struct SugarDecorator<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> Coffee for SugarDecorator<T> {
    fn cost(&self) -> u32 {
        self.coffee.cost() + 1
    }
}

fn main() {
    let base = BaseCoffee;
    println!("Basic: {}", base.cost());

    let with_milk = MilkDecorator { coffee: base };
    println!("Milk: {}", with_milk.cost());

    let base2 = BaseCoffee;
    let with_sugar = SugarDecorator { coffee: base2 };
    println!("Sugar: {}", with_sugar.cost());

    let with_both = SugarDecorator {
        coffee: MilkDecorator { coffee: BaseCoffee },
    };
    println!("Milk + Sugar: {}", with_both.cost());
}
