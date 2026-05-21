#include "ffi_examples.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int32_t c_add(int32_t a, int32_t b) {
    return a + b;
}

char* c_greet(const char* name) {
    char* greeting = (char*)malloc(100);
    snprintf(greeting, 100, "Hello from C, %s!", name);
    return greeting;
}

void c_uppercase_buffer(char* buffer, uint32_t len) {
    for (uint32_t i = 0; i < len; i++) {
        buffer[i] = (char)toupper((unsigned char)buffer[i]);
    }
}

void c_run_callback(C_Callback cb, int32_t value) {
    // Simulate some C logic and then call the Rust callback
    int32_t result = value * 2;
    if (cb) {
        cb(result);
    }
}
