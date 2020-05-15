use std::cmp::{Ord, Ordering};

enum Node<T>
where
    T: Ord,
{
    Nil,
    Red(T, Box<Node<T>>, Box<Node<T>>),
    Black(T, Box<Node<T>>, Box<Node<T>>),
}

struct RedBlackTree<T>
where
    T: Ord,
{
    root: Box<Node<T>>,
}

use Node::{Black, Nil, Red};

impl<T> RedBlackTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        Self {
            root: Box::new(Node::Nil),
        }
    }

    fn contains_inner(node: &Node<T>, value: T) -> bool {
        let (node_value, left, right) = match node {
            Nil => return false,
            Red(node_value, left, right) => (node_value, left, right),
            Black(node_value, left, right) => (node_value, left, right),
        };
        match value.cmp(&node_value) {
            Ordering::Less => Self::contains_inner(left, value),
            Ordering::Equal => true,
            Ordering::Greater => Self::contains_inner(right, value),
        }
    }

    fn contains(&self, value: T) -> bool {
        Self::contains_inner(&self.root, value)
    }

    fn insert_inner(node: Box<Node<T>>, value: T) -> (bool, Box<Node<T>>) {
        match *node {
            Nil => (true, Box::new(Red(value, Box::new(Nil), Box::new(Nil)))),
            Red(node_value, left, right) => match value.cmp(&node_value) {
                Ordering::Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    (changed, Box::new(Red(node_value, left, right)))
                }
                Ordering::Equal => (false, Box::new(Red(node_value, left, right))),
                Ordering::Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    (changed, Box::new(Red(node_value, left, right)))
                }
            },
            Black(node_value, left, right) => match value.cmp(&node_value) {
                Ordering::Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    if !changed {
                        return (false, Box::new(Black(node_value, left, right)));
                    }
                    let u = node_value;
                    let t4 = right;
                    match *left {
                        Red(v, left_left, left_right) => match (*left_left, *left_right) {
                            (Red(w, t1, t2), left_right) => {
                                let t3 = Box::new(left_right);
                                (
                                    true,
                                    Box::new(Red(
                                        v,
                                        Box::new(Black(w, t1, t2)),
                                        Box::new(Black(u, t3, t4)),
                                    )),
                                )
                            }
                            (left_left, Red(w, t2, t3)) => {
                                let t1 = Box::new(left_left);
                                (
                                    true,
                                    Box::new(Red(
                                        w,
                                        Box::new(Black(v, t1, t2)),
                                        Box::new(Black(u, t3, t4)),
                                    )),
                                )
                            }
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
                Ordering::Equal => (false, Box::new(Black(node_value, left, right))),
                Ordering::Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    if !changed {
                        return (false, Box::new(Black(node_value, left, right)));
                    }
                    let u = node_value;
                    let t1 = left;
                    match *right {
                        Red(v, right_left, right_right) => match (*right_left, *right_right) {
                            (Red(w, t2, t3), right_right) => {
                                let t4 = Box::new(right_right);
                                (
                                    true,
                                    Box::new(Red(
                                        w,
                                        Box::new(Black(u, t1, t2)),
                                        Box::new(Black(v, t3, t4)),
                                    )),
                                )
                            }
                            (right_left, Red(w, t3, t4)) => {
                                let t2 = Box::new(right_left);
                                (
                                    true,
                                    Box::new(Red(
                                        v,
                                        Box::new(Black(u, t1, t2)),
                                        Box::new(Black(w, t3, t4)),
                                    )),
                                )
                            }
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
            },
        }
    }

    fn insert(&mut self, value: T) -> bool {
        let root = std::mem::replace(&mut self.root, Box::new(Nil));
        let (changed, root) = Self::insert_inner(root, value);
        if !changed {
            self.root = root;
            return false;
        }
        match *root {
            Nil => unreachable!(),
            Red(node_value, left, right) => {
                self.root = Box::new(Black(node_value, left, right));
                true
            }
            Black(node_value, left, right) => {
                self.root = Box::new(Black(node_value, left, right));
                true
            }
        }
    }
}

#[test]
fn test_red_black_tree() {
    let tree = RedBlackTree::new();
    tree.insert(1);
    assert_eq!(tree.contains(1), true);
    assert_eq!(tree.contains(2), false);
    tree.insert(2);
    assert_eq!(tree.contains(2), true);
}
