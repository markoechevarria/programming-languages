#include <stdio.h>

typedef enum ItemType {
    WEAPON,
    POTION,
    ARMOR
} itemType_t ;

typedef struct Item {
    char *name;
    itemType_t item;
    int level;
} item_t;

void printItemDetails(item_t i) {
    itemType_t items;
    
    switch ( i.item ) {
        case WEAPON: printf("Deals [X] damage.\n"); break ;
        case POTION: printf("Heals [X] HP.\n"); break ;
        default: break;
    }
}

int main() {
    itemType_t itemTypes;

    item_t items[] = {
        {"marko", WEAPON, 10},
        {"marko", POTION, 10},
        {"marko", ARMOR, 10},
        {"marko", WEAPON, 10},
        {"marko", POTION, 10}
    };

    for ( int a; a < ( sizeof(items) / sizeof(items[0]) ); a++ ) {
        printItemDetails( items[a] );
    };

    return 0;
}

