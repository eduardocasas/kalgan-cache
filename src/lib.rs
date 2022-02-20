//! A wrapper for `redis` crate.

pub extern crate redis;
use log::{debug, error, warn};
use redis::{Client, Commands, Connection, ToRedisArgs};

/// The object that keeps the Redis connection.
pub struct Cache {
    connection: Connection,
}
impl Cache {
    /// Creates and return the `Cache` instance.
    /// # Examples:
    /// ```
    /// use kalgan_cache::Cache;
    /// # use std::env;
    /// let cache: Cache = Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
    /// ```
    pub fn new(redis_address: String) -> Cache {
        Cache {
            connection: Cache::get_connection(redis_address),
        }
    }
    /// Inserts a record in Redis database with the given arguments key-value.
    /// # Examples:
    /// ```
    /// # use kalgan_cache::Cache;
    /// # use std::env;
    /// # let mut cache: Cache = Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
    /// cache.insert("key", "value");
    /// # assert_eq!(cache.get("key").unwrap(), "value");
    /// ```
    pub fn insert<V: ToRedisArgs>(&mut self, key: &str, value: V) {
        match self.connection.set(key, value) {
            Ok(()) => debug!("Key '{}' added to Redis.", key),
            Err(e) => error!("{}", e.category().to_string()),
        }
    }
    /// Deletes a record in Redis database by key.
    /// # Examples:
    /// ```
    /// # use kalgan_cache::Cache;
    /// # use std::env;
    /// # let mut cache: Cache = Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
    /// # cache.insert("key", "value");
    /// cache.delete("key");
    /// # assert_eq!(cache.exists("key"), false);
    /// ```
    pub fn delete(&mut self, key: &str) {
        match self.connection.del(key) {
            Ok(()) => debug!("Key '{}' delete from Redis.", key),
            Err(e) => error!("{}", e.category().to_string()),
        }
    }
    /// Checks if a record in Redis database exists.
    /// # Examples:
    /// ```
    /// # use kalgan_cache::Cache;
    /// # use std::env;
    /// # let mut cache: Cache = Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
    /// # cache.insert("key", "value");
    /// cache.exists("key");
    /// # assert!(cache.exists("key"));
    /// ```
    pub fn exists(&mut self, key: &str) -> bool {
        let result: Option<String> = match self.connection.get(&key) {
            Ok(value) => value,
            Err(_e) => None,
        };
        match result {
            Some(_s) => true,
            None => false,
        }
    }
    /// Gets a record in Redis database by key.
    /// # Examples:
    /// ```
    /// # use kalgan_cache::Cache;
    /// # use std::env;
    /// # let mut cache: Cache = Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
    /// # cache.insert("key", "value");
    /// let key: String = cache.get("key").unwrap();
    /// # assert_eq!(key, "value".to_string());
    /// ```
    pub fn get(&mut self, key: &str) -> Option<String> {
        match self.connection.get(&key) {
            Ok(value) => value,
            Err(e) => {
                warn!("{}", e);
                warn!("Key {} Not found in Cache.", key);
                None
            }
        }
    }
    /// Returns Redis connection.
    fn get_connection(redis_address: String) -> Connection {
        match Client::open(redis_address) {
            Ok(client) => match client.get_connection() {
                Ok(conection) => conection,
                Err(e) => {
                    error!("Could not connect with Redis Service.");
                    panic!("{}", e);
                }
            },
            Err(e) => {
                error!("Could not open Redis Service.");
                panic!("{}", e);
            }
        }
    }
}
