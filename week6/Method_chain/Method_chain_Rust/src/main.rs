struct Calculator {
    result: f64,
}

impl Calculator {
    fn new() -> Self {
        Self { result: 0.0 }
    }

    fn add(&mut self, value: f64) -> &mut Self {
        self.result += value;
        self
    }

    fn subtract(&mut self, value: f64) -> &mut Self {
        self.result -= value;
        self
    }

    fn multiply(&mut self, value: f64) -> &mut Self {
        self.result *= value;
        self
    }

    fn divide(&mut self, value: f64) -> &mut Self {
        if value != 0.0 {
            self.result /= value;
        }
        self
    }

    fn get_result(&self) -> f64 {
        self.result
    }
}

fn main() {
    let mut calc = Calculator::new();
    let result = calc.add(5.0)
                     .multiply(2.0)
                     .subtract(3.0)
                     .divide(2.0)
                     .get_result();

    println!("{}", result);
}
