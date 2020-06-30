//! adapted from ct-wasm-ports/sha256/reference/sha256.h
//! See there for copyright info

#include <stdint.h>
#include <stddef.h> // size_t

typedef struct {
  uint8_t data[64];
  uint32_t datalen;
  uint64_t bitlen;
  uint32_t state[8];
} SHA256_CTX;

void guest_func_init(SHA256_CTX* ctx);
void guest_func_update(SHA256_CTX* ctx, const uint8_t data[], size_t len);
void guest_func_final(SHA256_CTX* ctx, uint8_t hash[]);
