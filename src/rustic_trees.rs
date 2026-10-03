//RUSTIC TREES! (Its real guys)
// Made by @JustJazzFR (plz give credit if you use this)
// Wrote a test in main so you can see how to use it. ill also have usages and documentation in the readme

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::os::raw::{c_int, c_void};

//helper fucntions/structs
//This one down here converts rust string data to a c "void*" (a pointer to a raw untyped memory address)
// Done to convert T to void*
fn to_cptr<T>(data: T) -> *mut c_void {
    Box::into_raw(Box::new(data)) as *mut c_void
}
//Helper function to convert c void* back to a rust T
// (i lwky wrote the tree logic in c cause i tried in rust before and didnt really like the experience)
unsafe fn from_cptr<T>(ptr: *mut c_void) -> Option<T> {
    if ptr.is_null() {
        None
    } else {
        Some(*Box::from_raw(ptr as *mut T))
    }
}
//Helper function for comparing (for the binary node, which is a binary search node).
// PLEASE make sure tht if ur using custom structs that they implement Ord or PartialOrd in some way.
unsafe extern "C" fn default_compare<T: PartialOrd>(a: *const c_void, b: *const c_void) -> c_int {
    if a.is_null() || b.is_null() {
        return 0;
    }
    let a = &*(a as *const T);
    let b = &*(b as *const T);
    match a.partial_cmp(b) {
        Some(Ordering::Less) => -1,
        Some(Ordering::Equal) => 0,
        Some(Ordering::Greater) => 1,
        None => 0,
    }
}
//Binary Node debug
fn bnode_dbg<T: Debug>(
    node: *const c::CBinaryNode,
    prefix: &str,
    is_left: bool,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if node.is_null() {
        return Ok(());
    }

    unsafe {
        let data = (*node).data as *const T;
        if !data.is_null() {
            let connector = if is_left { "├── " } else { "└── " };
            writeln!(f, "{}{}{:?}", prefix, connector, *data)?;
            let l = (*node).left;
            let r = (*node).right;

            let mut npref = prefix.to_string();
            npref.push_str(if is_left { "│   " } else { "    " });
            if !l.is_null() || !r.is_null() {
                bnode_dbg::<T>(l, &npref, true, f)?;
                bnode_dbg::<T>(r, &npref, false, f)?;
            }
        }
    }
    Ok(())
}
//nary node debug
fn nnode_dbg<T: Debug>(
    node: *const c::CNaryNode,
    prefix: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if node.is_null() {
        return Ok(());
    }

    unsafe {
        let count = (*node).count;
        for i in 0..count {
            let cptr = *(*node).children.offset(i as isize);
            if !cptr.is_null() {
                let cdata = (*cptr).data as *const T;
                if !cdata.is_null() {
                    let last = i == count - 1;
                    let connector = if last { "└── " } else { "├── " };
                    writeln!(f, "{}{}{:?}", prefix, connector, *cdata)?;

                    let mut npref = prefix.to_string();
                    npref.push_str(if last { "    " } else { "│   " });
                    nnode_dbg::<T>(cptr, &npref, f)?;
                }
            }
        }
    }
    Ok(())
}
// --[[[TREE ITERATORS]]]--
//Enum for traversal order
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TraversalOrder {
    InOrder,
    PreOrder,
    PostOrder,
}
//Binary Tree Iterator
pub struct BinaryTreeIterator<T> {
    stack: Vec<*mut c::CBinaryNode>,
    ord: TraversalOrder,
    pstack: Vec<*mut c::CBinaryNode>, //post stack
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BinaryTreeIterator<T> {
    unsafe fn new(root: *mut c::CBinaryNode, order: TraversalOrder) -> Self {
        let mut iter = BinaryTreeIterator {
            stack: Vec::new(),
            ord: order,
            pstack: Vec::new(),
            _phantom: std::marker::PhantomData,
        };

        if !root.is_null() {
            match order {
                TraversalOrder::PreOrder | TraversalOrder::InOrder => {
                    iter.push_left(root);
                }
                TraversalOrder::PostOrder => {
                    let mut traversal_stack = vec![root];
                    while let Some(node) = traversal_stack.pop() {
                        iter.pstack.push(node);
                        let l = (*node).left;
                        let r = (*node).right;
                        if !l.is_null() {
                            traversal_stack.push(l);
                        }
                        if !r.is_null() {
                            traversal_stack.push(r);
                        }
                    }
                }
            }
        }
        iter
    }

    unsafe fn push_left(&mut self, mut node: *mut c::CBinaryNode) {
        while !node.is_null() {
            self.stack.push(node);
            if matches!(self.ord, TraversalOrder::PreOrder) {
                break;
            }
            node = (*node).left;
        }
    }
}

impl<T> Iterator for BinaryTreeIterator<T>
where
    T: Clone + Debug + 'static,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.ord {
                TraversalOrder::PostOrder => {
                    let node = self.pstack.pop()?;
                    let data = (*node).data as *const T;
                    if data.is_null() {
                        return None;
                    }
                    Some((*data).clone())
                }

                TraversalOrder::InOrder => {
                    let node = self.stack.pop()?;
                    let data = (*node).data as *const T;

                    let r = (*node).right;
                    if !r.is_null() {
                        self.push_left(r);
                    }
                    if data.is_null() {
                        return None;
                    }
                    Some((*data).clone())
                }

                TraversalOrder::PreOrder => {
                    let node = self.stack.pop()?;
                    let data = (*node).data as *const T;

                    let r = (*node).right;
                    let l = (*node).left;

                    if !r.is_null() {
                        self.stack.push(r);
                    }
                    if !l.is_null() {
                        self.stack.push(l);
                    }
                    if data.is_null() {
                        return None;
                    }
                    Some((*data).clone())
                }
            }
        }
    }
}
//Mutable Binary tree iterator
pub struct BinaryTreeMutIterator<'a, T> {
    stack: Vec<*mut c::CBinaryNode>,
    ord: TraversalOrder,
    pstack: Vec<*mut c::CBinaryNode>,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> BinaryTreeMutIterator<'a, T> {
    unsafe fn new(root: *mut c::CBinaryNode, order: TraversalOrder) -> Self {
        let mut iter = BinaryTreeMutIterator {
            stack: Vec::new(),
            ord: order,
            pstack: Vec::new(),
            _phantom: std::marker::PhantomData,
        };

        if !root.is_null() {
            match order {
                TraversalOrder::InOrder | TraversalOrder::PreOrder => iter.push_left(root),
                TraversalOrder::PostOrder => {
                    let mut stack = vec![root];
                    while let Some(node) = stack.pop() {
                        iter.pstack.push(node);
                        let l = (*node).left;
                        let r = (*node).right;

                        if !l.is_null() {
                            stack.push(l);
                        }
                        if !r.is_null() {
                            stack.push(r);
                        }
                    }
                }
            }
        }
        iter
    }
    //helper fn
    unsafe fn push_left(&mut self, mut node: *mut c::CBinaryNode) {
        while !node.is_null() {
            self.stack.push(node);
            if matches!(self.ord, TraversalOrder::PreOrder) {
                break;
            }
            node = (*node).left;
        }
    }

    pub fn next(&mut self) -> Option<&'a mut T> {
        unsafe {
            match self.ord {
                TraversalOrder::PostOrder => {
                    let node = self.pstack.pop()?;
                    let data = (*node).data as *mut T;
                    if data.is_null() {
                        return None;
                    }
                    Some(&mut *data)
                }

                TraversalOrder::InOrder => {
                    let node = self.stack.pop()?;
                    let data = (*node).data as *mut T;
                    let r = (*node).right;

                    if !r.is_null() {
                        self.push_left(r);
                    }
                    if data.is_null() {
                        return None;
                    }
                    Some(&mut *data)
                }

                TraversalOrder::PreOrder => {
                    let node = self.stack.pop()?;
                    let data = (*node).data as *mut T;

                    let l = (*node).left;
                    let r = (*node).right;

                    if !l.is_null() {
                        self.stack.push(l);
                    }
                    if !r.is_null() {
                        self.stack.push(r);
                    }
                    if data.is_null() {
                        return None;
                    }
                    Some(&mut *data)
                }
            }
        }
    }
}
//Nary tree iterator
pub struct NaryTreeIterator<T> {
    stack: Vec<*mut c::CNaryNode>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NaryTreeIterator<T>
where
    T: Clone + Debug + 'static,
{
    unsafe fn new(root: *mut c::CNaryNode) -> Self {
        let mut stack = Vec::new();
        if !root.is_null() {
            stack.push(root);
        }
        NaryTreeIterator {
            stack,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for NaryTreeIterator<T>
where
    T: Clone + Debug + 'static,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let node = self.stack.pop()?;
            let data = (*node).data as *const T;

            let count = (*node).count;
            for i in (0..count).rev() {
                let cptr = *(*node).children.offset(i as isize);
                if !cptr.is_null() {
                    self.stack.push(cptr);
                }
            }

            if data.is_null() {
                None
            } else {
                Some((*data).clone())
            }
        }
    }
}
//mutable nary tree iterator
pub struct NaryTreeMutIterator<'a, T> {
    stack: Vec<*mut c::CNaryNode>,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> NaryTreeMutIterator<'a, T> {
    unsafe fn new(root: *mut c::CNaryNode) -> Self {
        let mut stack = Vec::new();
        if !root.is_null() {
            stack.push(root);
        }
        NaryTreeMutIterator {
            stack,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn next(&mut self) -> Option<&'a mut T> {
        unsafe {
            let node = self.stack.pop()?;
            let data = (*node).data as *mut T;

            let count = (*node).count;
            for i in (0..count).rev() {
                let cptr = *(*node).children.offset(i as isize);
                if !cptr.is_null() {
                    self.stack.push(cptr);
                }
            }

            if data.is_null() {
                None
            } else {
                Some(&mut *data)
            }
        }
    }
}

// -----------|
// BINARY TREE|
// -----------|
pub struct BinaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    value: T,
    root: *mut c::CBinaryNode,
    _phantom: std::marker::PhantomData<T>,
}
//impl
impl<T> BinaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    pub fn new(data: T) -> Self {
        let ptr = to_cptr(data.clone());
        BinaryTree {
            value: data,
            root: unsafe { c::new_node(ptr) },
            _phantom: std::marker::PhantomData::<T>,
        }
    }

    pub fn insert(&mut self, data: T) {
        unsafe {
            let ptr = to_cptr(data);
            c::insert(&mut self.root, ptr, Some(default_compare::<T>));
        }
    }

    pub fn delete(&mut self, what: T) -> Option<*mut c::CBinaryNode> {
        //put the value of the node to delete
        unsafe {
            let ptr = to_cptr(what);
            let deleted = c::del(&mut self.root, ptr, Some(default_compare::<T>));
            let _ = from_cptr::<T>(ptr);
            if deleted.is_null() {
                None
            } else {
                Some(deleted)
            }
        }
    }

    pub fn find(&self, what: T) -> Option<*mut c::CBinaryNode> {
        unsafe {
            let ptr = to_cptr(what);
            let node_ptr = c::find(self.root, ptr, Some(default_compare::<T>));
            let _ = from_cptr::<T>(ptr);
            if node_ptr.is_null() {
                None
            } else {
                Some(node_ptr)
            }
        }
    }

    pub fn empty(&self) -> bool {
        unsafe { c::is_empty(self.root) }
    }

    pub fn size(&self) -> usize {
        unsafe { c::size_of(self.root) + 1 as usize } //+1 for the tree itself
    }
    // RUST FUNCTIONS (no c used here unfortunately, )
    pub fn unwrap(&self) -> T {
        self.value.clone()
    }

    pub fn val(&self) -> &T {
        // RETURNS a refrence to the value
        &self.value
    }

    pub fn mut_val(&mut self) -> &mut T {
        // RETURNS a mutable refrence to te value
        &mut self.value
    }

    pub fn traverse(&self, order: TraversalOrder) -> BinaryTreeIterator<T> {
        unsafe { BinaryTreeIterator::new(self.root, order) }
    }

    pub fn traverse_mut(&mut self, order: TraversalOrder) -> BinaryTreeMutIterator<'_, T> {
        unsafe { BinaryTreeMutIterator::new(self.root, order) }
    }
}
//debug for debugging (cup for drinking ahh)
impl<T> Debug for BinaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if self.root.is_null() {
                return write!(f, "(empty tree)");
            }
            let data = (*self.root).data as *const T;
            if !data.is_null() {
                writeln!(f, "{:?}", *data)?;
            } else {
                writeln!(f, "(root is null)")?;
            }

            let l = (*self.root).left;
            let r = (*self.root).right;
            if !l.is_null() {
                bnode_dbg::<T>(l, "", true, f)?;
            }
            if !r.is_null() {
                bnode_dbg::<T>(r, "", false, f)?;
            }
        }
        Ok(())
    }
}
//drop for automatic cleanup
impl<T> Drop for BinaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    fn drop(&mut self) {
        unsafe {
            c::clear(&mut self.root);
        }
    }
}
// -----------|
// N-ARY TREE |
// -----------|
pub struct NaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    value: T,
    root: *mut c::CNaryNode,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    pub fn new(data: T) -> Self {
        let dptr = to_cptr(data.clone());
        unsafe {
            NaryTree {
                value: data,
                root: c::new_nnode(dptr),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn insert(&mut self, parent_node: Option<*mut c::CNaryNode>, data: T) {
        let target = parent_node.unwrap_or(self.root);
        if target.is_null() {
            eprintln!("Cannot insert a child into a null parent node");
            return;
        }
        unsafe {
            let ptr = to_cptr(data);
            let mut pmut = target;
            c::ninsert(&mut pmut, ptr);
        }
    }

    pub fn delete(
        &mut self,
        parent_node: Option<*mut c::CNaryNode>,
        what: T,
    ) -> Option<*mut c::CNaryNode> {
        let target = parent_node.unwrap_or(self.root);
        if target.is_null() {
            eprintln!("Cannot delete a child from a null parent node");
            return None;
        }
        unsafe {
            let pnode = &mut *target;
            let mut found: Option<u32> = None;
            let c_what = to_cptr(what.clone());

            for i in 0..pnode.count {
                let childptr = *pnode.children.offset(i as isize);
                if !childptr.is_null() {
                    let cdata = (*childptr).data;

                    if default_compare::<T>(c_what, cdata) == 0 {
                        found = Some(i);
                        break;
                    }
                }
            }

            let _ = from_cptr::<T>(c_what);

            if let Some(indx) = found {
                let mut pmut = target;
                let del = c::ndel(&mut pmut, indx);
                if del.is_null() {
                    None
                } else {
                    Some(del)
                }
            } else {
                None
            }
        }
    }

    pub fn find(&self, what: T) -> Option<*mut c::CNaryNode>
    where
        T: Clone + PartialOrd,
    {
        let c_what = to_cptr(what);
        unsafe {
            let node_ptr = c::nfind(self.root, c_what, Some(default_compare::<T>));
            let _ = from_cptr::<T>(c_what);

            if node_ptr.is_null() {
                None
            } else {
                Some(node_ptr)
            }
        }
    }

    pub fn empty(&self) -> bool {
        unsafe { c::nis_empty(self.root) }
    }

    pub fn size(&self) -> usize {
        unsafe { c::nsize_of(self.root) as usize }
    }
    pub fn unwrap(&self) -> T {
        self.value.clone()
    }

    pub fn val(&self) -> &T {
        // RETURNS a refrence to the value
        &self.value
    }

    pub fn mut_val(&mut self) -> &mut T {
        // RETURNS a mutable refrence to te value
        &mut self.value
    }

    pub fn iter(&self) -> NaryTreeIterator<T> {
        unsafe { NaryTreeIterator::new(self.root) }
    }

    pub fn iter_mut(&mut self) -> NaryTreeMutIterator<'_, T> {
        unsafe { NaryTreeMutIterator::new(self.root) }
    }
}
//Debug (for debugging)
impl<T> Debug for NaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            if self.root.is_null() {
                return write!(f, "(empty tree)");
            }

            let data = (*self.root).data as *const T;
            if !data.is_null() {
                writeln!(f, "{:?}", *data)?;
            } else {
                writeln!(f, "(empty root)")?;
            }

            nnode_dbg::<T>(self.root, "", f)?;
        }
        Ok(())
    }
}
//drop for automatic cleanup
impl<T> Drop for NaryTree<T>
where
    T: Clone + PartialOrd + Debug + 'static,
{
    fn drop(&mut self) {
        unsafe {
            c::nclear(&mut self.root);
        }
    }
}

// This kinda took a while to write :\
// ill prob also release the cbasictrees files separately as their own c library (do not forget to do this)
// Do literally anything to this code, just give credit pls :)
