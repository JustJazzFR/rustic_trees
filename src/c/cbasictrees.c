//this took me just 150,290 seconds of my life :cry:
// Written by @JustJazzFR
// ill probalby release this as its own separate c library.
#include "cbasictrees.h"
#include <stdlib.h>
//---------------------------------|
// binary node fncs                |
// --------------------------------|
CBinaryNode* new_node(void* data) {
    CBinaryNode* node = (CBinaryNode*)malloc(sizeof(CBinaryNode));
    if(node == NULL) return NULL;

    node->data = data;
    node->left = NULL;
    node->right = NULL;

    return node;
}

void insert(CBinaryNode** root, void* data, CompareFnc cmp) {
    if(*root == NULL) {
        *root = new_node(data);
        return;
    }
    if(cmp(data, (*root)->data) < 0) {
        insert(&((*root)->left), data, cmp);
    } else {
        insert(&((*root)->right), data, cmp);
    }
}

CBinaryNode* del(CBinaryNode** root, void* what, CompareFnc cmp) {
    if(*root == NULL) return NULL;
    int comp = cmp(what, (*root)->data);

    if(comp < 0) return del(&((*root)->left), what, cmp);
    if(comp > 0) return del(&((*root)->right), what, cmp);

    CBinaryNode* target = *root;

    if(target->left == NULL && target->right == NULL) {
        *root = NULL;
    } else if(target->left == NULL) {
        *root = target->right;
    } else if(target->right == NULL) {
        *root = target->left;
    } else {
        CBinaryNode* succ = target->right;
        while(succ->left != NULL) succ = succ->left;

        void* tmp = target->data;
        target->data = succ->data;
        succ->data = tmp;

        return del(&(target->right), what, cmp);
    }
    target->left = NULL;
    target->right = NULL;
    return target;
}

void clear(CBinaryNode** root) {
    if(*root == NULL) return;
    clear(&((*root)->right));
    clear(&((*root)->left));
    free(*root);
    *root = NULL;
}

CBinaryNode* find(CBinaryNode* root, void* what, CompareFnc cmp) {
    if(root == NULL) return NULL;
    int comp = cmp(what, root->data);
    if(comp == 0) return root;
    if (comp < 0) return find(root->left, what, cmp);
    return find(root->right, what, cmp);
}

bool is_empty(CBinaryNode* root) {
    return root == NULL? true : false;
}

size_t size_of(CBinaryNode* root) {
    if(root == NULL) return 0;
    return 1 + size_of(root->left) + size_of(root->right);
}
//---------------------------------|
//N-ary node fncs                  |
// --------------------------------|
CNaryNode* new_nnode(void* data) {
    CNaryNode* node = (CNaryNode*)malloc(sizeof(CNaryNode));
    if(node == NULL) return NULL;
    node->data = data;
    node->count = 0;
    node->children = NULL;
    return node;
}

void ninsert(CNaryNode** root, void* data) {
    if (*root == NULL) return;
    (*root)->children = (CNaryNode**)realloc((*root)->children, sizeof(CNaryNode*) * ((*root)->count + 1));
    (*root)->children[(*root)->count] = new_nnode(data);
    (*root)->count++;
}

CNaryNode* ndel(CNaryNode** root, unsigned int index) {
    if (*root == NULL || index >= (*root)->count) return NULL;

    CNaryNode* deleted_node = (*root)->children[index];
    for (unsigned int i = index; i < (*root)->count - 1; i++) {
        (*root)->children[i] = (*root)->children[i + 1];
    }
    (*root)->count--;
    if ((*root)->count > 0) {
        (*root)->children = (CNaryNode**)realloc((*root)->children, sizeof(CNaryNode*) * (*root)->count);
    } else {
        free((*root)->children);
        (*root)->children = NULL;
    }

    return deleted_node;
}

void nclear(CNaryNode** root) {
    if(*root == NULL) return;
    for(unsigned int i = 0; i < (*root)->count; i++) {
        nclear(&((*root)->children[i]));
    }
    (*root)->count = 0;
    free((*root)->children);
    free(*root);
    *root = NULL;
}

CNaryNode* nfind(CNaryNode* root, void* what, CompareFnc cmp) {
    if(root == NULL) return NULL;
    if(cmp(root->data, what) == 0) return root;
    for(unsigned int i = 0; i < root->count; i++) {
        CNaryNode* found = nfind(root->children[i], what, cmp);
        if(found != NULL) return found;
    }
    return NULL ;
}

bool nis_empty(CNaryNode* root) {
    return root == NULL? true : false;
}

size_t nsize_of(CNaryNode* root) {
    size_t size = 1;
    if(root == NULL) return 0;
    for(unsigned int i = 0; i < root->count; i++) {
        size += nsize_of(root->children[i]);
    }
    return size;
}
