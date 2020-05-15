use std::cmp::{Ord, Ordering};
use std::fmt;

enum Node<T: Ord> {
    Nil,
    Red(T, Box<Node<T>>, Box<Node<T>>),
    Black(T, Box<Node<T>>, Box<Node<T>>),
}

pub struct RedBlackTree<T: Ord> {
    root: Box<Node<T>>,
}

use Node::{Black, Nil, Red};

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::Nil),
        }
    }

    fn contains_inner(node: &Node<T>, value: T) -> bool {
        match node {
            Nil => return false,
            Red(node_value, left, right) | Black(node_value, left, right) => {
                match value.cmp(&node_value) {
                    Ordering::Less => Self::contains_inner(left, value),
                    Ordering::Equal => true,
                    Ordering::Greater => Self::contains_inner(right, value),
                }
            }
        }
    }

    pub fn contains(&self, value: T) -> bool {
        Self::contains_inner(&self.root, value)
    }

    fn insert_inner(node: Box<Node<T>>, value: T) -> (bool, Box<Node<T>>) {
        let nil = || Box::new(Nil);
        let red = |value, left, right| Box::new(Red(value, left, right));
        let black = |value, left, right| Box::new(Black(value, left, right));

        match *node {
            Nil => (true, red(value, nil(), nil())),
            Red(node_value, left, right) => match value.cmp(&node_value) {
                Ordering::Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    (changed, red(node_value, left, right))
                }
                Ordering::Equal => (false, red(node_value, left, right)),
                Ordering::Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    (changed, red(node_value, left, right))
                }
            },
            Black(node_value, left, right) => match value.cmp(&node_value) {
                Ordering::Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    if !changed {
                        return (false, black(node_value, left, right));
                    }
                    let u = node_value;
                    let t4 = right;
                    match *left {
                        Nil => unreachable!(),
                        Red(v, left_left, left_right) => match (*left_left, *left_right) {
                            (Red(w, t1, t2), left_right) => {
                                let t3 = Box::new(left_right);
                                (true, red(v, black(w, t1, t2), black(u, t3, t4)))
                            }
                            (left_left, Red(w, t2, t3)) => {
                                let t1 = Box::new(left_left);
                                (true, red(w, black(v, t1, t2), black(u, t3, t4)))
                            }
                            (left_left, left_right) => (
                                true,
                                black(u, red(v, Box::new(left_left), Box::new(left_right)), t4),
                            ),
                        },
                        Black(v, left_left, left_right) => {
                            (true, black(u, black(v, left_left, left_right), t4))
                        }
                    }
                }
                Ordering::Equal => (false, black(node_value, left, right)),
                Ordering::Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    if !changed {
                        return (false, black(node_value, left, right));
                    }
                    let u = node_value;
                    let t1 = left;
                    match *right {
                        Nil => unreachable!(),
                        Red(v, right_left, right_right) => match (*right_left, *right_right) {
                            (Red(w, t2, t3), right_right) => {
                                let t4 = Box::new(right_right);
                                (true, red(w, black(u, t1, t2), black(v, t3, t4)))
                            }
                            (right_left, Red(w, t3, t4)) => {
                                let t2 = Box::new(right_left);
                                (true, red(v, black(u, t1, t2), black(w, t3, t4)))
                            }
                            (right_left, right_right) => (
                                true,
                                black(u, t1, red(v, Box::new(right_left), Box::new(right_right))),
                            ),
                        },
                        Black(v, right_left, right_right) => {
                            (true, black(u, t1, black(v, right_left, right_right)))
                        }
                    }
                }
            },
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let root = std::mem::replace(&mut self.root, Box::new(Nil));
        let (changed, root) = Self::insert_inner(root, value);
        if !changed {
            self.root = root;
            return false;
        }
        match *root {
            Nil => unreachable!(),
            Red(node_value, left, right) | Black(node_value, left, right) => {
                self.root = Box::new(Black(node_value, left, right));
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
