#include <stdint.h>
    
__attribute__((export_name("fac")))
int32_t fac(int32_t n) {
    if (n == 0) {
        return 1;
    } else {
        return n * fac(n - 1);
    }
}