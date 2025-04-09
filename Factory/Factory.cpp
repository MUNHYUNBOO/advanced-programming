#include <iostream>
#include <string>

using namespace std;

class Animal {
public:
    virtual string speak() = 0;
    virtual ~Animal() {}
};

class Dog : public Animal {
public:
    string speak() override { return "Woof!"; }
};

class Cat : public Animal {
public:
    string speak() override { return "Meow!"; }
};

class AnimalFactory {
public:
    Animal* createAnimal(const string& type) {
        if (type == "dog") return new Dog();
        if (type == "cat") return new Cat();
        return nullptr;
    }
};

int main() {
    AnimalFactory factory;

    Animal* dog = factory.createAnimal("dog");
    Animal* cat = factory.createAnimal("cat");

    cout << "Dog says: " << dog->speak() << endl;
    cout << "Cat says: " << cat->speak() << endl;

    delete dog;
    delete cat;

    return 0;
}
