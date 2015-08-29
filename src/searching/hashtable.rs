//! Algorithm 3.6 Hashing with linear probing

use std::hash::{Hash, Hasher, SipHasher};
use std::mem;

/// A symbol table implementation based on hashing with linear probing.
pub struct SymbolTable<K: Eq + Hash + Clone, V: Clone> {
    /// number of key-value pairs in the table
    n: usize,
    /// size of linear-probing table
    m: usize,
    /// symbol table entries
    entries: Vec<Option<(K, V)>>,
}

impl<K: Eq + Hash + Clone, V: Clone> SymbolTable<K, V> {
    /// Constructs a new, empty symbol table.
    pub fn new() -> SymbolTable<K, V> {
        let initial_capacity = 16;
        SymbolTable {
            n: 0,
            m: initial_capacity,
            entries: vec![None; initial_capacity],
        }
    }

    /// Adds a key-value pair to the symbol table. If there is already an
    /// element with the same key, its value is updated.
    pub fn put(&mut self, key: K, val: V) {
        if self.n >= self.m/2 {
            let new_m = 2 * self.m;
            self.resize(new_m);
        }
        let (i, present) = self.position(&key);
        self.entries[i] = Some((key, val));
        if !present { self.n += 1; }
    }

    // Looks up `key` in the table. If it is present, returns its index and
    // `true`, if not, returns the index where it can be inserted and `false`.
    fn position(&self, key: &K) -> (usize, bool) {
        let mut i = self.hash(&key);
        loop {
            match self.entries[i] {
                Some((ref k, _)) => {
                    if k == key {
                        return (i, true);
                    } else {
                        i = (i + 1) % self.m;
                    }
                },
                _ => {
                    return (i, false);
                },
            }
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = SipHasher::new();
        key.hash::<SipHasher>(&mut hasher);
        (hasher.finish() % (self.m as u64)) as usize
    }

    fn resize(&mut self, new_m: usize) {
        let mut old_entries = vec![None; new_m];
        mem::swap(&mut self.entries, &mut old_entries);
        self.m = new_m;
        self.n = 0;

        for entry in old_entries.iter_mut() {
            if entry.is_some() {
                let mut temp = None;
                mem::swap(entry, &mut temp);
                let (key, val) = temp.unwrap();
                self.put(key, val);
            }
        }
    }

    /// Gets the value associated with the given key, or `None`.
    pub fn get(&self, key: &K) -> Option<V> {
        let (i, _) = self.position(&key);
        if let Some(ref pair) = self.entries[i] {
            Some(pair.1.clone())
        } else {
            None
        }
    }

    /// Removes the entry with the given key.
    pub fn delete(&mut self, key: &K) {
        let (mut i, present) = self.position(&key);
        if !present { return; }

        // remove entry
        self.entries[i] = None;

        // re-insert the entries in the cluster to the right of the deleted one
        loop {
            i = (i + 1) % self.m;
            if self.entries[i].is_none() {
                break;
            }
            let mut temp = None;
            mem::swap(&mut self.entries[i], &mut temp);
            self.n -= 1;
            let (key, val) = temp.unwrap();
            self.put(key, val);
        }

        // decrement size and shrink `entries` if necessary
        self.n -= 1;
        if self.n > 0 && self.n == self.m/8 {
            let new_m = self.m / 2;
            self.resize(new_m);
        }
    }

    /// True if the symbol table contains the given key.
    pub fn contains(&self, key: &K) -> bool {
        self.position(&key).1
    }

    /// True if the symbol table is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of entries in the symbol table.
    pub fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod test {
    use super::SymbolTable;

    fn empty_table() -> SymbolTable<String, isize> {
        SymbolTable::new()
    }

    fn table_5_elements() -> SymbolTable<String, isize> {
        let mut pt: SymbolTable<String, isize> = SymbolTable::new();
        pt.put("Fe".to_string(), 26);
        pt.put("Cr".to_string(), 24);
        pt.put("Si".to_string(), 14);
        pt.put("Ag".to_string(), 47);
        pt.put("Cu".to_string(), 29);
        pt
    }

    #[test]
    fn symbol_table_with_0_elements() {
        let pt = empty_table();
        assert!(pt.get(&"Fe".to_string()).is_none());
        assert!( ! pt.contains(&"Fe".to_string()));
        assert!(pt.is_empty());
        assert_eq!(0, pt.size());
    }

    #[test]
    fn symbol_table_with_1_element() {
        let mut pt = empty_table();
        pt.put("Fe".to_string(), 26);
        assert_eq!(Some(26), pt.get(&"Fe".to_string()));
        assert!(pt.get(&"Cr".to_string()).is_none());
        assert!(pt.contains(&"Fe".to_string()));
        assert!( ! pt.contains(&"Pt".to_string()));
        assert!( ! pt.is_empty());
        assert_eq!(1, pt.size());
    }

    #[test]
    fn symbol_table_with_5_elements() {
        let pt = table_5_elements();
        assert_eq!(Some(47), pt.get(&"Ag".to_string()));
        assert!(pt.get(&"Pb".to_string()).is_none());
        assert!(pt.contains(&"Ag".to_string()));
        assert!( ! pt.contains(&"Pt".to_string()));
        assert!( ! pt.is_empty());
        assert_eq!(5, pt.size());
    }

    #[test]
    fn deleting_entries() {
        let mut pt = table_5_elements();
        pt.delete(&"Ag".to_string());
        assert!( ! pt.contains(&"Ag".to_string()));
        assert_eq!(4, pt.size());
        for element in vec!["Cr", "Cu", "Fe", "Si"] {
            pt.delete(&element.to_string());
        }
        assert!(pt.is_empty());
    }

    #[test]
    fn changing_entry() {
        let mut pt = table_5_elements();
        pt.put("Fe".to_string(), 12345);
        assert_eq!(Some(12345), pt.get(&"Fe".to_string()));
        pt.delete(&"Fe".to_string());
        assert!( ! pt.contains(&"Fe".to_string()));
        pt.put("Fe".to_string(), 54321);
        assert_eq!(Some(54321), pt.get(&"Fe".to_string()));
    }
}
