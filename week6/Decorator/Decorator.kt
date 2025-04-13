open class Coffee {
    open fun cost(): Int = 5
}

open class CoffeeDecorator(protected val coffee: Coffee) : Coffee() {
    override fun cost(): Int = coffee.cost()
}

class MilkDecorator(coffee: Coffee) : CoffeeDecorator(coffee) {
    override fun cost(): Int = coffee.cost() + 2
}

class SugarDecorator(coffee: Coffee) : CoffeeDecorator(coffee) {
    override fun cost(): Int = coffee.cost() + 1
}

fun main() {
    val coffee = Coffee()
    println("Basic coffee: ${coffee.cost()}")

    val coffeeWithMilk = MilkDecorator(coffee)
    println("Coffee with milk: ${coffeeWithMilk.cost()}")

    val coffeeWithSugar = SugarDecorator(coffee)
    println("Coffee with sugar: ${coffeeWithSugar.cost()}")

    val coffeeWithMilkAndSugar = SugarDecorator(coffeeWithMilk)
    println("Coffee with milk and sugar: ${coffeeWithMilkAndSugar.cost()}")
}
