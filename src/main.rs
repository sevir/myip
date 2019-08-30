fn main() {
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        println!("{}: {}", iface.name, iface.ip());
    }
    println!("Public IP: {}", my_public_ip::resolve().unwrap());
}
