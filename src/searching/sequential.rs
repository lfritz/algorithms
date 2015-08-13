//! Algorithm 3.1 Sequential search (in an array).
//! The algorithm in the book uses a linked list, but that's more difficult in
//! Rust.

/// A symbol table implementation based on an unordered array.
pub struct SymbolTable<K: PartialEq, V: Clone> {
    vec: Vec<(K, V)>,
}

impl<K: PartialEq, V: Clone> SymbolTable<K, V> {
    /// Constructs a new, empty symbol table.
    pub fn new() -> SymbolTable<K, V> {
        SymbolTable { vec: Vec::new() }
    }

    /// Adds a key-value pair to the symbol table. If there is already an
    /// element with the same key, its value is updated.
    pub fn put(&mut self, key: K, value: V) {
        for ref mut pair in self.vec.iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        self.vec.push((key, value));
    }

    /// Gets the value associated with the given key, or `None`.
    pub fn get(&self, key: &K) -> Option<V> {
        for &(ref k, ref v) in &self.vec {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }

    /// Removes the entry with the given key.
    pub fn delete(&mut self, key: &K) {
        let mut i = 0;
        for &(ref k, _) in &self.vec {
            if k == key {
                break;
            }
            i += 1;
        }
        self.vec.remove(i);
    }

    /// True if the symbol table contains the given key.
    pub fn contains(&self, key: &K) -> bool {
        for &(ref k, _) in &self.vec {
            if k == key {
                return true
            }
        }
        false
    }

    /// True if the symbol table is empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns the number of entries in the symbol table.
    pub fn size(&self) -> usize {
        self.vec.len()
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
