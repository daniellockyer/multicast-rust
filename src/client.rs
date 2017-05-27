extern crate net2;

use net2::UdpBuilder;
use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("Usage: {} group_ip port", args[0]);
        return;
    }

    let mc_ip = Ipv4Addr::from_str(&args[1]).expect("cannot parse group_ip");
    let mc_port = args[2].parse::<u16>().expect("cannot parse port");
    let any = Ipv4Addr::from([0, 0, 0, 0]);

    let socket = UdpBuilder::new_v4()
        .expect("cannot create UDP socket")
        .reuse_address(true).expect("cannot set reuse address")
        .bind((any, mc_port)).expect("cannot bind");

    socket.join_multicast_v4(&mc_ip, &any).expect("cannot join");
    let mut buffer = [0u8; 1600];

    loop {
        let (size, addr) = socket.recv_from(&mut buffer).expect("cannot recv");
        println!("({} bytes from {}) {:?}", size, addr, &buffer[..size]);
    }
}
