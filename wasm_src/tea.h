#include <stdint.h>

void guest_func_encrypt(uint32_t v[2], const uint32_t k[4]);
void guest_func_decrypt(uint32_t v[2], const uint32_t k[4]);
