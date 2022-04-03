let Color =
    < Hsv: { hue: Double, sat: Double, val: Double }
    | Rgb: { red: Double, green: Double, blue: Double }
    >
in
Color.Rgb { red = 1.0, green = 1.0, blue = 0.0 }
