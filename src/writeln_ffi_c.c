#include <stdio.h>

extern void writeln_ffi_c(char *msg) {
    printf("%s\n", msg);
}
