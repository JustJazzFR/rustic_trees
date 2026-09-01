//TEST FILE

#include <stdio.h>
#include "cbasictrees.h"
#include <stdlib.h>

int main() {
    //BINARY TREE TEST
    CBinaryNode* tree = (CBinaryNode*)new_node((char*)"File");
    printf("Tree size: %zu\n", size_of(tree));

    insert(&tree, (char*)"Data 1");
    insert(&tree, (char*)"Data 2");
    const char* data = (const char*)find(tree, (char*)"Data 2")->data;
    data != NULL? printf("Data found: %s\n", data) : printf("Data not found\n");
    printf("Tree size: %zu\n", size_of(tree));
    void* deleted = del(&tree, (char*)"Data 2");
    printf("Deleted: %s\n", (char*)deleted);
    printf("Tree size: %zu\n", size_of(tree));
    clear(&tree);
    //Output of this^^:
    // Tree size: 1
    // Data Found: Data 2
    // Tree size: 3
    // Deleted: Data 2
    // Tree size: 2
    //it works! (i hope)


    // N-ARY TREE TEST
    CNaryNode* nary_tree = (CNaryNode*)new_nnode((char*)"Base");
    printf("---N-Ary test ---\n");
    printf("Tree size: %zu\n", nsize_of(nary_tree));
    ninsert(&nary_tree, (char*)"abc");
    ninsert(&nary_tree, (char*)"def");
    printf("Tree size: %zu\n", nsize_of(nary_tree));

    ninsert(&(nary_tree->children[0]), (char*)"def");
    ninsert(&(nary_tree->children[1]), (char*)"hij");
    printf("Tree size: %zu\n", nsize_of(nary_tree));

    CNaryNode* nfound = nfind(nary_tree, (char*)"def");
    if(nfound != NULL) {
        printf("data found: %s\n", (char*)nfound->data);
    }
    CNaryNode* removed = ndel(nary_tree, 0);
    if (removed != NULL) {
        printf("Removed branch starting with: %s\n", (char*)removed->data);
        nclear(&removed);
    }

    printf("N-ary Tree final size: %zu\n", nsize_of(nary_tree)); // Should be 2 (Base and hij)
    nclear(&nary_tree);
    //output:
    // Tree size: 1Tree size: 3
    // Tree size: 5
    // data found: def
    // Removed branch starting with: abc
    // N-ary Tree final size: 3
    // IT WORKS! (i also hope. i can start writing rust now)
    return 0;
}
