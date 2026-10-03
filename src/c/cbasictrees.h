//simple header file
// Written by @JustJazzFR
#ifndef CBINARYTREES_H
#define CBINARYTREES_H

#include <stdbool.h>
#include <stddef.h>
//Comparator function to compare values for the Binary Tree nd nfind in nary tree.
typedef int (*CompareFnc)(const void*, const void*);
//Binary node
typedef struct CBinaryNode {
    void* data;
    struct CBinaryNode* left;
    struct CBinaryNode* right;
} CBinaryNode;

CBinaryNode* new_node(void* data); //new node
void insert(CBinaryNode** root, void* data, CompareFnc cmp); //inserts a node into the tree
CBinaryNode* del(CBinaryNode** root, void* what, CompareFnc cmp); //deletes a node from the tree nd frees it
void clear(CBinaryNode** root); //deletes EVERY node from the root and frees their memory
CBinaryNode* find(CBinaryNode* root, void* what, CompareFnc cmp); // finds a node with the given data
bool is_empty(CBinaryNode* root); //returns true if the tree is empty
size_t size_of(CBinaryNode* root); //returns the number of nodes in the tree

//N-ary node
typedef struct CNaryNode {
    void* data;
    unsigned int count;
    struct CNaryNode** children;
} CNaryNode;

CNaryNode* new_nnode(void* data); //'n' stands for n-ary btw
void ninsert(CNaryNode** root, void* data);
CNaryNode* ndel(CNaryNode** root, unsigned int index);
void nclear(CNaryNode** root);
CNaryNode* nfind(CNaryNode* root, void* what, CompareFnc cmp);
bool nis_empty(CNaryNode* root);
size_t nsize_of(CNaryNode* root);

#endif
