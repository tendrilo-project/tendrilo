extern crate bincode;
extern crate hex;
extern crate byteorder;
use byteorder::{BigEndian, LittleEndian, NativeEndian, NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::string::{String};

pub enum ByteOrder {
    BigEndian,
    LittleEndian,
    NativeEndian,
    NetworkEndian
}

pub struct Serializer {
    name: &'static str,
    data: Vec<u8>
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
  let strs: Vec<String> = (*bytes).iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join(" ")
}

impl Serializer {

    pub fn create() -> Serializer {
        Serializer {
            name: "Serializer",
            data: vec![]
        }
    }

    pub fn create_named(name: &'static str) -> Serializer {
        Serializer {
            name: name,
            data: vec![]
        }
    }

    // Mutable operations
    pub fn push_byte_mut(&mut self, buffer: u8) {
        self.data.push(buffer);
    }

    // Fluent operations

    pub fn fluent_byte_push(mut self, buffer: u8) -> Serializer {
        self.data.push(buffer);
        self
    }

    pub fn fluent_buffer_push(mut self, buffer: &Vec<u8>) -> Serializer {
        self.data.extend_from_slice(buffer);
        self
    }

    pub fn fluent_static_buffer_push(mut self, buffer: &'static [u8]) -> Serializer {
        self.data.extend_from_slice(buffer);
        self
    }

    pub fn fluent_u16_push(mut self, int: u16, byteorder: ByteOrder) -> Serializer {
        match byteorder {
             ByteOrder::LittleEndian => self.data.write_u16::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => self.data.write_u16::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => self.data.write_u16::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => self.data.write_u16::<NetworkEndian>(int).unwrap()
        }
        self
    }

    pub fn fluent_i32_push(mut self, int: i32, byteorder: ByteOrder) -> Serializer {
        match byteorder {
             ByteOrder::LittleEndian => self.data.write_i32::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => self.data.write_i32::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => self.data.write_i32::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => self.data.write_i32::<NetworkEndian>(int).unwrap()
        }
        self
    }

    pub fn fluent_u32_push(mut self, int: u32, byteorder: ByteOrder) -> Serializer {
        match byteorder {
             ByteOrder::LittleEndian => self.data.write_u32::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => self.data.write_u32::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => self.data.write_u32::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => self.data.write_u32::<NetworkEndian>(int).unwrap()
        }
        self
    }

    pub fn fluent_u64_push(mut self, int: u64, byteorder: ByteOrder) -> Serializer {
        match byteorder {
             ByteOrder::LittleEndian => self.data.write_u64::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => self.data.write_u64::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => self.data.write_u64::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => self.data.write_u64::<NetworkEndian>(int).unwrap()
        }
        self
    }

    pub fn fluent_i64_push(mut self, int: i64, byteorder: ByteOrder) -> Serializer {
        match byteorder {
             ByteOrder::LittleEndian => self.data.write_i64::<LittleEndian>(int).unwrap(),
             ByteOrder::BigEndian => self.data.write_i64::<BigEndian>(int).unwrap(),
             ByteOrder::NativeEndian => self.data.write_i64::<NativeEndian>(int).unwrap(),
             ByteOrder::NetworkEndian => self.data.write_i64::<NetworkEndian>(int).unwrap()
        }
        self
    }

    pub fn build(&self) -> Serializer {
        Serializer {
            name: self.name,
            data: self.data.clone()
        }
    }

    pub fn dump(&self) {
        println!("{} hex dump:\n{}", self.name, to_hex_string(&self.data));
    }

    pub fn get_stream(&self) -> Vec<u8> {
        self.data.clone()
    }
}
