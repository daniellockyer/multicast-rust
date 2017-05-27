extern crate net2;

use net2::UdpBuilder;
use std::net::Ipv4Addr;
use std::{thread, time};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("Usage: {} group_ip port", args[0]);
        return;
    }

    let socket = UdpBuilder::new_v4()
        .expect("cannot create UDP socket")
        .reuse_address(true).expect("cannot set reuse address")
        .bind((Ipv4Addr::from([0, 0, 0, 0]), 0)).expect("cannot bind");

	let addr = format!("{}:{}", &args[1], &args[2]);
	let two_secs = time::Duration::new(2, 0);
    let buffer = [3u8; 1600];

    loop {
        let size = socket.send_to(&buffer, &addr).expect("cannot send");
        println!("Sent {} bytes on multicast", size);

		thread::sleep(two_secs);
    }
}
