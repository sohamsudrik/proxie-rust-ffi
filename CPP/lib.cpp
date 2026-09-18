#include "wrapper.h"

#include <algorithm>
#include <cctype>
#include <cstring>
#include <cstdlib>
#include <string>


size_t str_length(const char* input)
{
    if (input == nullptr)
    {
        return 0;
    }

    return std::strlen(input);
}


char* str_reverse(const char* input)
{
    if (input == nullptr)
    {
        return nullptr;
    }

    std::string result(input);

    std::reverse(result.begin(), result.end());

    char* output =
        static_cast<char*>(std::malloc(result.size() + 1));

    if (output == nullptr)
    {
        return nullptr;
    }

    std::memcpy(
        output,
        result.c_str(),
        result.size() + 1
    );

    return output;
}


size_t count_vowels(const char* input)
{
    if (input == nullptr)
    {
        return 0;
    }

    size_t count = 0;

    while (*input != '\0')
    {
        char ch = static_cast<char>(
            std::tolower(
                static_cast<unsigned char>(*input)
            )
        );

        if (
            ch == 'a' ||
            ch == 'e' ||
            ch == 'i' ||
            ch == 'o' ||
            ch == 'u'
        )
        {
            count++;
        }

        input++;
    }

    return count;
}


char* to_uppercase(const char* input)
{
    if (input == nullptr)
    {
        return nullptr;
    }

    std::string result(input);

    for (char& ch : result)
    {
        ch = static_cast<char>(
            std::toupper(
                static_cast<unsigned char>(ch)
            )
        );
    }

    char* output =
        static_cast<char*>(std::malloc(result.size() + 1));

    if (output == nullptr)
    {
        return nullptr;
    }

    std::memcpy(
        output,
        result.c_str(),
        result.size() + 1
    );

    return output;
}


void free_string(char* ptr)
{
    if (ptr != nullptr)
    {
        std::free(ptr);
    }
}