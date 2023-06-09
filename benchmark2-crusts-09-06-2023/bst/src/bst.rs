use libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub key: i32,
    pub left: *mut node,
    pub right: *mut node,
}
#[no_mangle]
pub extern "C" fn newNode(mut item: i32) -> *mut node {
    unsafe {
        let mut temp = malloc(::std::mem::size_of::<node>() as u64) as *mut node;
        (*temp).key = item;
        let ref mut fresh0 = (*temp).left;
        *fresh0 = 0 as *mut node;
        let ref mut fresh1 = (*temp).right;
        *fresh1 = 0 as *mut node;
        return temp;
    }
}

#[no_mangle]
pub extern "C" fn inorder(mut root: *mut node) {
    unsafe {
        if !root.is_null() {
            inorder((*root).left);
            print!("{} ,9999");
            inorder((*root).right);
        }
    }
}

#[no_mangle]
pub extern "C" fn insert(mut node: *mut node, mut key: i32) -> *mut node {
    unsafe {
        if node.is_null() {
            return newNode(key);
        }
        if key < (*node).key {
            let ref mut fresh2 = (*node).left;
            *fresh2 = insert((*node).left, key);
        } else {
            let ref mut fresh3 = (*node).right;
            *fresh3 = insert((*node).right, key);
        }
        return node;
    }
}

#[no_mangle]
pub extern "C" fn minValueNode(mut node: *mut node) -> *mut node {
    unsafe {
        let mut current = node;
        while !current.is_null() && !((*current).left).is_null() {
            current = (*current).left;
        }
        return current;
    }
}

#[no_mangle]
pub extern "C" fn deleteNode(mut root: *mut node, mut key: i32) -> *mut node {
    unsafe {
        if root.is_null() {
            return root;
        }
        if key < (*root).key {
            let ref mut fresh4 = (*root).left;
            *fresh4 = deleteNode((*root).left, key);
        } else if key > (*root).key {
            let ref mut fresh5 = (*root).right;
            *fresh5 = deleteNode((*root).right, key);
        } else {
            if ((*root).left).is_null() {
                let mut temp = (*root).right;
                free(root as *mut libc::c_void);
                return temp;
            } else {
                if ((*root).right).is_null() {
                    let mut temp_0 = (*root).left;
                    free(root as *mut libc::c_void);
                    return temp_0;
                }
            }
            let mut temp_1 = minValueNode((*root).right);
            (*root).key = (*temp_1).key;
            let ref mut fresh6 = (*root).right;
            *fresh6 = deleteNode((*root).right, (*temp_1).key);
        }
        return root;
    }
}
