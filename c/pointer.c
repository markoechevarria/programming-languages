#include <stdio.h>

void mymemccpy(void *t1, void *t2, int n) {

    char *first = t1;
    char *second = t2;
    
    while ( n-- ) {
        *first++ = *second++;
    }

}

int main() {
    char *texto1 = "hello world";
    char *texto2;

    mymemccpy(texto1, texto2, 11 * sizeof(char));

    printf("%s\n", texto2);

    return 0;
}
