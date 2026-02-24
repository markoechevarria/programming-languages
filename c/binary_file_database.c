#include <stdio.h>

typedef struct {
    char name[50];
    int age;
    long id;
} Person;

void add(char *name_file, Person *person) {
    FILE *file = fopen(name_file, "ab");
     
    if (file == NULL) { return; }

    fwrite(person, sizeof person, 1, file );
    fclose(file);
}

void search(char *name_file, int index) {
    FILE *file = fopen(name_file, "r");
    if (file == NULL) { return ; }

    fseek()

}

int main() {
    Person persons[3] = {
        {"marko", 20, 1},
        {"juan", 40, 2},
        {"luis", 10, 3},
    };

    char *name_file = "db";

    add(name_file, persons);

    return 0;
}
