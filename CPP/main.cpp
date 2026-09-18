#include <iostream>
#include "wrapper.h"

int main() {
    std::cout << "==================================================" << std::endl;
    std::cout << "          Proxie C++ Standalone Demo              " << std::endl;
    std::cout << "==================================================" << std::endl;

    const char* sample = "Hello, C++ World!";
    std::cout << "\nInput string: \"" << sample << "\"" << std::endl;

    std::cout << "String length: " << str_length(sample) << std::endl;
    std::cout << "Vowel count:   " << count_vowels(sample) << std::endl;

    char* reversed = str_reverse(sample);
    if (reversed != nullptr) {
        std::cout << "Reversed:      \"" << reversed << "\"" << std::endl;
        free_string(reversed);
    }

    char* uppercase = to_uppercase(sample);
    if (uppercase != nullptr) {
        std::cout << "Uppercase:     \"" << uppercase << "\"" << std::endl;
        free_string(uppercase);
    }

    std::cout << "\n==================================================" << std::endl;
    std::cout << "C++ functions executed successfully!" << std::endl;
    std::cout << "==================================================" << std::endl;

    return 0;
}

