//use local_ip_address::list_afinet_netifas;
use serde::Deserialize;
extern crate ifaces;


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
    // let network_interfaces = list_afinet_netifas();
    // let network_interfaces = match network_interfaces {
    //     Ok(vect) => vect,
    //     Err(error) => panic!("Error loading network interfaces {:?}", error),
    // };


    // for (name, ip) in network_interfaces.iter() {
    //     println!("{}:\t\t{:?}", name, ip);
    // }
    let network_interfaces = ifaces::Interface::get_all().unwrap();

    for iface in network_interfaces.into_iter() {
        match iface.addr {
            Some(_) if iface.name != "lo" => {
                let tabs = match iface.name.len() {
                    12.. => "\t",
                    7.. => "\t\t",
                    _ => "\t\t\t",
                };
                if iface.name.len() > 12 {
                    println!("{}:{}{}", iface.name, tabs, iface.addr.unwrap().to_string());
                }else{
                    println!("{}:{}{}", iface.name, tabs, iface.addr.unwrap().to_string());
                }
                
            },
            _ => (),
        }
        
    }


    let external_ip = reqwest::blocking::get("https://ipinfo.io/")?
    .json::<Ipinfo>()?;

    println!("ext-inet:\t\t{}", external_ip.ip);

    Ok(())
}
