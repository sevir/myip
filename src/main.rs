extern crate my_internet_ip;

fn main() {
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        println!("{}: {}", iface.name, iface.ip());
    }

    let ip: ::std::net::IpAddr = match my_internet_ip::get() {
        Ok(ip) => ip,
        Err(e) => panic!("Could not get IP: {:?}", e),
    };
    println!("Public IP: {}", ip);
}
