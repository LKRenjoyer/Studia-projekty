pub mod turtle {
    use std::f64::consts::PI;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    impl Color {
        pub const fn new(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b }
        }
        pub const RED: Color = Color::new(255, 0, 0);
        pub const GREEN: Color = Color::new(0, 255, 0);
        pub const BLUE: Color = Color::new(0, 0, 255);
        pub const BLACK: Color = Color::new(0, 0, 0);
        pub const WHITE: Color = Color::new(255, 255, 255);
        pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
            self.r = r;
            self.g = g;
            self.b = b;
        }
    }
    impl From<String> for Color {
        fn from(s: String) -> Self {
            let triplet: Vec<u8> = s
                .chars()
                .map(|x| if x.is_numeric() { x } else { ' ' })
                .collect::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u8>().unwrap())
                .collect();
            Color::new(triplet[0], triplet[1], triplet[2])
        }
    }
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "rgb({}, {}, {})", self.r, self.g, self.b) // zapisywanie w par. stroke w svg
        }
    }
    #[derive(Debug)]
    pub struct Turtle {
        pub drawing: bool,
        pub angle: f64,
        pub x: f64,
        pub y: f64,
        pub color: Color,
    }

    impl Turtle {
        pub const fn new() -> Self {
            Self {
                drawing: true,
                angle: 0.0,
                x: 0.0,
                y: 0.0,
                color: Color::BLACK,
            }
        }

        pub fn fd(&mut self, d: f64) {
            self.x += d * self.angle.sin();
            self.y += d * -self.angle.cos(); // - cos bo os Y jest odwrocona
        }

        pub fn turn(&mut self, deg: f64) {
            self.angle += (deg * PI) / 180.0;
            if self.angle > PI {
                self.angle -= 2.0 * PI;
            } else if self.angle < -PI {
                self.angle += 2.0 * PI
            }
        }
        pub fn set_color(&mut self, color: Color) {
            self.color = color;
        }
    }
}
