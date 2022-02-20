use env_logger::{Builder, WriteStyle};
use log::info;
use log::LevelFilter;
use std::env;
use std::io::Write;

fn main() {
    logger_init();
    let mut conn = kalgan_cache::Cache::new(env::var("REDIS_SERVER").unwrap().to_string());
    let key = "test";
    let value = "1";
    info!("Creating key...");
    conn.insert(key, value);
    info!("Checking key...");
    check_key(&mut conn, key);
    info!("Getting key value...");
    info!("{}={}", key, conn.get(key).unwrap());
    info!("Deleting key...");
    conn.delete(key);
    info!("Checking key again...");
    check_key(&mut conn, key);
    info!("Getting key value...");
    match conn.get(key) {
        Some(value) => info!("{}={}", key, value),
        None => info!("Key Not Found"),
    }
}
fn check_key(conn: &mut kalgan_cache::Cache, key: &str) {
    info!(
        "{}",
        if conn.exists(key) {
            "Key Found."
        } else {
            "Key Not Found."
        }
    );
}
fn logger_init() {
    match env::var_os("RUST_LOG") {
        Some(input_level_filter) => {
            let mut builder = Builder::new();
            builder
                .format(|buf, record| {
                    let mut level_style = buf.default_level_style(record.level());
                    level_style.set_bold(true);
                    writeln!(
                        buf,
                        "{}{}: {}",
                        fill_with_spaces(&record.level().to_string()),
                        level_style.value(record.level()),
                        record.args()
                    )
                })
                .filter(None, get_level_filter(input_level_filter.to_str().unwrap()))
                .write_style(WriteStyle::Always)
                .init();
        }
        None => {}
    };
}
fn fill_with_spaces(level: &String) -> String {
    const LENGTH: usize = 5;
    let mut level_length = level.len();
    let mut spaces = "".to_string();
    while level_length < LENGTH {
        spaces.push_str(" ");
        level_length = level_length + 1;
    }
    return spaces;
}
fn get_level_filter(input_level_filter: &str) -> LevelFilter {
    return match input_level_filter.to_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        _ => panic!("LevelFilter {} Not found.", input_level_filter),
    };
}
