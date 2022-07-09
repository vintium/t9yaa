#![allow(dead_code)]


use std::collections::HashMap;



type KeyboardKey = char;

#[derive(Debug)]
pub struct Layout {
    values: HashMap<KeyboardKey, Vec<char>>,
    keys: HashMap<char, KeyboardKey>
}

impl Layout {
    fn from(m: HashMap<KeyboardKey, Vec<char>>) -> Layout {
        let mut keys = HashMap::new();
        for (key, values) in &m {
            for value in values {
                keys.insert(*value, *key);
            }
        }
        Layout {
            values: m,
            keys: keys
        }
    }

    pub fn standard_t9() -> Layout {
        Layout::from(
            HashMap::from([
                ('2', vec!['a', 'b', 'c']),
                ('3', vec!['d', 'e', 'f']),
                ('4', vec!['g', 'h', 'i']),
                ('5', vec!['j', 'k', 'l']),
                ('6', vec!['m', 'n', 'o']),
                ('7', vec!['p', 'q', 'r', 's']),
                ('8', vec!['t', 'u', 'v']),
                ('9', vec!['w', 'x', 'y', 'z']),
            ])
        )
    }
    
    pub fn values_of(&self, k: KeyboardKey) -> Option<&Vec<char>> { self.values.get(&k) }
    pub fn key_for(&self, v: char) -> Option<KeyboardKey> { self.keys.get(&v).copied() }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
