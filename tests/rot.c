#include <stdio.h>

int main() {
    unsigned char buf[] =
    "\x0b\x1c\x2d\x3e\x4f\x60\x71\x82\x93\xa4\xb5\xc6\xd7\xe8"
    "\xf9\x0a";

    for (int i = 0; i < sizeof buf; i++) {
        buf[i] = (buf[i] - 11) & 0xff;
    }

    for (int i = 0; i < (sizeof buf - 1); i++) {
        printf("\\x%02x", buf[i]);
    }
    printf("\n");

    return 0;
}