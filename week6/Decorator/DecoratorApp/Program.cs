using System;

class Coffee
{
    public virtual int Cost()
    {
        return 5;
    }
}

class CoffeeDecorator : Coffee
{
    protected Coffee coffee;

    public CoffeeDecorator(Coffee c)
    {
        coffee = c;
    }

    public override int Cost()
    {
        return coffee.Cost();
    }
}

class MilkDecorator : CoffeeDecorator
{
    public MilkDecorator(Coffee c) : base(c) { }

    public override int Cost()
    {
        return coffee.Cost() + 2;
    }
}

class SugarDecorator : CoffeeDecorator
{
    public SugarDecorator(Coffee c) : base(c) { }

    public override int Cost()
    {
        return coffee.Cost() + 1;
    }
}

class Program
{
    static void Main()
    {
        Coffee coffee = new Coffee();
        Console.WriteLine("Basic coffee: " + coffee.Cost());

        Coffee coffeeWithMilk = new MilkDecorator(coffee);
        Console.WriteLine("Coffee with milk: " + coffeeWithMilk.Cost());

        Coffee coffeeWithSugar = new SugarDecorator(coffee);
        Console.WriteLine("Coffee with sugar: " + coffeeWithSugar.Cost());

        Coffee coffeeWithMilkAndSugar = new SugarDecorator(coffeeWithMilk);
        Console.WriteLine("Coffee with milk and sugar: " + coffeeWithMilkAndSugar.Cost());
    }
}
