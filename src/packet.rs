use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub no_delay: bool,
    pub data: Vec<u8>,
}