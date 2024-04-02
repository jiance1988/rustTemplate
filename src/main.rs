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


#[cfg(feature = "modbus")]
#[derive(Debug, Serialize,Deserialize)]
struct SqlError {
    a: i32,
    b: i32,
    c: i32,
}

