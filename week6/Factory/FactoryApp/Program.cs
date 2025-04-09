using System;

abstract class Animal {
    public abstract string Speak();
}

class Dog : Animal {
    public override string Speak() => "Woof!";
}

class Cat : Animal {
    public override string Speak() => "Meow!";
}

class AnimalFactory {
    public Animal CreateAnimal(string type) {
        return type switch {
            "dog" => new Dog(),
            "cat" => new Cat(),
            _ => null
        };
    }
}

class Program {
    static void Main() {
        AnimalFactory factory = new AnimalFactory();

        Animal dog = factory.CreateAnimal("dog");
        Animal cat = factory.CreateAnimal("cat");

        Console.WriteLine($"Dog says: {dog.Speak()}");
        Console.WriteLine($"Cat says: {cat.Speak()}");
    }
}
