//extern crate time;
extern crate chrono;
extern crate bincode;
extern crate hex;
extern crate byteorder;

use chrono::{ Utc };
use std::net::{TcpStream};
use std::io::{Write, Read};

mod util;
use util::crypto::{CryptoUtils};

mod serializer;
use serializer::{Serializer, ByteOrder};

fn main() {

    let version : Serializer = Serializer::create_named("Version")
        .fluent_i32_push(70015, ByteOrder::LittleEndian)
        .fluent_u64_push(0, ByteOrder::LittleEndian)
        .fluent_i64_push(Utc::now().timestamp() as i64, ByteOrder::LittleEndian)

        //recv
        .fluent_u64_push(0, ByteOrder::LittleEndian)
        .fluent_buffer_push(&vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x01 ])
        .fluent_u16_push(8333, ByteOrder::NetworkEndian)

        //trans
        .fluent_u64_push(0, ByteOrder::LittleEndian)
        .fluent_buffer_push(&vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x00 ])
        .fluent_u16_push(8333, ByteOrder::NetworkEndian)

        .fluent_buffer_push(&vec![0x3B, 0x2E, 0xB3, 0x5D, 0x8C, 0xE6, 0x17, 0x65]) //nounce
        .fluent_u64_push(0, ByteOrder::LittleEndian)
        .fluent_i32_push(507360, ByteOrder::LittleEndian)
        .fluent_byte_push(0)
        .build();

    version.dump();

    println!("");
    //raw_version.extend_from_slice(b"::ffff:127.0.0.1");
    //raw_version.extend_from_slice(b"107.191.33.83\0\0\0");

    let checksum = CryptoUtils::sha256(&CryptoUtils::sha256(&version.get_stream()));

    let message : Serializer = Serializer::create_named("Message")
        .fluent_u32_push(0xD9B4BEF9 as u32, ByteOrder::LittleEndian)
        .fluent_static_buffer_push(b"version\0\0\0\0\0")
        .fluent_u32_push(version.get_stream().len() as u32, ByteOrder::LittleEndian)
        .fluent_byte_push(checksum[3])
        .fluent_byte_push(checksum[2])
        .fluent_byte_push(checksum[1])
        .fluent_byte_push(checksum[0])
        .fluent_buffer_push(&version.get_stream())
        .build();

    message.dump();

    /*
    {
        let mut stream = TcpStream::connect("52.14.94.187:8333").unwrap();
        let _ = stream.write(&message.get_stream());
        let _ = stream.read(&mut [0; 128]); // ignore here too\
    }
    */
    // the stream is closed here
}
