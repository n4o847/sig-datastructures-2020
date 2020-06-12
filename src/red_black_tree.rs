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
    // 参照系メソッド
    fn color(&self) -> &Color {
        &self.0.as_ref().unwrap().color
    }

    fn color_mut(&mut self) -> &mut Color {
        &mut self.0.as_mut().unwrap().color
    }

    fn value(&self) -> &T {
        &self.0.as_ref().unwrap().value
    }

    fn value_mut(&mut self) -> &mut T {
        &mut self.0.as_mut().unwrap().value
    }

    fn left(&self) -> &Node<T> {
        &self.0.as_ref().unwrap().left
    }

    fn left_mut(&mut self) -> &mut Node<T> {
        &mut self.0.as_mut().unwrap().left
    }

    fn right(&self) -> &Node<T> {
        &self.0.as_ref().unwrap().right
    }

    fn right_mut(&mut self) -> &mut Node<T> {
        &mut self.0.as_mut().unwrap().right
    }

    // 判定系メソッド
    fn is_null(&self) -> bool {
        self.0.is_none()
    }

    fn is_red(&self) -> bool {
        if self.is_null() {
            false
        } else {
            matches!(self.color(), Red)
        }
    }

    fn is_black(&self) -> bool {
        !self.is_red()
    }

    // 値を奪う
    fn take(&mut self) -> Self {
        Node(self.0.take())
    }

    // 色交換
    fn swap_colors(&mut self, other: &mut Self) {
        mem::swap(self.color_mut(), other.color_mut())
    }

    // 左回転
    //    u          w
    //  w   c  <=  a   u
    // a b            b c
    fn rotate_left(&mut self) {
        let mut w = self.take();
        let mut u = w.right_mut().take();
        *w.right_mut() = u.left_mut().take();
        *u.left_mut() = w;
        *self = u;
    }

    // 右回転
    //    u          w
    //  w   c  =>  a   u
    // a b            b c
    fn rotate_right(&mut self) {
        let mut u = self.take();
        let mut w = u.left_mut().take();
        *u.left_mut() = w.right_mut().take();
        *w.right_mut() = u;
        *self = w;
    }

    // 左回転・色交換
    fn flip_left(&mut self) {
        let mut right = self.right_mut().take();
        self.swap_colors(&mut right);
        *self.right_mut() = right;
        self.rotate_left();
    }

    // 右回転・色交換
    fn flip_right(&mut self) {
        let mut left = self.left_mut().take();
        self.swap_colors(&mut left);
        *self.left_mut() = left;
        self.rotate_right();
    }

    // 検索
    fn contains(&self, value: &T) -> bool {
        if self.is_null() {
            false
        } else {
            match value.cmp(self.value()) {
                Less => self.left().contains(value),
                Equal => true,
                Greater => self.right().contains(value),
            }
        }
    }

    // 挿入
    fn insert(&mut self, value: T) -> bool {
        if self.is_null() {
            *self = Node(Some(Box::new(NodeInner {
                color: Red,
                value,
                left: Node(None),
                right: Node(None),
            })));
            true
        } else {
            let changed = match value.cmp(self.value()) {
                Less => self.left_mut().insert(value),
                Equal => false,
                Greater => self.right_mut().insert(value),
            };
            if changed {
                self.insert_fixup();
            }
            changed
        }
    }

    // 挿入に伴う修正
    fn insert_fixup(&mut self) {
        // 左傾性を保つ
        if self.left().is_black() && self.right().is_red() {
            self.flip_left();
        }
        if self.is_black() && self.left().is_red() {
            if self.right().is_red() {
                if self.left().left().is_red() || self.right().left().is_red() {
                    *self.color_mut() = Red;
                    *self.left_mut().color_mut() = Black;
                    *self.right_mut().color_mut() = Black;
                }
            } else {
                if self.left().left().is_red() {
                    self.flip_right();
                }
            }
        }
    }

    // 削除
    // (削除されたかどうか, double black かどうか)
    fn remove(&mut self, value: &T) -> (bool, bool) {
        if self.is_null() {
            (false, false)
        } else {
            let (change, mut double) = match value.cmp(self.value()) {
                Less => {
                    let (changed, mut double) = self.left_mut().remove(value);
                    if double {
                        double = self.remove_fixup_left();
                    }
                    (changed, double)
                }
                Equal => {
                    // 右子が空なら左子に差し替え
                    // そうでなければ右部分木の最小を取ってきてそれに差し替え
                    if self.right().is_null() {
                        let n = *self.0.take().unwrap();
                        *self = n.left;
                        (true, matches!(n.color, Black))
                    } else {
                        let (value, mut double) = self.right_mut().remove_min();
                        *self.value_mut() = value;
                        if double {
                            double = self.remove_fixup_right();
                        }
                        (true, double)
                    }
                }
                Greater => {
                    let (changed, mut double) = self.right_mut().remove(value);
                    if double {
                        double = self.remove_fixup_right();
                    }
                    (changed, double)
                }
            };
            // 左傾性を保つ
            if !self.is_null() && self.left().is_black() && self.right().is_red() {
                self.flip_left();
            }
            if double && self.is_red() {
                *self.color_mut() = Black;
                double = false;
            }
            (change, double)
        }
    }

    // 最小値の削除
    // (取り除かれた値, self が double black であるかどうか)
    fn remove_min(&mut self) -> (T, bool) {
        if self.left().is_null() {
            // 左が空なら右は黒なので、取り除かれた節が黒 ⇔ double
            let n = *self.0.take().unwrap();
            *self = n.right;
            let mut double = matches!(n.color, Black);
            if double && self.is_red() {
                *self.color_mut() = Black;
                double = false;
            }
            (n.value, double)
        } else {
            let (value, mut double) = self.left_mut().remove_min();
            if double {
                double = self.remove_fixup_left();
            }
            // 左傾性を保つ
            if !self.is_null() && self.left().is_black() && self.right().is_red() {
                self.flip_left();
            }
            (value, double)
        }
    }

    // 左部分木のノード削除に伴う修正
    fn remove_fixup_left(&mut self) -> bool {
        // Case 2
        if !self.right().is_null() && self.right().is_black() {
            *self.right_mut().color_mut() = Red;
            self.flip_left();
            if self.left().right().is_black() {
                if self.is_red() {
                    *self.color_mut() = Black;
                    false
                } else {
                    true
                }
            } else {
                self.left_mut().rotate_left();
                self.flip_right();
                *self.left_mut().color_mut() = Black;
                *self.right_mut().color_mut() = Black;
                if self.right().right().is_black() {
                    false
                } else {
                    self.right_mut().flip_left();
                    false
                }
            }
        } else {
            false
        }
    }

    // 右部分木のノード削除に伴う修正
    fn remove_fixup_right(&mut self) -> bool {
        // Case 1
        if self.is_black() && self.left().is_red() {
            self.flip_right();
            // self.right() は赤であることが確定しているので double にはならない
            let double = self.right_mut().remove_fixup_right();
            debug_assert!(!double);
            false
        // Case 3
        } else if !self.left().is_null() && self.left().is_black() && self.right().is_black() {
            *self.left_mut().color_mut() = Red;
            self.flip_right();
            if self.right().left().is_red() {
                self.right_mut().rotate_right();
                self.flip_left();
                *self.left_mut().color_mut() = Black;
                *self.right_mut().color_mut() = Black;
                false
            } else {
                if self.left().is_red() {
                    *self.left_mut().color_mut() = Black;
                    *self.right_mut().color_mut() = Black;
                    false
                } else {
                    self.flip_left();
                    if self.is_red() {
                        *self.color_mut() = Black;
                        false
                    } else {
                        true
                    }
                }
            }
        } else {
            false
        }
    }

    fn check(&self) -> Result<usize, &str> {
        if self.is_null() {
            Ok(1)
        } else {
            if self.is_red() && (self.left().is_red() || self.right().is_red()) {
                return Err("Property 9.4 (no-red-edge) not satisfied.");
            }
            if self.left().is_black() && self.right().is_red() {
                return Err("Property 9.5 (left-leaning) not satisfied.");
            }
            let l = self.left().check()?;
            let r = self.right().check()?;
            if l != r {
                return Err("Property 9.3 (black-height) not satisfied.");
            }
            if self.is_red() {
                Ok(l)
            } else {
                Ok(l + 1)
            }
        }
    }
}

pub struct RedBlackTree<T: Ord> {
    root: Node<T>,
    len: usize,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self {
            root: Node(None),
            len: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root.contains(value)
    }

    pub fn insert(&mut self, value: T) -> bool {
        let changed = self.root.insert(value);
        *self.root.color_mut() = Black;
        if changed {
            self.len += 1;
        }
        changed
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let (changed, _double) = self.root.remove(value);
        if changed {
            self.len -= 1;
        }
        changed
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn check(&self) -> Result<(), &str> {
        self.root.check()?;
        Ok(())
    }
}

impl<T: Ord + fmt::Debug> fmt::Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn show<T: Ord + fmt::Debug>(node: &Node<T>) -> (usize, usize, Vec<String>) {
            if node.is_null() {
                (0, 0, vec![])
            } else {
                let (l, li, left) = show(node.left());
                let (r, ri, right) = show(node.right());
                let mut v = vec![];
                let fs = if node.is_black() {
                    format!("{:02?}", node.value())
                } else {
                    format!("\x1b[31m{:02?}\x1b[m", node.value())
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
                        left.get(i).map_or_else(|| " ".repeat(l), |s| s.to_string())
                            + "  "
                            + &right
                                .get(i)
                                .map_or_else(|| " ".repeat(r), |s| s.to_string()),
                    );
                }
                (l + 2 + r, l + 1, v)
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
    use rand::seq::SliceRandom;

    #[test]
    fn test_red_black_tree() {
        let mut tree = RedBlackTree::new();
        let mut v = (0..100).collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        v.shuffle(&mut rng);
        for i in 0..100 {
            if v[i] % 2 == 0 {
                println!("> insert({:?})", v[i]);
                assert_eq!(tree.insert(v[i]), true);
                println!("{:?}", tree);
                tree.check().unwrap();
            }
        }
        for i in 0..100 {
            assert_eq!(tree.contains(&i), i % 2 == 0);
        }
        v.shuffle(&mut rng);
        for i in 0..100 {
            println!("> remove({:?})", v[i]);
            if v[i] % 2 == 0 {
                assert_eq!(tree.remove(&v[i]), true);
                println!("{:?}", tree);
                tree.check().unwrap();
            } else {
                assert_eq!(tree.remove(&v[i]), false);
            }
        }
    }
}
