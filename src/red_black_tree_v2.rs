use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use std::mem;

enum Color {
    Red,
    Black,
}

use Color::{Black, Red};

// 外部の型に実装を加えることはできないので、扱いやすくするためのラッパー
// null pointer optimization の恩恵を受けるため Option<Box< >> を使用
struct Node<T>(Option<Box<NodeInner<T>>>);

// 内部構造
struct NodeInner<T> {
    color: Color,
    value: T,
    left: Node<T>,
    right: Node<T>,
}

impl<T: Ord> Node<T> {
    // 判定系メソッド
    fn is_null(&self) -> bool {
        self.0.is_none()
    }

    fn is_red(&self) -> bool {
        match self.0.as_ref() {
            Some(b) => matches!(b.color, Red),
            None => false,
        }
    }

    fn is_black(&self) -> bool {
        !self.is_red()
    }

    // 参照系メソッド
    fn color(&mut self) -> &mut Color {
        &mut self.0.as_mut().unwrap().color
    }

    fn value(&mut self) -> &mut T {
        &mut self.0.as_mut().unwrap().value
    }

    fn left(&mut self) -> &mut Node<T> {
        &mut self.0.as_mut().unwrap().left
    }

    fn right(&mut self) -> &mut Node<T> {
        &mut self.0.as_mut().unwrap().right
    }

    // 値を奪う
    fn take(&mut self) -> Self {
        Node(self.0.take())
    }

    // 値を差し替える
    fn replace(&mut self, src: Node<T>) -> Self {
        mem::replace(self, src)
    }

    fn take_left(&mut self) -> Self {
        self.left().take()
    }

    fn replace_left(&mut self, left: Self) -> Self {
        self.left().replace(left)
    }

    fn take_right(&mut self) -> Self {
        self.right().take()
    }

    fn replace_right(&mut self, right: Self) -> Self {
        self.right().replace(right)
    }

    fn push_black(&mut self) {
        *self.color() = Red;
        *self.left().color() = Black;
        *self.right().color() = Black;
    }

    fn pull_black(&mut self) {
        *self.color() = Black;
        *self.left().color() = Red;
        *self.right().color() = Red;
    }

    // 色交換
    fn swap_colors(&mut self, other: &mut Self) {
        mem::swap(self.color(), other.color())
    }

    // 左回転
    //    u          w
    //  w   c  <=  a   u
    // a b            b c
    fn rotate_left(&mut self) {
        let mut w = self.0.take().unwrap();
        let mut u = w.right.0.take().unwrap();
        w.right.0 = u.left.0.take();
        u.left.0.replace(w);
        self.0.replace(u);
    }

    // 右回転
    //    u          w
    //  w   c  =>  a   u
    // a b            b c
    fn rotate_right(&mut self) {
        let mut u = self.0.take().unwrap();
        let mut w = u.left.0.take().unwrap();
        u.left.0 = w.right.0.take();
        w.right.0.replace(u);
        self.0.replace(w);
    }

    // 左回転・色交換
    fn flip_left(&mut self) {
        let mut right = self.take_right();
        self.swap_colors(&mut right);
        self.replace_right(right);
        self.rotate_left();
    }

    // 右回転・色交換
    fn flip_right(&mut self) {
        let mut left = self.take_left();
        self.swap_colors(&mut left);
        self.replace_left(left);
        self.rotate_right();
    }

    // 検索
    fn contains(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(b) => match value.cmp(&b.value) {
                Less => b.left.contains(value),
                Equal => true,
                Greater => b.right.contains(value),
            },
        }
    }

    // 挿入
    fn insert(&mut self, value: T) -> bool {
        if self.is_null() {
            self.0.replace(Box::new(NodeInner {
                color: Red,
                value,
                left: Node(None),
                right: Node(None),
            }));
            return true;
        } else {
            let changed = match value.cmp(self.value()) {
                Less => {
                    let changed = self.left().insert(value);
                    changed
                }
                Equal => false,
                Greater => {
                    let changed = self.right().insert(value);
                    changed
                }
            };
            if changed {
                self.insert_fixup();
            }
            return changed;
        };
    }

    // 挿入に伴う修正
    fn insert_fixup(&mut self) {
        // 左傾性を保つ
        if self.left().is_black() && self.right().is_red() {
            self.flip_left();
        }
        if self.is_black() && self.left().is_red() {
            if self.right().is_red() {
                if self.left().left().is_red() {
                    self.push_black();
                }
                if self.right().left().is_red() {
                    self.push_black();
                }
            } else {
                if self.left().left().is_red() {
                    self.flip_right();
                }
            }
        }
    }

    // 削除
    fn remove(&mut self, value: &T) {
        match self.0.as_mut() {
            None => (),
            Some(b) => match value.cmp(&b.value) {
                Less => b.left.remove(value),
                Equal => {
                    // 右子が空なら左子に差し替え
                    // そうでなければ右部分木の最小を取ってきてそれに差し替え
                    if b.right.is_null() {
                        let n = *self.0.take().unwrap();
                        mem::replace(self, n.left);
                    } else {
                        let (value, mut double) = b.right.remove_min();
                        *self.value() = value;
                        if double {
                            double = self.remove_fixup_right();
                        }
                    }
                }
                Greater => b.right.remove(value),
            },
        }
    }

    // 最小値の削除
    // (取り除かれた値, self が double black であるかどうか)
    fn remove_min(&mut self) -> (T, bool) {
        if self.left().is_null() {
            // 左が空なら右は黒なので、取り除かれた節が黒 ⇔ double
            let n = *self.0.take().unwrap();
            mem::replace(self, n.right);
            (n.value, matches!(n.color, Black))
        } else {
            let (value, mut double) = self.left().remove_min();
            if double {
                double = self.remove_fixup_left();
            }
            (value, double)
        }
    }

    // 左部分木のノード削除に伴う修正
    fn remove_fixup_left(&mut self) -> bool {
        // Case 2
        if self.right().is_black() {
            *self.right().color() = Red;
            self.flip_left();
            if self.left().right().is_black() {
                if self.is_red() {
                    *self.color() = Black;
                    return false;
                } else {
                    return true;
                }
            } else {
                self.left().rotate_left();
                self.flip_right();
                *self.left().color() = Black;
                *self.right().color() = Black;
                if self.right().right().is_black() {
                    return false;
                } else {
                    self.right().flip_left();
                    return false;
                }
            }
        }
        false
    }

    // 右部分木のノード削除に伴う修正
    fn remove_fixup_right(&mut self) -> bool {
        // Case 1
        if self.left().is_red() {
            self.flip_right();
            // self.right() は赤であることが確定しているので double にはならない
            let double = self.right().remove_fixup_right();
            return false;
        // Case 3
        } else {
            *self.left().color() = Red;
            self.flip_right();
            if self.right().left().is_red() {
                self.right().rotate_right();
                self.flip_left();
                *self.left().color() = Black;
                *self.right().color() = Black;
                return false;
            } else {
                if self.left().is_red() {
                    *self.left().color() = Black;
                    *self.right().color() = Black;
                    return false;
                } else {
                    self.flip_left();
                    if self.is_red() {
                        *self.color() = Black;
                        return false;
                    } else {
                        return true;
                    }
                }
            }
        }
    }
}

pub struct RedBlackTree<T: Ord> {
    root: Node<T>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: Node(None) }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root.contains(value)
    }

    pub fn insert(&mut self, value: T) -> bool {
        let changed = self.root.insert(value);
        *self.root.color() = Black;
        changed
    }

    pub fn remove(&mut self, value: &T) {
        self.root.remove(value)
    }
}

impl<T: Ord + fmt::Debug> fmt::Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn show<T: Ord + fmt::Debug>(node: &Node<T>) -> (usize, usize, Vec<String>) {
            if node.is_null() {
                return (0, 0, vec!["".to_string()]);
            } else {
                let (l, li, left) = show(&node.0.as_ref().unwrap().left);
                let (r, ri, right) = show(&node.0.as_ref().unwrap().right);
                let mut v = vec![];
                let fs = if node.is_black() {
                    format!("{:02?}", &node.0.as_ref().unwrap().value)
                } else {
                    format!("\x1b[31m{:02?}\x1b[m", &node.0.as_ref().unwrap().value)
                };
                v.push(
                    " ".repeat(li)
                        + &"_".repeat(l - li)
                        + &fs
                        + &"_".repeat(ri)
                        + &" ".repeat(r - ri),
                );
                for i in 0..std::cmp::max(left.len(), right.len()) {
                    v.push(
                        left.get(i)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| " ".repeat(l))
                            + "  "
                            + &right
                                .get(i)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| " ".repeat(r)),
                    );
                }
                return (l + 2 + r, l + 1, v);
            }
        }
        writeln!(f)?;
        let (_, _, v) = show(&self.root);
        for l in v {
            writeln!(f, "{}", l)?;
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::RedBlackTree;
    use rand;
    use rand::seq::SliceRandom;

    #[test]
    fn test_red_black_tree() {
        let mut tree = RedBlackTree::new();
        let mut v = (0..100).collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        v.shuffle(&mut rng);
        for i in 0..100 {
            if v[i] % 2 == 0 {
                assert_eq!(tree.insert(v[i]), true);
            }
        }
        dbg!(&tree);
        for i in 0..100 {
            assert_eq!(tree.contains(&i), i % 2 == 0);
        }
        v.shuffle(&mut rng);
        for i in 0..100 {
            // assert_eq!(tree.remove(&v[i]), v[i] % 2 == 0);
            dbg!(v[i]);
            tree.remove(&v[i]);
            dbg!(&tree);
        }
    }
}