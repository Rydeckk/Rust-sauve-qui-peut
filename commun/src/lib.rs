pub use {
    serde::{Deserialize, Serialize},
    std::io::{self, Read, Write},
    std::net::{TcpListener, TcpStream},
    std::sync::{Arc, Mutex},
    std::thread,
    std::collections::HashMap,
    rand::{thread_rng, Rng},
    rand::distributions::Alphanumeric,
    std::ops::RangeInclusive,
    serde_json
};

pub mod structs;
pub mod utils;
pub mod decodage;
pub mod encodage;