use std::cmp::{
    Ord,
    Ordering::{Equal, Greater, Less},
};
use std::fmt;
use Node::{Black, Nil, Red};

enum Node<T: Ord> {
    Nil,
    Red(T, Box<Node<T>>, Box<Node<T>>),
    Black(T, Box<Node<T>>, Box<Node<T>>),
}

pub struct RedBlackTree<T: Ord> {
    root: Box<Node<T>>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: box Nil }
    }

    fn contains_inner(node: &Node<T>, value: T) -> bool {
        match node {
            Nil => false,
            Red(node_value, left, right) | Black(node_value, left, right) => {
                match value.cmp(&node_value) {
                    Less => Self::contains_inner(left, value),
                    Equal => true,
                    Greater => Self::contains_inner(right, value),
                }
            }
        }
    }

    pub fn contains(&self, value: T) -> bool {
        Self::contains_inner(&self.root, value)
    }

    fn insert_inner(node: Box<Node<T>>, value: T) -> (bool, Box<Node<T>>) {
        match node {
            box Nil => (true, box Red(value, box Nil, box Nil)),
            box Red(node_value, left, right) => match value.cmp(&node_value) {
                Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    (changed, box Red(node_value, left, right))
                }
                Equal => (false, box Red(node_value, left, right)),
                Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    (changed, box Red(node_value, left, right))
                }
            },
            box Black(v, l, r) => match value.cmp(&v) {
                Less => match Self::insert_inner(l, value) {
                    (true, box Red(lv, box Red(llv, lll, llr), lr)) => (
                        true,
                        box Red(lv, box Black(llv, lll, llr), box Black(v, lr, r)),
                    ),
                    (true, box Red(lv, ll, box Red(lrv, lrl, lrr))) => (
                        true,
                        box Red(lrv, box Black(lv, ll, lrl), box Black(v, lrr, r)),
                    ),
                    (changed, l) => (changed, box Black(v, l, r)),
                },
                Equal => (false, box Black(v, l, r)),
                Greater => match Self::insert_inner(r, value) {
                    (true, box Red(rv, box Red(rlv, rll, rlr), rr)) => (
                        true,
                        box Red(rlv, box Black(v, l, rll), box Black(rv, rlr, rr)),
                    ),
                    (true, box Red(rv, rl, box Red(rrv, rrl, rrr))) => (
                        true,
                        box Red(rv, box Black(v, l, rl), box Black(rrv, rrl, rrr)),
                    ),
                    (changed, r) => (changed, box Black(v, l, r)),
                },
            },
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let root = std::mem::replace(&mut self.root, box Nil);
        let (changed, root) = Self::insert_inner(root, value);
        if !changed {
            self.root = root;
            return false;
        }
        match root {
            box Nil => unreachable!(),
            box Red(node_value, left, right) | box Black(node_value, left, right) => {
                self.root = box Black(node_value, left, right);
                true
            }
        }
    }
}

impl<T: Ord + fmt::Debug> fmt::Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RedBlackTree {{")?;
        let mut stack = vec![];
        stack.push((0, self.root.as_ref()));
        while let Some((depth, node)) = stack.pop() {
            match node {
                Nil => (),
                Red(value, left, right) => {
                    write!(f, "{}", "    ".repeat(1 + depth))?;
                    writeln!(f, "Red({:?})", value)?;
                    stack.push((depth + 1, right));
                    stack.push((depth + 1, left));
                }
                Black(value, left, right) => {
                    write!(f, "{}", "    ".repeat(1 + depth))?;
                    writeln!(f, "Black({:?})", value)?;
                    stack.push((depth + 1, right));
                    stack.push((depth + 1, left));
                }
            }
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::RedBlackTree;

    #[test]
    fn test_red_black_tree() {
        let mut tree = RedBlackTree::new();
        for i in 0..100 {
            let i = (i % 10) * 10 + (i / 10);
            if i % 2 == 0 {
                assert_eq!(tree.insert(i), true);
            }
        }
        for i in 0..100 {
            assert_eq!(tree.contains(i), i % 2 == 0);
        }
        dbg!(&tree);
    }
}
