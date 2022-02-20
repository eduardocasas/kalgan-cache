# kalgan-cache

A wrapper for redis crate used by Kalgan Framework.


## Examples

```rust
use kalgan_cache::Cache;
let cache: Cache = Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
```
```rust
cache.insert("key", "value");
```
```rust
cache.delete("key");
```
```rust
cache.exists("key");
```
```rust
let key: String = cache.get("key").unwrap();
```
    
## Documentation

For further information please visit:

* [Official Kalgan Site](https://kalgan.eduardocasas.com)
* [API Documentation on docs.rs](https://docs.rs/crate/kalgan-cache/latest)


## License

This crate is licensed under either of the following licenses:

* [MIT License](https://choosealicense.com/licenses/mit/)
* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
