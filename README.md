# Hash Table Implemented in Rust

```rust
let mut table = Table::new();

table.set(&Key::new("key1"), &Value::Boolean(false));
table.set(&Key::new("key2"), &Value::Number(10.0));
table.set(&Key::new("key3"), &Value::String("hi".to_string()));

assert!(table.get(&Key::new("key1")).unwrap() == Value::Boolean(false));
assert!(table.get(&Key::new("key2")).unwrap() == Value::Number(10.0));
assert!(table.get(&Key::new("key3")).unwrap() == Value::String("hi".to_string()));
assert!(table.get(&Key::new("key4")).is_none());

table.delete(&Key::new("key1"));
assert!(table.get(&Key::new("key1")).is_none());

table.delete(&Key::new("key4"));
assert!(table.get(&Key::new("key4")).is_none());
```
