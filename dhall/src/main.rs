use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Point {
    x: u64,
    y: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
enum Color {
    Hsv { hue: f64, sat: f64, val: f64 },
    Rgb { red: f64, green: f64, blue: f64 },
}

fn main() {
    let point = serde_dhall::from_file("point.dhall")
        .parse::<Point>()
        .unwrap();
    println!("{:?}", point);

    let color = serde_dhall::from_file("color.dhall")
        .parse::<Color>()
        .unwrap();
    println!("{:?}", color);
}
