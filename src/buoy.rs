use crate::capture::Capture;
use crate::process::Color;
use crate::process::ImageProcess;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BuoyColor {
    RED,
    GREEN,
    BLUE,
    YELLOW,
    MARKER { color: MarkerColor },
    UNDEFINED,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MarkerColor {
    BLUE,
    PINK,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Size {
    TALL,
    SHORT,
}

#[derive(Copy, Clone)]
pub struct Buoy {
    color: BuoyColor,
    size: Size,
    coordinates: (usize, usize),
    distance: f32,
}

impl Buoy {
    pub fn new(color: BuoyColor, size: Size, coordinates: (usize, usize), distance: f32) -> Self {
        return Self {
            color,
            size,
            coordinates,
            distance,
        };
    }
    pub fn get_color(&self) -> BuoyColor{
        return self.color;
    }
    pub fn get_size(&self) -> Size{
        return self.size;
    }
    pub fn get_coordinates(&self) -> (usize, usize){
        return self.coordinates;
    }
    pub fn get_distance(&self) -> f32{
        return self.distance;
    }
}

#[derive(Clone)]
pub struct AllBuoy{
    buoys: Vec<Buoy>,
    size: usize,
    index: usize,
}

impl Iterator for AllBuoy{
    type Item = Buoy;
    fn next(&mut self) -> Option<Buoy>{
        let result = if self.index < self.size{
            self.buoys[self.index]
        } else{
            return None;
        };
        self.index += 1;
        return Some(result);
    }
}

impl AllBuoy{
    pub fn new() -> Self{
        return Self{
            buoys: Vec::new(),
            size: 0,
            index: 0,
        };
    }
    pub fn add(&mut self, buoy: Buoy){
        self.buoys.push(buoy);
        self.size += 1;
    }
    pub fn get_heading(&self) -> f32{
        let closest_red = self.get_closest_red();
        let closest_green = self.get_closest_green();
        let horizontal_fov: usize = 86;
        let horizontal_width: usize = 640;
        let center: usize = 320;
        let green_distance_from_center: usize = closest_green.get_coordinates().0 - center;
        let red_distance_from_center: usize = closest_red.get_coordinates().0 - center;
        let center_from_center: usize = green_distance_from_center - red_distance_from_center;
        let heading = (center_from_center / horizontal_width) as f32 * horizontal_fov as f32;
        return heading;
    }
    pub fn get_closest_red(&self) -> Buoy{
        let mut closest_red: Buoy = Buoy { color: BuoyColor::RED, size: Size::TALL, coordinates: (1, 1), distance: 100000.0 };
        for buoy in self.clone(){
            if buoy.get_distance() < closest_red.get_distance() && buoy.get_color() == BuoyColor::RED{
                closest_red = buoy;
            }
        }
        return closest_red;
    }
    pub fn get_closest_green(&self) -> Buoy{
        let mut closest_green: Buoy = Buoy { color: BuoyColor::GREEN, size: Size::TALL, coordinates: (1, 1), distance: 100000.0 };
        for buoy in self.clone(){
            if buoy.get_distance() < closest_green.get_distance() && buoy.get_color() == BuoyColor::GREEN{
                closest_green = buoy;
            }
        }
        return closest_green;
    }
}

const BUOY_MIN: usize = 130000;
const BUOY_MAX: usize = 150000;

pub fn find_buoys(process: &ImageProcess, capture: &Capture) -> AllBuoy {
    let mut buoy_list: AllBuoy = AllBuoy::new();
    let mut pixel_vec: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut pixel_hm: HashMap<(usize, usize), usize> = HashMap::new();
    let image = process.get_matrix();
    for i in 0..image.len(){
        for j in 0..image[i].len(){
            if i >= 1 && pixel_hm.contains_key(&(i - 1, j)) && image[i - 1][j] == image[i][j]{
                pixel_vec[*pixel_hm.get(&(i - 1, j)).unwrap()].push((i, j));
                pixel_hm.insert((i, j), *pixel_hm.get(&(i - 1, j)).unwrap());
            }
            else if j >= 1 && pixel_hm.contains_key(&(i, j - 1)) && image[i][j - 1] == image[i][j] {
                pixel_vec[*pixel_hm.get(&(i, j - 1)).unwrap()].push((i, j));
                pixel_hm.insert((i, j), *pixel_hm.get(&(i, j - 1)).unwrap());
            }
            else{
                pixel_hm.insert((i, j), pixel_vec.len());
                pixel_vec.push(vec!((i, j)));
            }
        }
    }
    let mut pixels_loc: usize = 0;
    while pixels_loc < pixel_vec.len(){
        if pixel_vec[pixels_loc].len() <= 1000{
            pixel_vec.remove(pixels_loc);
        }
        else{
            pixels_loc += 1;
        }
    }
    for ele in pixel_vec{
        let size: usize = ele.len();
        let coord: (usize, usize) = ele[2];
        let mut dist: f32 = capture.get_depth(coord.0, coord.1);
        if dist == 0.0{
            dist = capture.get_depth(ele[100].0, ele[100].1);
        }
        let multiplier: f32 =  dist * dist;
        let buoy_factor = (size as f32 * multiplier) as usize;
        if buoy_factor >= BUOY_MIN && buoy_factor <= BUOY_MAX{
            let buoy_color: BuoyColor = if image[coord.0][coord.1] == Color::RED{
                BuoyColor::RED
            } else if image[coord.0][coord.1] == Color::GREEN{
                BuoyColor::GREEN
            } else {
                BuoyColor::UNDEFINED
            };
            if buoy_color != BuoyColor::UNDEFINED{
                buoy_list.add(Buoy::new(buoy_color, Size::TALL, coord, dist));
            }
        }
    }
    return buoy_list;
}
