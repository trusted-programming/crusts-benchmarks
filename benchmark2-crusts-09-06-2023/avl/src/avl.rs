use libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub key: i32,
    pub left: *mut Node,
    pub right: *mut Node,
    pub height: i32,
}
#[no_mangle]
pub extern "C" fn height(mut N: *mut Node) -> i32 {
    unsafe {
        if N.is_null() {
            return 0;
        }
        return (*N).height;
    }
}

#[no_mangle]
pub extern "C" fn max(mut a: i32, mut b: i32) -> i32 {
    return if a > b { a } else { b };
}

#[no_mangle]
pub extern "C" fn newNode(mut key: i32) -> *mut Node {
    unsafe {
        let mut node = malloc(::std::mem::size_of::<Node>() as u64) as *mut Node;
        (*node).key = key;
        let ref mut fresh0 = (*node).left;
        *fresh0 = 0 as *mut Node;
        let ref mut fresh1 = (*node).right;
        *fresh1 = 0 as *mut Node;
        (*node).height = 1;
        return node;
    }
}

#[no_mangle]
pub extern "C" fn rightRotate(mut y: *mut Node) -> *mut Node {
    unsafe {
        let mut x = (*y).left;
        let mut T2 = (*x).right;
        let ref mut fresh2 = (*y).left;
        *fresh2 = T2;
        (*y).height = max(height((*y).left), height((*y).right)) + 1;
        let ref mut fresh3 = (*x).right;
        *fresh3 = y;
        (*x).height = max(height((*x).left), height((*x).right)) + 1;
        return x;
    }
}

#[no_mangle]
pub extern "C" fn leftRotate(mut x: *mut Node) -> *mut Node {
    unsafe {
        let mut y = (*x).right;
        let mut T2 = (*y).left;
        let ref mut fresh4 = (*x).right;
        *fresh4 = T2;
        (*x).height = max(height((*x).left), height((*x).right)) + 1;
        let ref mut fresh5 = (*y).left;
        *fresh5 = x;
        (*y).height = max(height((*y).left), height((*y).right)) + 1;
        return y;
    }
}

#[no_mangle]
pub extern "C" fn getBalance(mut N: *mut Node) -> i32 {
    unsafe {
        if N.is_null() {
            return 0;
        }
        return height((*N).left) - height((*N).right);
    }
}

#[no_mangle]
pub extern "C" fn insert(mut node: *mut Node, mut key: i32) -> *mut Node {
    unsafe {
        if node.is_null() {
            return newNode(key);
        }
        if key < (*node).key {
            let ref mut fresh6 = (*node).left;
            *fresh6 = insert((*node).left, key);
        } else if key > (*node).key {
            let ref mut fresh7 = (*node).right;
            *fresh7 = insert((*node).right, key);
        } else {
            return node;
        };
        (*node).height = 1 + max(height((*node).left), height((*node).right));
        let mut balance = getBalance(node);
        if balance > 1 && key < (*(*node).left).key {
            return rightRotate(node);
        }
        if balance < -1 && key > (*(*node).right).key {
            return leftRotate(node);
        }
        if balance > 1 && key > (*(*node).left).key {
            let ref mut fresh8 = (*node).left;
            *fresh8 = leftRotate((*node).left);
            return rightRotate(node);
        }
        if balance < -1 && key < (*(*node).right).key {
            let ref mut fresh9 = (*node).right;
            *fresh9 = rightRotate((*node).right);
            return leftRotate(node);
        }
        return node;
    }
}

#[no_mangle]
pub extern "C" fn minValueNode(mut node: *mut Node) -> *mut Node {
    unsafe {
        let mut current = node;
        while !((*current).left).is_null() {
            current = (*current).left;
        }
        return current;
    }
}

#[no_mangle]
pub extern "C" fn preOrder(mut root: *mut Node) {
    unsafe {
        if !root.is_null() {
            print!("{} ,9999");
            preOrder((*root).left);
            preOrder((*root).right);
        }
    }
}
