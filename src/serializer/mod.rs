extern crate bincode;
extern crate hex;
extern crate byteorder;

use std::io::{Cursor, Seek, Write};
use byteorder::{BigEndian, LittleEndian, NativeEndian, NetworkEndian, ReadBytesExt, WriteBytesExt};

pub enum ByteOrder {
    BigEndian,
    LittleEndian,
    NativeEndian,
    NetworkEndian
}

pub struct Serializer {
    data: Vec<u8>
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
  let strs: Vec<String> = (*bytes).iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join(" ")
}

impl Serializer {
    pub fn new() -> Serializer {
        Serializer {
            data: vec![]
        }
    }

    pub fn push_byte(&self, buffer: u8) -> Serializer {
        let mut new_data = self.data.clone();
        new_data.push(buffer);
        Serializer {
            data: new_data
        }
    }

    pub fn push_byte_mut(&mut self, buffer: u8) {
        self.data.push(buffer);
    }

    pub fn push_byte_fluent(mut self, buffer: u8) -> Serializer {
        self.data.push(buffer);
        self
    }

    pub fn build(&self) -> Serializer {
        Serializer {
            data: self.data.clone()
        }
    }

    pub fn push(&self, buffer: &Vec<u8>) -> Serializer {
        let mut new_data = self.data.clone();
        new_data.extend_from_slice(buffer);
        Serializer {
            data: new_data
        }
    }

    pub fn push_i32(&self, int: i32, byteorder: ByteOrder) -> Serializer {
        let mut new_data = self.data.clone();
        match byteorder {
             ByteOrder::LittleEndian => new_data.write_i32::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => new_data.write_i32::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => new_data.write_i32::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => new_data.write_i32::<NetworkEndian>(int).unwrap()
        }
        Serializer {
            data: new_data
        }
    }

    pub fn fluent_push_i32(&self, int: i32, byteorder: ByteOrder) -> Serializer {
        let mut new_data = self.data.clone();
        match byteorder {
             ByteOrder::LittleEndian => new_data.write_i32::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => new_data.write_i32::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => new_data.write_i32::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => new_data.write_i32::<NetworkEndian>(int).unwrap()
        }
        Serializer {
            data: new_data
        }
    }

    pub fn print(&self) {
        println!("{}", to_hex_string(&self.data));
    }

    pub fn get_stream(&self) -> Vec<u8> {
        self.data.clone()
    }
}
