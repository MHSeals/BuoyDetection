use realsense_rust::frame::{ColorFrame, PixelKind};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    TEAL,
    PURPLE,
    GRAY,
    WHITE,
    BLACK,
    PINK,
}

#[derive(Clone)]
pub struct ImageProcess {
    rgb_image: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl ImageProcess {
    pub fn new(realsense_color: ColorFrame) -> Self {
        let dimensions: (usize, usize) = (640, 480);
        let mut combined_vec: Vec<Color> = Vec::new();
        for pixel in realsense_color.iter() {
            match pixel {
                PixelKind::Bgra8 { b, g, r, a: _ } => {
                    let color: Color = get_pixel_color(*r, *g, *b);
                    combined_vec.push(color);
                }
                _ => (),
            }
        }
        let mut rgb_image: Vec<Vec<Color>> = Vec::new();
        for i in 0..dimensions.1 {
            rgb_image.push(Vec::new());
            for j in 0..dimensions.0 {
                rgb_image[i].push(combined_vec[j + i * dimensions.0]);
            }
        }
        return Self {
            rgb_image,
            width: dimensions.0,
            height: dimensions.1,
        };
    }
    pub fn get_matrix(&self) -> Vec<Vec<Color>> {
        return self.rgb_image.clone();
    }
}

fn get_pixel_color(r: u8, g: u8, b: u8) -> Color {
    let r: u16 = r as u16;
    let g: u16 = g as u16;
    let b: u16 = b as u16;
    let brightness: u16 = (r / 3) + (g / 3) + (b / 3);
    let color: Color;
    if r > (g + b) && (g + b) < 100 {
        color = Color::RED
    } else if r > (b + 20) && r > (g + 20) {
        color = Color::PINK;
    } else if g > (r + b) && (g + b) < 100 {
        color = Color::GREEN;
    } else if b > (r + g) && (g + b) < 100 {
        color = Color::BLUE;
    } else if r >= 200 && g >= 200 && b <= 50 {
        color = Color::YELLOW;
    } else if r >= 200 && g <= 50 && b >= 200 {
        color = Color::PURPLE;
    } else if r <= 50 && g >= 200 && b >= 200 {
        color = Color::TEAL;
    } else if brightness >= 230 {
        color = Color::WHITE;
    } else if brightness <= 25 {
        color = Color::BLACK;
    } else {
        color = Color::GRAY;
    }
    return color;
}
