use std::fmt;

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
        match (h1.as_mut(), h2.as_mut()) {
            (None, _) => h2,
            (_, None) => h1,
            (Some(b1), Some(b2)) if b1.value > b2.value => Self::merge(h2, h1),
            (Some(b1), _) => {
                if rand::random() {
                    b1.left = Self::merge(b1.left.take(), h2);
                } else {
                    b1.right = Self::merge(b1.right.take(), h2);
                }
                h1
            }
        }
    }

    // より Rust らしい実装
    // 本では absorb() となっている
    pub fn append(&mut self, other: &mut MeldableHeap<T>) {
        self.root = Self::merge(self.root.take(), other.root.take());
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
        self.root = Self::merge(node, self.root.take());
        self.len += 1;
    }

    // 本では remove() となっている
    pub fn pop(&mut self) -> Option<T> {
        match self.root.take() {
            None => None,
            Some(b) => {
                self.root = Self::merge(b.left, b.right);
                self.len -= 1;
                Some(b.value)
            }
        }
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
            h1.insert(i * 2);
        }
        dbg!(&h1);
        let mut h2 = MeldableHeap::new();
        for i in 0..16 {
            h2.insert(i * 2 + 1);
        }
        dbg!(&h2);
        h1.append(&mut h2);
        assert!(h2.is_empty());
        dbg!(&h1);
        assert_eq!(h1.pop(), Some(0));
        assert_eq!(h1.pop(), Some(1));
        assert_eq!(h1.pop(), Some(2));
        dbg!(&h1);
    }
}
