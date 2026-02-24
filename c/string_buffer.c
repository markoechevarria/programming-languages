#include <stdio.h>
#include <stdlib.h>

char *readline(FILE *line) {

    int size = 4;
    int offset = 0;
    char *buffer = malloc(size * sizeof(char));
    int c;

    if ( buffer == NULL ) { return NULL; }

    while ( c = fgetc(line) , c != '\n' && c != EOF ) {

        if ( offset == size - 1) {
            size = size * 2;
            char *tmp = realloc(buffer, size);

            if ( tmp == NULL ) { 
                free(buffer);
                return NULL; 
            }

            buffer = tmp;
        }

        buffer[offset++] = c;

    }

    if ( c == EOF && offset == 0) {
        free(buffer);
        return NULL;
    }

    if ( offset < size - 1 ) {
        char *tmp = realloc(buffer, offset + 1);

        if ( tmp != NULL ) {
            buffer = tmp;
        }
    }

    buffer[offset] = '\0';

    return buffer;

}

int main(){
    FILE *fp = fopen("file.txt", "r");
    char *line;
 
    while ( ( line = readline(fp) ) != NULL ) {
        printf("%s\n", line);
        free(line);
    }

    return 0;
}
