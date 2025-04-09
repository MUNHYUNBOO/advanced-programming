#include <iostream>

class Calculator {
private:
    double result;

public:
    Calculator() : result(0) {}

    Calculator& add(double value) {
        result += value;
        return *this;
    }

    Calculator& subtract(double value) {
        result -= value;
        return *this;
    }

    Calculator& multiply(double value) {
        result *= value;
        return *this;
    }

    Calculator& divide(double value) {
        if (value != 0) {
            result /= value;
        }
        return *this;
    }

    double get_result() const {
        return result;
    }
};

int main() {
    Calculator calc;
    double result = calc.add(5)
                        .multiply(2)
                        .subtract(3)
                        .divide(2)
                        .get_result();

    std::cout << result << std::endl;
    return 0;
}
