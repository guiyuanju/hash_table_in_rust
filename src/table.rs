use std::fmt;

#[derive(Debug)]
pub struct Table {
    pub count: usize,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub enum Entry {
    KeyValue(KeyValue),
    Tombstone,
    None,
}

#[derive(Debug, Clone)]
pub struct KeyValue {
    key: Key,
    value: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Key {
    name: String,
    hash: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Key {
    pub fn new(name: &str) -> Self {
        let mut hash: u32 = 2166136261u32;
        for &b in name.as_bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(16777619);
        }

        Self {
            name: name.to_string(),
            hash,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{:}", b),
            Self::Number(n) => write!(f, "{:}", n),
            Self::String(s) => write!(f, "{:}", s),
        }
    }
}

enum FindResult {
    Found(usize),
    Available(usize),
    Tombstone(usize),
}

fn find_entry(entries: &Vec<Entry>, key: &Key) -> FindResult {
    let mut index = (key.hash % entries.len() as u32) as usize;
    let mut tombstone: Option<FindResult> = None;

    loop {
        match &entries[index] {
            Entry::KeyValue(kv) => {
                if kv.key == *key {
                    return FindResult::Found(index);
                }
            }
            Entry::Tombstone => {
                if tombstone.is_none() {
                    tombstone = Some(FindResult::Tombstone(index))
                }
            }
            Entry::None => {
                if tombstone.is_some() {
                    return tombstone.unwrap();
                }
                return FindResult::Available(index);
            }
        }

        index = (index + 1) % entries.len();
    }
}

impl Table {
    const MAX_LOAD: f32 = 0.75;

    pub fn new() -> Self {
        Table {
            count: 0,
            entries: vec![],
        }
    }

    fn adjust_capacity(&mut self, capacity: usize) {
        let mut entries: Vec<Entry> = vec![Entry::None; capacity];

        self.count = 0;
        for e in &self.entries {
            if let Entry::KeyValue(entry) = e {
                if let FindResult::Available(index) = find_entry(&mut entries, &entry.key) {
                    entries[index] = Entry::KeyValue(entry.clone());
                    self.count += 1
                }
            }
        }

        self.entries = entries;
    }

    pub fn get(&self, key: &Key) -> Option<Value> {
        if self.entries.len() == 0 {
            return None;
        }

        if let FindResult::Found(entry_index) = find_entry(&self.entries, &key) {
            if let Entry::KeyValue(kv) = &self.entries[entry_index] {
                return Some(kv.value.clone());
            }
        }

        return None;
    }

    pub fn set(&mut self, key: &Key, value: &Value) {
        if (self.count + 1) as f32 > Self::MAX_LOAD * self.entries.len() as f32 {
            let capacity = if self.entries.len() < 8 {
                8
            } else {
                self.entries.len() * 2
            };

            self.adjust_capacity(capacity);
        }

        let index = match find_entry(&self.entries, &key) {
            FindResult::Found(index) => index,
            FindResult::Available(index) => {
                self.count += 1;
                index
            }
            FindResult::Tombstone(index) => index,
        };
        self.entries[index] = Entry::KeyValue(KeyValue {
            key: key.clone(),
            value: value.clone(),
        });
    }

    pub fn delete(&mut self, key: &Key) {
        if self.count == 0 {
            return;
        }

        if let FindResult::Found(index) = find_entry(&self.entries, &key) {
            self.entries[index] = Entry::Tombstone;
        }
    }

    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    pub fn load_factor(&self) -> f32 {
        self.count as f32 / self.entries.len() as f32
    }

    pub fn visualize(&self) {
        for e in &self.entries {
            if let Entry::KeyValue(entry) = e {
                print!("[{:?}: {:}] ", entry.key.name, entry.value)
            } else {
                print!("[ ] ")
            }
        }
        println!()
    }
}
