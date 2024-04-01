use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
    println!("sql error: {:?}", SqlError {
        #[cfg(feature = "modbus")]
        a: 1,
        #[cfg(feature = "modbus")]
        b: 2,
        #[cfg(feature = "modbus")]
        c: 3,
    });
}

#[derive(Debug, Serialize, Deserialize)]
struct SqlError {
    #[cfg(feature = "modbus")]
    a: i32,
    #[cfg(feature = "modbus")]
    b: i32,
    #[cfg(feature = "modbus")]
    c: i32,
}