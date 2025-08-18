// A versatile tool used to represent a type that can take on one of several possible variants

fn main() {

    enum Gender{
        male, female
    }

    // Enhanced Enums
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home_ip = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

}