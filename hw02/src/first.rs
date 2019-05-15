#[derive(Debug)]
pub struct BST {
    root: Link,
}

impl BST {
    pub fn new() -> BST {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: i32) -> bool {
        self.root.search(elem)
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl Link {
    pub fn insert(&mut self, elem: i32) -> bool {
        match self {
            Link::Empty => {
                let node = Box::new(Node {
                    elem,
                    left: Link::Empty,
                    right: Link::Empty,
                });
                
                std::mem::replace(self, Link::More(node));
                true
            },
            Link::More(node) => {
                if elem < node.elem {
                    node.left.insert(elem)
                } else if elem > node.elem {
                    node.right.insert(elem)
                } else {
                    false
                }
            }
        }
    }

    pub fn search(&self, elem: i32) -> bool {
        match self {
            Link::Empty => false,
            Link::More(node) => {
                if elem < node.elem {
                    node.left.search(elem)
                } else if elem > node.elem {
                    node.right.search(elem)
                } else {
                    true
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut tree = BST::new();

        // Shouldn't find anything
        assert!(!tree.search(4));

        // Populate tree; these should all return true
        assert!(tree.insert(2));
        assert!(tree.insert(1));
        assert!(tree.insert(3));
        assert!(tree.insert(5));
        assert!(tree.insert(4));

        // Element already in tree
        assert!(!tree.insert(4));

        // Now we should find this
        assert!(tree.search(4));

        // But not this
        assert!(!tree.search(6));
    }
}