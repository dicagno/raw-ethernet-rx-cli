use pcap::{Capture, Device};
use pnet::packet::ethernet::EthernetPacket;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <interface> <ethertype>", args[0]);
        return;
    }
    let interface_name = &args[1];
    let ethertype_str = &args[2];

    let custom_ethertype = u16::from_str_radix(ethertype_str.trim_start_matches("0x"), 16).expect("Invalid Ethertype, please provide it in HEX notation");

    let device = Device::list()
        .unwrap()
        .into_iter()
        .find(|dev| dev.name == *interface_name)
        .unwrap();

    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();

    println!("Listening on interface: {}, filtering for Ethertype: 0x{:04x}", interface_name, custom_ethertype);

    while let Ok(packet) = cap.next() {
        if let Some(ethernet) = EthernetPacket::new(packet.data) {
            if ethernet.get_ethertype().0 == custom_ethertype {
                println!("Captured packet with Ethertype 0x{:04x}: {:?}", custom_ethertype, ethernet);
            }
        }
    }
}
