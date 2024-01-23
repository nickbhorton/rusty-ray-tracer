mod linear_algebra;
mod object;

use crate::linear_algebra::vector::{Vec2, Vec3, Vec4};
use crate::object::objects::{Ray, Sphere};
use rayon::prelude::*;

struct Scene {
    spheres: Vec<Sphere>,
}
impl Scene {
    fn from(spheres: Vec<Sphere>) -> Scene {
        Scene { spheres }
    }
}

struct ImageSettings {
    width: u32,
    height: u32,
}
impl ImageSettings {
    fn from(width: u32, height: u32) -> ImageSettings {
        ImageSettings { width, height }
    }
}
struct Image<'a> {
    pixel_data: Vec<Vec4<u8>>,
    settings: &'a ImageSettings,
}
impl<'a> Image<'a> {
    fn from(pixel_data: Vec<Vec4<u8>>, image_settings: &'a ImageSettings) -> Image<'a> {
        Image {
            pixel_data,
            settings: image_settings,
        }
    }

    fn write_image(self, path: &std::path::Path) {
        let file = std::fs::File::create(path).unwrap();
        let ref mut w = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.settings.width, self.settings.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        let mut data: Vec<u8> = vec![];
        for pix in self.pixel_data {
            data.push(pix.x);
            data.push(pix.y);
            data.push(pix.z);
            data.push(pix.w);
        }
        writer.write_image_data(&data).unwrap();
    }
}

struct Pixel<'a> {
    scene: &'a Scene,
    image_settings: &'a ImageSettings,
    position: Vec2<u32>,
    result_color: Vec4<u8>,
}
impl<'a> Pixel<'a> {
    fn from(scene: &'a Scene, image_settings: &'a ImageSettings, x: u32, y: u32) -> Pixel<'a> {
        Pixel {
            scene,
            image_settings,
            position: Vec2::from(x, y),
            result_color: Vec4::from(0, 0, 0, 0),
        }
    }

    fn calculate(&mut self) {
        let x_center = self.position.x - self.image_settings.width / 2;
        let y_center = self.position.y - self.image_settings.height / 2;
        let screen_distance = 2.0;
        let origin = Vec3::from(0.0, 0.0, 0.0);
        let direction = Vec3::from(0.0, 0.0, 1.0);
        let ray = Ray::from(origin, direction);
        let result = self.scene.spheres[0].intersect(&ray);
        if let some_t = result {
            self.result_color = Vec4::from(0, 0, 0, 255);
        } else {
            self.result_color = Vec4::from(255, 255, 255, 255);
        }
    }

    fn get_color(self) -> Vec4<u8> {
        self.result_color
    }
}
fn main() {
    // Set up scene
    let start = std::time::Instant::now();
    let sphere1 = Sphere::from(0.0, 0.0, 10.0, 1.0);
    let sphere_vector = vec![sphere1];
    let scene = Scene::from(sphere_vector);

    let image_settings = ImageSettings::from(1000, 1000);

    let mut pixels: Vec<Pixel> = vec![];
    for y in 0..image_settings.height {
        for x in 0..image_settings.width {
            pixels.push(Pixel::from(&scene, &image_settings, x, y));
        }
    }
    let mut colors: Vec<Vec4<u8>> = vec![];
    let duration = start.elapsed();
    println!("Setup: {:?}", duration);

    // Calculate the pixels
    let start = std::time::Instant::now();
    pixels.par_iter_mut().for_each(|e| e.calculate());
    let duration = start.elapsed();
    println!("Pixel calculation: {:?}", duration);

    // Write to png file
    let start = std::time::Instant::now();
    for pix in pixels {
        colors.push(pix.get_color());
    }
    let image = Image::from(colors, &image_settings);
    let path = std::path::Path::new(r"images/test_image1.png");
    image.write_image(&path);
    let duration = start.elapsed();
    println!("Write to png: {:?}", duration);
}
