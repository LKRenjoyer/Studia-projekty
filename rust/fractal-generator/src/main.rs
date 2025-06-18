use std::fs::File;
use std::io::Write;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Vec<(u8, u8, u8)>>,
}
pub struct Complex {
    im: f64,
    re: f64,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let black_pixel = (0, 0, 0);
        let data = vec![vec![black_pixel; width]; height];
        Image {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        if x < self.width && y < self.height {
            self.data[y][x] = color;
        } else {
            println!("ERROR : x,y poza zakresem {x} | {y}")
        }
    }

    pub fn save_image(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        writeln!(file, "P3\n {} {} \n 255", self.width, self.height).expect("Err naglowek");
        for row in &self.data {
            for &(r, g, b) in row {
                writeln!(file, "{} {} {}", r, g, b).expect("Err zapisywanie pixela");
            }
        }
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
    pub fn to_string(self) {
        println!("{} + i{}", self.re, self.im);
    }
    pub fn add(&mut self, to_add: Complex) {
        self.re += to_add.re;
        self.im += to_add.im;
    }
    pub fn diff(&mut self, to_diff: Complex) {
        self.re -= to_diff.re;
        self.im -= to_diff.im;
    }
    pub fn mult(&mut self, to_mult: Complex) {
        let a = self.re;
        let b = self.im;
        let c = to_mult.re;
        let d = to_mult.im;
        self.re = a * c - b * d;
        self.im = b * c + a * d;
    }
    pub fn abs_square(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}
impl Copy for Complex {}
impl Clone for Complex {
    fn clone(&self) -> Self {
        *self
    }
}
fn point_of_index(
    x: usize,
    y: usize,
    min_re: f64,
    max_re: f64,
    min_im: f64,
    max_im: f64,
    width: usize,
    height: usize,
) -> (f64, f64) {
    (
        min_re + (x as f64 / width as f64) * (max_re - min_re),
        max_im - (y as f64 / height as f64) * (max_im - min_im),
    )
}
fn get_rgb(iterations: i32, max_iter: i32) -> (u8, u8, u8) {
    let t = iterations as f64 / max_iter as f64;
    let r = 12.0 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0;
    let g = 18.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0;
    let b = 10.5 * (1.0 - t) * t * t * t * 255.0;
    (
        (r as i32 % 256) as u8,
        (g as i32 % 256) as u8,
        (b as i32 % 256) as u8,
    )
}
fn get_color(point: Complex, max_iter: i32, limit: f64) -> (u8, u8, u8) {
    let mut iterations = 0;
    let mut curr_pt = Complex::new(0.0, 0.0);
    while iterations < max_iter && curr_pt.abs_square() <= limit * limit {
        curr_pt.mult(curr_pt);
        curr_pt.add(point);
        //curr_pt.to_string();
        iterations += 1;
    }
    //point.to_string();
    //curr_pt.to_string();
    //println!("{}", iterations);
    get_rgb(iterations, max_iter)
}

fn generate_image(
    min_re: f64,
    max_re: f64,
    min_im: f64,
    max_im: f64,
    im_height: usize,
    im_width: usize,
    filename: &str,
) {
    let mut image = Image::new(im_width, im_height);
    for y in 0..image.height {
        for x in 0..image.width {
            let coords = point_of_index(x, y, min_re, max_re, min_im, max_im, im_width, im_height);
            let point = Complex::new(coords.0, coords.1);
            let color = get_color(point, 800, 2.0);
            image.set_pixel(x, y, color);
        }
    }
    image.save_image(filename);
}
#[test]
fn test1() {
    generate_image(-1.5, 1.5, -1.5, 1.5, 700, 700, "set_center.ppm")
}
#[test]
fn test_full_set() {
    generate_image(-2.5, 1.0, -2.0, 2.0, 1000, 1000, "set_full.ppm");
}

#[test]
fn test_zoom_in() {
    generate_image(-0.75, -0.74, 0.1, 0.11, 800, 800, "set_zoom_in.ppm");
}

#[test]
fn test_top_right_quadrant() {
    generate_image(0.0, 1.5, 0.0, 1.5, 600, 600, "set_top_right.ppm");
}

#[test]
fn test_bottom_left_quadrant() {
    generate_image(-2.0, -0.5, -1.5, 0.0, 600, 600, "set_bottom_left.ppm");
}
#[test]
fn test_low_resolution() {
    generate_image(-1.5, 0.5, -1.0, 1.0, 200, 200, "set_low_res.ppm");
}

#[test]
fn test_wide_aspect_ratio() {
    generate_image(-2.0, 1.0, -1.0, 1.0, 1200, 600, "set_wide.ppm");
}

#[test]
fn test_tall_aspect_ratio() {
    generate_image(-1.0, 0.5, -1.5, 1.5, 600, 1200, "set_tall.ppm");
}

#[test]
fn test_off_center() {
    generate_image(0.25, 1.25, -0.5, 0.5, 700, 700, "set_off_center.ppm");
}

#[test]
fn test_deep_zoom() {
    generate_image(
        -0.5555998601048486,
        -0.5418662845040672,
        0.49153696023859084,
        0.5052705359393723,
        800,
        800,
        "set_deep_zoom.ppm",
    );
}
fn main() {}
