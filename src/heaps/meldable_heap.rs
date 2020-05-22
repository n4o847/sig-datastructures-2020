use std::cmp::Ord;
use std::fmt;
use std::mem;

struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct MeldableHeap<T: Ord> {
    root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: Ord> MeldableHeap<T> {
    pub fn new() -> Self {
        MeldableHeap { root: None, len: 0 }
    }

    fn merge(mut h1: Option<Box<Node<T>>>, mut h2: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match (&mut h1, &mut h2) {
            (None, _) => {
                return h2;
            }
            (_, None) => {
                return h1;
            }
            (Some(b1), Some(b2)) if b1.value > b2.value => {
                return Self::merge(h2, h1);
            }
            (Some(b1), _) => {
                if rand::random() {
                    let owned_l = mem::replace(&mut b1.left, None);
                    b1.left = Self::merge(owned_l, h2);
                } else {
                    let owned_r = mem::replace(&mut b1.right, None);
                    b1.right = Self::merge(owned_r, h2);
                }
                return h1;
            }
        };
    }

    // より Rust らしい実装
    // 本では absorb() となっている
    pub fn append(&mut self, other: &mut MeldableHeap<T>) {
        let owned_root = mem::replace(&mut self.root, None);
        let owned_other_root = mem::replace(&mut other.root, None);
        self.root = Self::merge(owned_root, owned_other_root);
        self.len += other.len;
        other.len = 0;
    }

    // 本では add() となっている
    pub fn insert(&mut self, value: T) {
        let node = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
        let owned_root = mem::replace(&mut self.root, None);
        self.root = Self::merge(node, owned_root);
        self.len += 1;
    }

    // 本では remove() となっている
    pub fn pop(&mut self) -> Option<T> {
        let owned_root = mem::replace(&mut self.root, None);
        match owned_root {
            None => {
                return None;
            }
            Some(b) => {
                let Node { value, left, right } = *b;
                self.root = Self::merge(left, right);
                self.len -= 1;
                return Some(value);
            }
        };
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl<T: Ord + fmt::Debug> fmt::Debug for MeldableHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MeldableHeap {{")?;
        let mut stack = vec![];
        stack.push((0, &self.root));
        while let Some((d, node)) = stack.pop() {
            match node {
                None => {}
                Some(b) => {
                    write!(f, "{}", "    ".repeat(1 + d))?;
                    writeln!(f, "{:?}", b.value)?;
                    stack.push((d + 1, &b.right));
                    stack.push((d + 1, &b.left));
                }
            };
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::MeldableHeap;

    #[test]
    fn test_meldable_heap() {
        let mut h1 = MeldableHeap::new();
        for i in 0..16 {
            h1.insert(i);
        }
        dbg!(&h1);
        let mut h2 = MeldableHeap::new();
        for i in 0..16 {
            h2.insert(i);
        }
        dbg!(&h2);
        h1.append(&mut h2);
        dbg!(&h1);
        assert_eq!(h1.pop(), Some(0));
        assert!(h2.is_empty());
    }
}
