# Description: 
This is a relatively simple library i made using rust and c to make trees. I decided to write the tree logic in c cause i tried in rust before and didnt really like it, and also i just like c better bcz its simpler.
if you need to contact me so i can fix any problems or add anything, message me on discord here: justjazzfr
all the trees impl debug ({:?}) for easy printing.
usages can be viewed in src/main.rs and src/c/test.c

# RUSTIC TREES DOCUMENTATION:
## BinaryTree<T>
```rust
pub struct BinaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    value: T, //root value
    root: *mut c::CBinaryNode, //root node
    _phantom: std::marker::PhantomData<T>, // marker for T
}
```
### Methods
- `pub fn new(data: T) -> Self` creates a new tree and initializes the root value with the data & puts tht data in the root node also
- `pub fn insert(&mut self, data: T)` inserts data into the node (this is a binary search tree)
- `pub fn delete(&mut self, what: T) -> Option<*mut c::CBinaryNode>` deletes the node with the specified data and returns the deleted node.
- `pub fn find(&self, what: T) -> Option<*mut c::CBinaryNode>` finds the node with the specified data and returns the node.
- `pub fn empty(&self) -> bool` returns true if the node is empty (false if it isnt)
- `pub fn size(&self) -> usize` returns how many nodes in the tree
- `pub fn unwrap(&self) -> T` returns the clone of the inner value of the node
- `pub fn val(&self) -> &T` immutable refrence to the value
- `pub fn mut_val(&mut self) -> &mut T` mutable refrence to the value
- `pub fn traverse(&self, order: TraversalOrder) -> BinaryTreeIterator<T>` returns a BinaryTreeIterator. works the same as .iter() (you can apply .filter(), .find(), etc.)
- `pub fn traverse_mut(&mut self, order: TraversalOrder) -> BinaryTreeMutIterator<'_, T>` returns a BinaryTreeMutIterator. allows you to mutate the values. works differently from regular traverse (chech the main.rs file)

## NaryTree<T>
```rust
pub struct NaryTree<T> {
    value: T,
    root: *mut c::CNaryNode,
   _phantom: std::marker::PhantomData<T>, 
}
```
### Methods
- `pub fn new(data: T) -> Self` creates a new tree and initializes the root value with the data & puts tht data in the root node also
- `pub fn insert(&mut self, parent_node: Option<*mut c::CNaryNode>, data: T)` inserts a value into the specified parent node (None for the root tree node). Check main.rs for how this works.
- `pub fn delete(&mut self, parent_node: Option<*mut c::CNaryNode>, what: T) -> Option<*mut c::CNaryNode>` deletes a node and returns the node deleted.
- `pub fn find(&self, what: T) -> Option<*mut c::CNaryNode>` finds the node with the specified value and returns the node
- `pub fn empty(&self) -> bool` returns true if the node is empty (false if it isnt)
- `pub fn size(&self) -> usize` returns how many nodes are in the tree
- `pub fn unwrap(&self) -> T` returns the clone of the inner value of the tree
- `pub fn val(&self) -> &T` immutable refrence of the value
- `pub fn mut_val(&mut self) -> &mut T` mutable refrence to the value
- `pub fn iter(&self) -> NaryTreeIterator<T>` returns a NaryTreeIterator. Works like regular .iter() (you can apply .filter(), .find(), etc.)
- `pub fn iter_mut(&mut self) -> NaryTreeMutIterator<'_, T>` returns a NaryTreeMutIterator. works differently from .iter() (check main.rs)
## Helper functions/structures
- `fn to_cptr<T>(data: T) -> *mut c_void` converts T to a void* (or *mut c_void)
- `unsafe fn from_cptr<T>(ptr: *mut c_void) -> Option<T>` converts void* to T
- `unsafe extern "C" fn default_compare<T: PartialOrd>(a: *const c_void, b: *const c_void) -> c_int` default comparator function (for the binary tree to determine where to insert values)
- `fn bnode_dbg<T: Debug>(node: *const c::CBinaryNode, prefix: &str, is_left: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result` debug formatting for binary tree
- `fn nnode_dbg<T: Debug>(node: *const c::CNaryNode, prefix: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result` debug formatting for the nary tree
```rust 
pub enum TraversalOrder {
    InOrder,
    PreOrder,
    PostOrder
}
```
traversal order for binary node traverse() and traverse_mut()

```rust
pub struct BinaryTreeIterator<T> {
    stack: Vec<*mut c::CBinaryNode>,
    ord: TraversalOrder,
    pstack: Vec<*mut c::CBinaryNode>, //post stack
    _phantom: std::marker::PhantomData<T>,
}
```
Binary tree iterator for iterating through the tree (contains Iterator so you can use the iterator functions)
### BinaryTreeIterator methods
- `unsafe fn new(root: *mut c::CBinaryNode, order: TraversalOrder) -> Self` creates a new BinaryTreeIterator with the specified root and traversal order
- `unsafe fn push_left(&mut self, mut node: *mut c::CBinaryNode)` pushes a node to the stack and sets it equal to its left
---

```rust
pub struct BinaryTreeMutIterator<'a, T> {
    stack: Vec<*mut c::CBinaryNode>,
    ord: TraversalOrder,
    pstack: Vec<*mut c::CBinaryNode>,
    _phantom: std::marker::PhantomData<&'a mut T>,
}
```
mutable binary tree iterator for iterating through the tree
### BinaryTreeMutIterator methods
- `unsafe fn new(root: *mut c::CBinaryNode, order: TraversalOrder) -> Self` creates a new BinaryTreeIterator with the specified root and traversal order
- `unsafe fn push_left(&mut self, mut node: *mut c::CBinaryNode)` pushes a node to the stack and sets it equal to its left
- `pub fn next(&mut self) -> Option<&'a mut T>` works like Iterator next() (goes to the next value). check main.rs for how it's used
---

```rust
pub struct NaryTreeIterator<T> {
    stack: Vec<*mut c::CNaryNode>,
    _phantom: std::marker::PhantomData<T>,
}
```
nary tree iterator for iterating through the nary tree (contains iterator so you can use the iterator functions)
### NaryTreeIterator functions
- `unsafe fn new(root: *mut c::CNaryNode) -> Self` creates a new NaryTreeIterator with the specified root
---

```rust
pub struct NaryTreeMutIterator<'a, T> {
    stack: Vec<*mut c::CNaryNode>,
    _phantom: std::marker::PhantomData<&'a mut T>,
}
```
mutable nary tree iterator
### NaryTreeMutIterator methods
- `unsafe fn new(root: *mut c::CNaryNode) -> Self` creates a new NaryTreeIterator with the specified root
- `pub fn next(&mut self) -> Option<&'a mut T>` works like Iterator next() (goes to the next value). check main.rs for how its used
---

# C BASIC TREES DOCUMENTATION:
## Comparator function
`typedef int (*CompareFnc)(const void*, const void*);` compares 2 datas to see if they are equal
## CBinaryNode
```c
typedef struct CBinaryNode {
    void* data;
    struct CBinaryNode* left;
    struct CBinaryNode* right;
} CBinaryNode;
```
### Methods
- `CBinaryNode* new_node(void* data);` creates a new node with the specified data
- `void insert(CBinaryNode** root, void* data, CompareFnc cmp);` inserts a new node
- `CBinaryNode* del(CBinaryNode** root, void* what, CompareFnc cmp);` deletes a node with the specified data
- `void clear(CBinaryNode** root);` deletes every node from the root and frees the memory
- `CBinaryNode* find(CBinaryNode* root, void* what, CompareFnc cmp);` finds the specified node and returns the node itself
- `bool is_empty(CBinaryNode* root);` returns true if the root is empty (false if it isnt)
- `size_t size_of(CBinaryNode* root);` returns how many nodes in the root
## CNaryNode
```c
typedef struct CNaryNode {
    void* data;
    unsigned int count;
    struct CNaryNode** children;
} CNaryNode;
```
### Methods
- `CNaryNode* new_nnode(void* data);` creates a new Nary node with the specified data
- `void ninsert(CNaryNode** root, void* data);` inserts a new node with the specified data into the root
- `CNaryNode* ndel(CNaryNode** root, unsigned int index);` deletes a node from the index specified and returns the deleted node
- `void nclear(CNaryNode** root);` deletes every node from the root and frees its memory
- `CNaryNode* nfind(CNaryNode* root, void* what, CompareFnc cmp);` finds a node with the data provided and returns the node
- `bool nis_empty(CNaryNode* root);` returns true if the root is empty (false if it isnt)
- `size_t nsize_of(CNaryNode* root);` returns how many nodes in the root
