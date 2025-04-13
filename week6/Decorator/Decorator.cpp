#include <iostream>
using namespace std;

class Coffee {
public:
    virtual int cost() const {
        return 5;
    }
    virtual ~Coffee() = default;
};

class CoffeeDecorator : public Coffee {
protected:
    Coffee* coffee;
public:
    CoffeeDecorator(Coffee* c) : coffee(c) {}
    int cost() const override {
        return coffee->cost();
    }
};

class MilkDecorator : public CoffeeDecorator {
public:
    MilkDecorator(Coffee* c) : CoffeeDecorator(c) {}
    int cost() const override {
        return coffee->cost() + 2;
    }
};

class SugarDecorator : public CoffeeDecorator {
public:
    SugarDecorator(Coffee* c) : CoffeeDecorator(c) {}
    int cost() const override {
        return coffee->cost() + 1;
    }
};

int main() {
    Coffee* coffee = new Coffee();
    cout << "Basic coffee: " << coffee->cost() << endl;

    Coffee* coffeeWithMilk = new MilkDecorator(coffee);
    cout << "Coffee with milk: " << coffeeWithMilk->cost() << endl;

    Coffee* coffeeWithSugar = new SugarDecorator(coffee);
    cout << "Coffee with sugar: " << coffeeWithSugar->cost() << endl;

    Coffee* coffeeWithMilkAndSugar = new SugarDecorator(coffeeWithMilk);
    cout << "Coffee with milk and sugar: " << coffeeWithMilkAndSugar->cost() << endl;

    // 메모리 해제
    delete coffeeWithMilkAndSugar;
    delete coffeeWithSugar;
    delete coffeeWithMilk;
    delete coffee;

    return 0;
}
