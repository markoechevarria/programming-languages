#include <stdio.h>

typedef struct Struct1 {
    char x;     // 1 -> 8 
    double y;   // 8 -> 8
    int z;      // 4 -> 8
} struct1;      // total size 8 + 8 + 8 = 24

typedef struct Struct2 {
    double x;   // 8 -> 8
    int y;      // 4 
    char z;     // 1 -> 4 + 1 < 8 => 8
} struct2;      // total size 8 + 8 = 16

int main() {
    printf("The size of struct1 is %d\n", sizeof(struct1));
    printf("The size of struct2 is %d\n", sizeof(struct2));
}
