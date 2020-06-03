use std::mem;

pub struct LinearHashTable<T> {
    t: Vec<Item<T>>,
    n: usize, // 値の個数
    q: usize, // null でない値の個数
    d: usize, // t.len() == 1 << d
}

#[derive(PartialEq)]
enum Item<T> {
    Value(T),
    Null,
    Del,
}

pub trait Hashable {
    fn hash(&self) -> usize {
        0
    }
}

impl<T: Hashable + Eq> LinearHashTable<T> {
    pub fn new() -> Self {
        LinearHashTable {
            t: vec![Item::Null],
            n: 0,
            q: 0,
            d: 0,
        }
    }

    // 本では find() となっている
    fn get(&self, x: &T) -> Option<&T> {
        let mut i = x.hash();
        while self.t[i] != Item::Null {
            if let Item::Value(y) = &self.t[i] {
                if y == x {
                    return Some(y);
                }
            }
            i = if i + 1 == self.t.len() { 0 } else { i + 1 };
        }
        None
    }

    // 本では add() となっている
    pub fn insert(&mut self, x: T) -> bool {
        if self.get(&x).is_some() {
            return false;
        }
        if 2 * (self.q + 1) > self.t.len() {
            self.resize();
        }
        let mut i = x.hash();
        while self.t[i] != Item::Null && self.t[i] != Item::Del {
            i = if i + 1 == self.t.len() { 0 } else { i + 1 };
        }
        if self.t[i] == Item::Null {
            self.q += 1;
        }
        self.n += 1;
        self.t[i] = Item::Value(x);
        true
    }

    // 説明では返り値は bool と言っているのにコードでは T を返している？
    pub fn remove(&mut self, x: &T) -> bool {
        let mut i = x.hash();
        while self.t[i] != Item::Null {
            if let Item::Value(y) = &self.t[i] {
                if y == x {
                    self.t[i] = Item::Del;
                    self.n -= 1;
                    if 8 * self.n < self.t.len() {
                        self.resize();
                    }
                    return true;
                }
            }
            i = if i + 1 == self.t.len() { 0 } else { i + 1 };
        }
        false
    }

    fn resize(&mut self) {
        let mut d = 1;
        while 1 << d < 3 * self.n {
            d += 1;
        }
        let mut t_new = Vec::with_capacity(1 << d);
        t_new.resize_with(1 << d, || Item::Null);
        self.q = self.n;
        self.d = d;
        let t_old = mem::replace(&mut self.t, t_new);
        for item in t_old {
            if let Item::Value(x) = &item {
                let mut i = x.hash();
                while self.t[i] != Item::Null {
                    i = if i + 1 == self.t.len() { 0 } else { i + 1 };
                }
                self.t[i] = item;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::{Hashable, LinearHashTable};

    impl Hashable for i32 {}

    #[test]
    fn test_linear_hash_table_hand() {
        let mut h = LinearHashTable::new();
        assert_eq!(h.get(&0), None);
        assert_eq!(h.len(), 0);

        // insert 0
        assert_eq!(h.insert(0), true);
        assert_eq!(h.get(&0), Some(&0));
        assert_eq!(h.len(), 1);

        // insert 1
        assert_eq!(h.insert(1), true);
        assert_eq!(h.get(&1), Some(&1));
        assert_eq!(h.len(), 2);

        // remove 0
        assert_eq!(h.remove(&0), true);
        assert_eq!(h.get(&0), None);
        assert_eq!(h.len(), 1);

        // insert 1
        assert_eq!(h.insert(1), false);
        assert_eq!(h.get(&1), Some(&1));
        assert_eq!(h.len(), 1);

        // remove 0
        assert_eq!(h.remove(&0), false);
        assert_eq!(h.get(&0), None);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn test_linear_hash_table_large() {
        let mut h = LinearHashTable::new();
        for i in 0..100 {
            if i % 2 == 0 {
                h.insert(i);
            }
        }
        assert_eq!(h.len(), 50);
        for i in 0..100 {
            assert_eq!(h.get(&i).is_some(), i % 2 == 0);
        }
    }
}
