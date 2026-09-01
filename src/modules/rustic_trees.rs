//RUSTIC TREES!
// Made by @JustJazzFR (plz give credit if you use this)

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

//helper fucntions
//helper fncs
fn to_cptr<T: AsRef<CStr>>(data: T) {
    let cstr = CString::new(data.as_ref().to_bytes()).expect("Failed to convert");
    cstr.into_raw() as *mut c_void
}

fn from_cptr(ptr: *mut c_void) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        unsafe {
            let cstr = CString::from_raw(ptr as *mut i8);
            Some(cstr.to_string_lossy().into_owned())
        }
    }
}
//BINARY TREE
pub struct BinaryTree<T>
where
    T: AsRef<CStr> + 'static,
{
    root: *mut CBinaryNode,
    _phantom: std::marker::PhantomData<T>,
}
//impl
impl<T> BinaryTree<T>
where
    T: AsRef<CStr> + 'static,
{
    pub fn new() -> Self {
        BinaryTree {
            root: ptr::null_mut(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, data: T) {
        unsafe {
            let ptr = to_cptr(data);
            cbasictrees_insert(&mut self.root, ptr);
        }
    }

    pub fn delete(&mut self, what: T) -> Option<String> {
        unsafe {
            let cstr = CString::new(what.as_ref().to_bytes()).expect("Failed to convert");
            let pwhat = cstr.as_ptr() as *mut c_void;
            let deleted = cbascitrees_del(&mut self.root, pwhat);
            from_cptr(deleted);
        }
    }

    pub fn find_node(&self, what: T) -> Option<String> {
        unsafe {
            let cstr = CString::new(what.as_ref().to_bytes()).expect("Failed to convert");
            let pwhat = cstr.as_ptr() as *mut c_void;

            let node_ptr = cbascitrees_find(self.root, pwhat);
            if node_ptr.is_null() {
                None
            } else {
                let node_data = (*node_ptr).data;
                CStr::from_ptr(node_data as *const i8)
                    .to_str()
                    .ok()
                    .map(|s| s.to_string())
            }
        }
    }

    pub fn empty(&self) -> bool {
        unsafe { cbascitrees_is_empty(self.root) }
    }

    pub fn size(&self) -> usize {
        unsafe { cbascitrees_size_of(self.root) as usize }
    }
}
//drop
impl<T> Drop for BinaryTree<T>
where
    T: AsRef<CStr> + 'static,
{
    fn drop(&mut self) {
        unsafe {
            clear(&mut self.root);
        }
    }
}
//N-ARY NODE
pub struct NaryTree<T>
where
    T: AsRef<CStr> + 'static,
{
    root: *mut CNaryNode,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NaryTree<T>
where
    T: AsRef<CStr> + 'static,
{
    pub fn new(data: T) -> Self {
        unsafe {
            let ptr = to_cptr(data);
            NaryTree {
                root: cbascitrees_new_nnode(ptr),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn insert(&mut self, parent: *mut CNaryNode, data: T) {
        if parent.is_null() {
            eprintln!("Cannot insert a child into a null parent node");
            return;
        }
        unsafe {
            let ptr = to_cptr(data);
            cbascitrees_ninsert(&mut parent, ptr);
        }
    }

    pub fn delete(&mut self, parent: *mut CNaryNode, indx: u32) -> Option<String> {
        if parent.is_null() {
            eprintln!("Cannot delete a child from a null parent node");
            return None;
        }
        unsafe {
            let deleted = cbasictrees_ndel(parent, indx);
            if deleted.is_null() {
                None
            } else {
                let node_data = (*deleted).data;
                let data_str = CStr::from_ptr(node_data as *const i8)
                    .to_str()
                    .ok()
                    .map(|s| s.to_string());

                let mut tmp_ptr = deleted;
                cbasictrees_nclear(&mut tmp_ptr);

                data_str
            }
        }
    }

    pub fn find_node(&self, what: T) -> Option<*mut CNaryNode> {
        unsafe {
            let cstr = CString::new(what.as_ref().to_bytes()).expect("Failed to convert");
            let pwhat = cstr.as_ptr() as *mut c_void;
            let node_ptr = cbasictrees_nfind(self.root, what_ptr);
            if node_ptr.is_null() {
                None
            } else {
                Some(node_ptr)
            }
        }
    }

    pub fn empty(&self) -> bool {
        unsafe { cbasictrees_nis_empty(self.root) }
    }

    pub fn size(&self) -> usize {
        unsafe { cbasictrees_nsize(self.root) as usize }
    }
}

impl<T> Drop for NaryTree<T>
where
    T: AsRef<CStr> + 'static,
{
    fn drop(&mut self) {
        unsafe {
            cbasictrees_nclear(&mut self.root);
        }
    }
}
