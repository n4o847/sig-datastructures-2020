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
        Self {
            root: Box::new(Nil),
        }
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
        let nil = || Box::new(Nil);
        let red = |value, left, right| Box::new(Red(value, left, right));
        let black = |value, left, right| Box::new(Black(value, left, right));

        match *node {
            Nil => (true, red(value, nil(), nil())),
            Red(node_value, left, right) => match value.cmp(&node_value) {
                Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    (changed, red(node_value, left, right))
                }
                Equal => (false, red(node_value, left, right)),
                Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    (changed, red(node_value, left, right))
                }
            },
            Black(node_value, left, right) => match value.cmp(&node_value) {
                Less => {
                    let (changed, left) = Self::insert_inner(left, value);
                    if !changed {
                        return (false, black(node_value, left, right));
                    }
                    let (v, l, r) = (node_value, left, right);
                    if let Red(lv, ll, lr) = *l {
                        if let Red(llv, lll, llr) = *ll {
                            (true, red(lv, black(llv, lll, llr), black(v, lr, r)))
                        } else if let Red(lrv, lrl, lrr) = *lr {
                            (true, red(lrv, black(lv, ll, lrl), black(v, lrr, r)))
                        } else {
                            (true, black(v, red(lv, ll, lr), r))
                        }
                    } else {
                        (true, black(v, l, r))
                    }
                }
                Equal => (false, black(node_value, left, right)),
                Greater => {
                    let (changed, right) = Self::insert_inner(right, value);
                    if !changed {
                        return (false, black(node_value, left, right));
                    }
                    let (v, l, r) = (node_value, left, right);
                    if let Red(rv, rl, rr) = *r {
                        if let Red(rlv, rll, rlr) = *rl {
                            (true, red(rlv, black(v, l, rll), black(rv, rlr, rr)))
                        } else if let Red(rrv, rrl, rrr) = *rr {
                            (true, red(rv, black(v, l, rl), black(rrv, rrl, rrr)))
                        } else {
                            (true, black(v, l, red(rv, rl, rr)))
                        }
                    } else {
                        (true, black(v, l, r))
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
                    writeln!(f, "\x1b[31m{:?}\x1b[m", value)?;
                    stack.push((depth + 1, right));
                    stack.push((depth + 1, left));
                }
                Black(value, left, right) => {
                    write!(f, "{}", "    ".repeat(1 + depth))?;
                    writeln!(f, "{:?}", value)?;
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
