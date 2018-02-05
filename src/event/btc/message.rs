struct Version {
    version: i32,
    services: u64,
    timestamp: i64,
    addr_recv_services: u64,
    addr_recv_ip_address: [u8; 16],
    add_recv_port: u16,
    addr_trans_services: u64,
    addr_trans_ip_address: [u8; 16],
    addr_trans_port: u16,
    nonce: u64,
    user_agent_bytes: u64,
    user_agent: vec![], //variable size--not needed if previous field 0
    start_height: i32,
    relay: bool
}

struct Message {
    magic: u32,
    command: [u8; 12],
    length: u32,
    checksum: u32,
    payload: vec![] // Ex: Version
}
