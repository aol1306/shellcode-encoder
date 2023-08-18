#include <stdio.h>

int main() {
    unsigned char buf[] =
    "\x0b\x1a\x29\x38\x4f\x5e\x6d\x7c\x83\x92\xa1\xb0\xc7\xd6"
    "\xe5\xf4";

    for (int i = 0; i < (sizeof buf - 1); i++) {
        buf[i] = buf[i] ^ 11;
    }

    for (int i = 0; i < (sizeof buf - 1); i++) {
        printf("\\x%02x", buf[i]);
    }
    printf("\n");

    return 0;
}