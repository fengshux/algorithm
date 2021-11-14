


pub struct Tree{    
    root: *mut Node,
    count: usize,
}

impl Tree{
    pub fn new() -> Self {
        Tree{
            root: std::ptr::null_mut(),
            count: 0
        }
    }
    // add a new item into the tree. returns `true` if
    // zhe insertion happend, and `false` if the given
    // data was already present in the tree.
    pub fn add(&mut self, data: i32) -> bool {
        let node: *mut Node = Node::new(data);
        
        // TODO

        
        return true;
    }
}

#[derive(Debug)]
pub struct Node {
    data: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    pub fn new(data: i32) -> *mut Self {
        Box::into_raw(Box::new(Self{
            data: data,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        }))
    }
}


