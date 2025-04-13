#include <iostream>
#include <string>

class InternalClass {
public:
    std::string fetch() {
        return "get user info";
    }
};

class ExternalClass {
public:
    std::string search() {
        return "get user info";
    }
};

class Adapter {
private:
    ExternalClass* external;
public:
    Adapter(ExternalClass* ext) : external(ext) {}

    std::string fetch() {
        return external->search();
    }
};

int main() {
    InternalClass internal;
    std::cout << "Internal: " << internal.fetch() << std::endl;

    ExternalClass external;
    Adapter adapter(&external);
    std::cout << "External through Adapter: " << adapter.fetch() << std::endl;

    return 0;
}
