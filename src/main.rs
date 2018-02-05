//extern crate time;
extern crate chrono;
extern crate bincode;
extern crate hex;
extern crate byteorder;

use chrono::{ Utc };
//use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
//use std::io::{Cursor, Seek, Write};

mod util;
use util::crypto::{CryptoUtils};

//use byteorder::{BigEndian, LittleEndian, NativeEndian, NetworkEndian, ReadBytesExt, WriteBytesExt};

mod serializer;
use serializer::{Serializer, ByteOrder};

fn main() {
    println!("Bitcoin play!");

    let mut serializer : Serializer = Serializer::new();
    serializer = serializer.push_i32(70015, ByteOrder::LittleEndian);
    serializer = serializer.push_byte(1);
    serializer = serializer.push(&serializer.get_stream());
    serializer.print();

    serializer.push_byte_mut(1);
    serializer.push_byte_mut(0);
    serializer.push_byte_mut(1);
    serializer.push_byte_mut(0);
    serializer.print();


    serializer = serializer.push_byte_fluent(2)
        .push_byte_fluent(2)
        .push_byte_fluent(2)
        .build();
    serializer.print();
    //let mut buffer = Cursor::new(vec![]);


    /*
    let mut test = vec![];
    test.extend_from_slice(&serialize_i32(70015, ByteOrder::LittleEndian));
    test.extend_from_slice(&serialize_i32(70015, ByteOrder::BigEndian));

    println!("{}", to_hex_string(&test));

    let mut raw_version = vec![];
    raw_version.write_i32::<LittleEndian>(70015).unwrap();
    raw_version.write_u64::<LittleEndian>(0).unwrap();
    raw_version.write_i64::<LittleEndian>(Utc::now().timestamp() as i64).unwrap();
    raw_version.write_u64::<LittleEndian>(0).unwrap();
    raw_version.extend_from_slice(&vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x01 ]);
    //raw_version.extend_from_slice(b"::ffff:127.0.0.1");
    raw_version.write_u16::<NetworkEndian>(8333).unwrap();
    raw_version.write_u64::<BigEndian>(0).unwrap();

    //raw_version.extend_from_slice(b"107.191.33.83\0\0\0");
    raw_version.extend_from_slice(&vec![ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00 ]);

    raw_version.write_u16::<NetworkEndian>(8333).unwrap();
    //raw_version.write_u64::<LittleEndian>(23445).unwrap();
    raw_version.extend_from_slice(&vec![0x3B, 0x2E, 0xB3, 0x5D, 0x8C, 0xE6, 0x17, 0x65]);

    raw_version.write_u64::<LittleEndian>(0).unwrap();
    raw_version.write_i32::<LittleEndian>(507360).unwrap();
    raw_version.push(0);

    println!("raw_version hex content:");
    println!("{}", to_hex_string(&raw_version));

/*
    let version : Version = Version {
        version: 70015,
        services: 0x00,
        timestamp: Utc::now().timestamp() as i64,
        addr_recv_services: 0,
        addr_recv_ip_address: *b"::ffff:127.0.0.1",
        add_recv_port: 8333,
        addr_trans_services: 0,
        addr_trans_ip_address: *b"107.191.33.83\0\0\0",
        addr_trans_port: 8333,
        nonce: 0, //random
        user_agent_bytes: 0,
        start_height: 507360,
        relay: false
    };
*/
    println!("");println!("");println!("");println!("");println!("");

    let checksum = crypto_utils::CryptoUtils::sha256(&crypto_utils::CryptoUtils::sha256(&raw_version));

/*
    let message : Message = Message {
        magic: 0xD9B4BEF9 as u32,
        command: *b"version\0\0\0\0\0",
        length: mem::size_of::<Version>() as u32,
        checksum: 34, //u32
        payload: version
    };
*/
    println!("raw_version length is {}", raw_version.len());
    let mut raw_message = vec![];
    raw_message.write_u32::<LittleEndian>(0xD9B4BEF9 as u32).unwrap();
    raw_message.extend_from_slice(b"version\0\0\0\0\0");
    raw_message.write_u32::<LittleEndian>(raw_version.len() as u32).unwrap();
    raw_message.push(checksum[3]);
    raw_message.push(checksum[2]);
    raw_message.push(checksum[1]);
    raw_message.push(checksum[0]);
    raw_message.extend_from_slice(&raw_version);

    println!("raw_message hex content:");
    println!("{}", to_hex_string(&raw_message));

/*
F9 BE B4 D9
76 65 72 73 69 6F 6E 00 00 00 00 00
00 00 00 5D (X)
09 F8 B3 8F





00 01 11 7F (X)
00 00 00 00 00 00 00 01 (X)

00 00 00 00 5A 76 2E 36 (X)

00 00 00 00 00 00 00 00 3A 3A 66 66 66 66 3A 31 32 37 2E 30 2E 30 2E 31 20 8D
00 00 00 00 00 00 00 00 31 30 37 2E 31 39 31 2E 33 33 2E 38 33 00 00 00 20 8D

00 00 00 00 00 00 09 29

00 00 00 00 00 00 00 00

00 07 BD E0 (X)

00
*/
        let test_messsage = vec![
             // magic number
             0xF9, 0xBE, 0xB4, 0xD9,
             // command
             0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00,
             0x00, 0x00,
             // message length
             0x65, 0x00, 0x00, 0x00,
             // checksum
             0x03, 0x0E, 0xCC, 0x57,

             // version
             0x62, 0xEA, 0x00, 0x00,
             // services
             0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             // timestamp
             0x11, 0xB2, 0xD0, 0x50, 0x00, 0x00, 0x00, 0x00,



             // addr recv
             0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //service

             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, //address

             0x00, 0x00, //port



             // addr from
             0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             // nonce
             0x3B, 0x2E, 0xB3, 0x5D, 0x8C, 0xE6, 0x17, 0x65,
             // user-agent
             0x0F, 0x2F, 0x53, 0x61, 0x74, 0x6F, 0x73, 0x68, 0x69, 0x3A,
             0x30, 0x2E, 0x37, 0x2E, 0x32, 0x2F,
             // last block height
             0xC0, 0x3E, 0x03, 0x00,
             // relay
    0x00];

    {
        //let listener = TcpListener::bind("127.0.0.1:8333").unwrap();
        //let mut stream = TcpStream::connect("52.14.94.187:8333").unwrap();
        //let _ = stream.write(&raw_message);
        //let _ = stream.read(&mut [0; 128]); // ignore here too\
    } // the stream is closed here
*/
}
