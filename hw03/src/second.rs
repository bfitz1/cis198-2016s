// Per the course's suggestion, only the right subtree
// is iterated through.

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

impl<T: PartialOrd> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn insert(&mut self, elem: T) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: T) -> bool {
        self.root.search(elem)
    }
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { next: self.root.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { next: self.root.as_mut().map(|node| &mut **node) }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

trait InsertSearch {
    type Item;

    fn insert(&mut self, elem: Self::Item) -> bool;
    fn search(&self, elem: Self::Item) -> bool;
}

impl<T: PartialOrd> InsertSearch for Link<T> {
    type Item = T;

    fn insert(&mut self, elem: Self::Item) -> bool {
        match self {
            None => {
                let node = Box::new(Node {
                    elem,
                    left: None,
                    right: None,
                });
                
                self.replace(node);
                true
            },
            Some(node) => {
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

    fn search(&self, elem: Self::Item) -> bool {
        match self {
            None => false,
            Some(node) => {
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

pub struct IntoIter<T>(BST<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|node| {
            self.0.root = node.right;
            node.elem
        })
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
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

    #[test]
    fn for_loop() {
        let mut tree = BST::new();
        tree.insert(1); tree.insert(2); tree.insert(3);

        for _ in (&tree).into_iter() {}

        for _ in (&mut tree).into_iter() {}

        for _ in tree.into_iter() {}
    }

    #[test]
    fn into_iter() {
        let mut tree = BST::new();
        tree.insert(1); tree.insert(2); tree.insert(3);

        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut tree = BST::new();
        tree.insert(1); tree.insert(2); tree.insert(3);

        let mut iter = (&tree).into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut tree = BST::new();
        tree.insert(1); tree.insert(2); tree.insert(3);

        let mut iter = (&mut tree).into_iter();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}