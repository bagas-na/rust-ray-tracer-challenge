use raytracer::draw::{Canvas, Color};
use raytracer::matrix::Matrix4;
use raytracer::tuple::Tuple;
use std::f64::consts::PI;
use std::fs;

const CANVAS_WIDTH: usize = 256;
const CANVAS_HEIGHT: usize = 256;

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let color = Color::new(1.0, 1.0, 1.0);
    let transform = Matrix4::identity().rotation_z(PI / 6.0);
    let mut position = Tuple::new_point(0.0, 96.0, 0.0);
    
    let (pos_x, pos_y) = pos_to_canvas_coord(&position);
    canvas.set_pixel_at(pos_x, pos_y, &color).ok();

    for _ in 0..12 {
        position = &transform * &position;
        let (pos_x, pos_y) = pos_to_canvas_coord(&position);
        canvas.set_pixel_at(pos_x, pos_y, &color).ok();
    }

    fs::write("./clock.ppm", canvas.to_ppm()).unwrap();
}

/// The origin is located at the center of the canvas
fn pos_to_canvas_coord(position: &Tuple) -> (usize, usize) {
    let (x, y, _, _) = position.get();
    let (x, y) = (x + (CANVAS_WIDTH / 2) as f64, (CANVAS_HEIGHT / 2) as f64 - y);
    (x.round() as usize, y.round() as usize)
}
