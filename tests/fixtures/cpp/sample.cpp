#include <iostream>

namespace myns {
    class MyClass {
    public:
        void method() {
            std::cout << "Hi" << std::endl;
        }
    };
}

void top_level_func() {
    return;
}
