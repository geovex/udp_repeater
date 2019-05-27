extern crate clap;
extern crate net2;

use clap::{App, Arg};
use net2::UdpSocketExt;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};


fn main() {
    let app = App::new("udp_repeater")
        .arg(Arg::with_name("src").required(true))
        .arg(Arg::with_name("dst").required(true).min_values(1));
    let matches = app.get_matches();
    let receiver = UdpSocket::bind(matches.value_of("src").unwrap()).unwrap();
    receiver.set_recv_buffer_size(1024 * 1024).ok();
    let mut senders: Vec<SocketAddr> = Vec::new();
    for dst in matches.values_of("dst").unwrap() {
        for dstaddr in dst.to_socket_addrs().unwrap_or(Vec::new().into_iter()) {
            senders.push(dstaddr);
        }
    }
    let mut tmp_buffer = [0u8; 65_536];
    loop {
        match receiver.recv(&mut tmp_buffer) {
            Ok(size) => {
                for sender in &senders {
                    receiver.send_to(&tmp_buffer[0..size], sender).ok();
                    //println!("sended")
                }
            }
            e @ Err(_) => eprintln!("recv error {:?}", e),
        }
    }
}
