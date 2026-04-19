#![no_std]


mod domain;

#[cfg(test)]
mod test {
    use domain;

    use crate::domain::kv_store::{KVStore, Database, Key, Value};

    #[test]
    fn test_insert() {
        let mut db = Database::new();
        let key = Key::parse("key").unwrap();
        let value = Value::parse("value").unwrap();

        let _ = db.set(key.clone(), value.clone());

        assert_eq!(db.get(&key).unwrap(), value);
    }
}