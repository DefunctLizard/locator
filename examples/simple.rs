use ipgeolocate::{Locator, Service};

// Prints the city where 1.1.1.1 is.
fn main() {
    let service = Service::IpApi;
    let ip = "1.1.1.1";

    match Locator::get(ip, service) {
        Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
        Err(error) => println!("Error: {}", error),
    };
}
