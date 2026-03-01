#include <stdio.h>

int mystrlen(char *p) {
    int len = 0;
    while ( p[len] != '\0' ) { len++; }
    return len;
}

void *mystrcpy(char t[], char s[]) {
    int i = 0;

    while ( s[i] != '\0' ) {
        t[i] = s[i];
        i++;
    }

    t[i] = '\0';
}

int main() {
    int i = 10;

    printf("%d\n", i);
    printf("%p\n", &i);
    printf("%p\n", &i);

    char *texto = "Hello world";
    printf("%d\n", mystrlen(texto));

    char texto1[] = "hello world";
    char texto2[] = "bye moon";

    mystrcpy(texto1, texto2);
    printf("%s\n", texto1);
}
