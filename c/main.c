#include <stdio.h>

typedef struct CoordinateDef {
    int x;
    int y;
} coordinate;

typedef struct testSize {
    int x;
    int y;
    char z;
    char a;
    char b;
    char c;
    char aea;
} testsize;

struct Coordinate {
    int x;
    int y;
};

coordinate create_new_coordinate(int x, int y, int z) {
    coordinate new_coordinate = {
        .x = x,
        .y = y
    };

    return new_coordinate;
}

int main() {
    coordinate new_coordinate = create_new_coordinate(1,2,3);
    printf(" x -> %d and y -> %d\n", new_coordinate.x, new_coordinate.y);

    printf(" size of testsize => %d \n", sizeof( testsize ));
    return 0;
}
