#ifndef PROXIE_WRAPPER_H
#define PROXIE_WRAPPER_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

size_t str_length(const char* input);

char* str_reverse(const char* input);

size_t count_vowels(const char* input);

char* to_uppercase(const char* input);

void free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif