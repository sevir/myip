use local_ip_address::list_afinet_netifas;
use serde::Deserialize;


#[derive(Deserialize)]
struct Ipinfo {
    ip: String,
    // city: String,
    // region: String,
    // country: String,
    // loc: String,
    // org: String,
    // typezone: String
}


fn main() -> Result<(), reqwest::Error> {
    let network_interfaces = list_afinet_netifas().unwrap();

    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t\t{:?}", name, ip);
    }

    let external_ip = reqwest::blocking::get("https://ipinfo.io/")?
    .json::<Ipinfo>()?;

    println!("Internet:\t{}", external_ip.ip);

    Ok(())
}
