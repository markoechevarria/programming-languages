#include <stdio.h>
#include <stdlib.h>

struct node_t {
    int value;
    struct node_t *node;
};

void list_add(struct node_t *head, int value) {
    struct node_t *new_node = malloc( sizeof( struct node_t) );
    if ( new_node == NULL ) { return; }
    new_node->value = value;

    if ( head == NULL ) {
        head = new_node;
        return ;
    }

    struct node_t *aux = head;
    while ( aux->node != NULL ) {
        aux = aux->node; 
    }

    aux->node = new_node;
}

void list_print(struct node_t *head) {
    struct node_t *aux = head;
    while ( aux != NULL) {
        printf( "%p : %d\n", aux, aux->value );
        aux = aux->node;
    };
}

void list_free( struct node_t *head) {
    struct node_t *parent = head;
    struct node_t *child;
    
    while (parent != NULL) {
        child = parent->node;
        free(parent);
        parent = child;
    }
}

int main() {

    struct node_t *linked_list = malloc(sizeof( struct node_t));
    if ( linked_list == NULL ) { return 1; }

    list_add( linked_list , 1);
    list_add( linked_list , 2);
    list_add( linked_list , 3);
    list_add( linked_list , 4);
    list_print( linked_list );
    list_free(linked_list);

    return 0;
}
