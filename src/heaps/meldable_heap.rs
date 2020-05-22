use std::cmp::Ord;
use std::fmt;
use std::mem;

enum Tree<T: Ord> {
    Nil,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

pub struct MeldableHeap<T: Ord> {
    root: Box<Tree<T>>,
    len: usize,
}

use Tree::{Nil, Node};

impl<T: Ord> MeldableHeap<T> {
    pub fn new() -> Self {
        MeldableHeap {
            root: Box::new(Nil),
            len: 0,
        }
    }

    fn merge(mut h1: Box<Tree<T>>, mut h2: Box<Tree<T>>) -> Box<Tree<T>> {
        match (h1.as_mut(), h2.as_mut()) {
            (Nil, _) => {
                return h2;
            }
            (_, Nil) => {
                return h1;
            }
            (Node(ref v1, _, _), Node(ref v2, _, _)) if v1 > v2 => {
                return Self::merge(h2, h1);
            }
            (Node(_, l, r), _) => {
                if rand::random() {
                    let owned_l = mem::replace(l, Box::new(Nil));
                    *l = Self::merge(owned_l, h2);
                } else {
                    let owned_r = mem::replace(r, Box::new(Nil));
                    *r = Self::merge(owned_r, h2);
                }
                return h1;
            }
        };
    }

    // より Rust らしい実装
    // 本では absorb() となっている
    pub fn append(&mut self, other: &mut MeldableHeap<T>) {
        let owned_root = mem::replace(&mut self.root, Box::new(Nil));
        let owned_other_root = mem::replace(&mut other.root, Box::new(Nil));
        self.root = Self::merge(owned_root, owned_other_root);
        self.len += other.len;
        other.len = 0;
    }

    // 本では add() となっている
    pub fn insert(&mut self, value: T) {
        let node = Box::new(Node(value, Box::new(Nil), Box::new(Nil)));
        let owned_root = mem::replace(&mut self.root, Box::new(Nil));
        self.root = Self::merge(node, owned_root);
        self.len += 1;
    }

    // 本では remove() となっている
    pub fn pop(&mut self) -> Option<T> {
        let owned_root = mem::replace(&mut self.root, Box::new(Nil));
        match *owned_root {
            Nil => {
                return None;
            }
            Node(v, l, r) => {
                self.root = Self::merge(l, r);
                self.len -= 1;
                return Some(v);
            }
        };
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        matches!(*self.root, Tree::Nil)
    }
}

impl<T: Ord + fmt::Debug> fmt::Debug for MeldableHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MeldableHeap {{")?;
        let mut stack = vec![];
        stack.push((0, &self.root));
        while let Some((d, node)) = stack.pop() {
            match node.as_ref() {
                Nil => {}
                Node(v, l, r) => {
                    write!(f, "{}", "    ".repeat(1 + d))?;
                    writeln!(f, "{:?}", v)?;
                    stack.push((d + 1, r));
                    stack.push((d + 1, l));
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
    }
}
