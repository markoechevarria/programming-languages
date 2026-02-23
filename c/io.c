#include <stdio.h>

int printSize(int *a) { 
    printf("%d\n", sizeof(*a));
    return sizeof(a);
}

int main() {
    int array[5] = { 1,2,3,4,5};
    int number = 5;

    printf("size in main of number %d\n", sizeof(int));
    printf("The size of array is %d\n", printSize(array));
    printf("The size of number is %d\n", printSize(&number));

    FILE *fp;
    fp = fopen("file.txt", "r");

    int c = fgetc(fp);
    printf("%c\n", c);
    fclose(fp);

    fp = fopen("file.txt", "r");
    while ( (c = fgetc(fp)) != EOF ) {
        printf("%c", c);
    }
    fclose(fp);

    char s[1024];
    int linecount = 0;
    fp = fopen("file.txt", "r");

    while ( fgets(s, sizeof s, fp) != NULL) {
        printf("%d: %s", ++linecount, s);
    }
    fclose(fp);

    fp = fopen("list.txt", "r");
    char name[1024];
    float length;
    int mass;

    while ( fscanf(fp, "%s %f %d", name, &length, &mass) != EOF ) {
        printf("%s whale, %d tonnes, %.2f meters\n", name, mass, length); 
    }
    fclose(fp);

    int x = 32;
    fp = fopen("output.txt", "w");

    fputc('B', fp);
    fputc('\n', fp);
    fprintf(fp, "x = %d\n", x);
    fputs("Hello, world\n", fp);
    fclose(fp);

    unsigned char bytes[6] = {5, 37, 0, 88, 255, 12};
    fp = fopen("output.bin", "wb");

    fwrite(bytes, sizeof(char), 6, fp);
    fclose(fp);

    int byte_count = 10;
    while (byte_count -- ){
        printf("%d", byte_count);
    }

    printf("\n");
    byte_count = 10;
    while (--byte_count ){
        printf("%d", byte_count);
    }
    printf("\n");
}
