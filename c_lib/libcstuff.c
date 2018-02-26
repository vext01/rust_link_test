#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

uint64_t
c_func_in_lib() {
    printf("Hi from the C lib\n");
    return 42;
}
