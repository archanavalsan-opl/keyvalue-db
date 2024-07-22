use std::collections::HashMap;

struct KeyValueDB {
    store: HashMap<String, String>,
}

impl KeyValueDB {
    fn new() -> KeyValueDB {
        KeyValueDB {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }
}

fn main() {
    let mut db: KeyValueDB = KeyValueDB::new();

    db.insert("Ram".to_string(), "12".to_string());
    db.insert("Adil".to_string(), "28".to_string());
    db.insert("Ziya".to_string(), "16".to_string());

    match db.get("Ziya") {
        Some(value) => println!("Ziya: {}", value),
        None => println!("Ziya not found"),
    }

    db.delete("Ziya");

    match db.get("Ziya") {
        Some(value) => println!("Ziya: {}", value),
        None => println!("Ziya not found"),
    }
}