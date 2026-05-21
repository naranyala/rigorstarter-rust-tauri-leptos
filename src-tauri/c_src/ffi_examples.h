#ifndef FFI_EXAMPLES_H
#define FFI_EXAMPLES_H

#include <stdint.h>

// Basic math
int32_t c_add(int32_t a, int32_t b);

// String manipulation: returns a new string (caller must free)
char* c_greet(const char* name);

// Buffer manipulation: modifies a buffer in place
void c_uppercase_buffer(char* buffer, uint32_t len);

// Callback example: C calls a function pointer provided by Rust
typedef void (*C_Callback)(int32_t);
void c_run_callback(C_Callback cb, int32_t value);

#endif
