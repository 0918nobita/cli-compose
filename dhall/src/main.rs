use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Point {
    x: u64,
    y: u64,
}

fn main() {
    if let Ok(point) = serde_dhall::from_file("point.dhall").parse::<Point>() {
        println!("{:?}", point);
    } else {
        eprintln!("Failed to parse point.dhall");
        std::process::exit(1);
    }
}
