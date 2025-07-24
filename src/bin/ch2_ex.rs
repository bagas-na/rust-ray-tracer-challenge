use raytracer::draw::{Canvas, Color};
use raytracer::tuple::Tuple;
use std::fs;

const CANVAS_WIDTH: usize = 960;
const CANVAS_HEIGHT: usize = 540;
const TICK_RATE: f64 = 1.0;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let mut color = Color::new(0.99, 0.625, 0.0);

    let mut projectile = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.5, 1.0, 0.0).normalize() * 10.0,
    };

    let env = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.04, 0.0, 0.0),
    };

    let (pos_x, pos_y) = position_to_canvas_pixel_at(&projectile.position);
    canvas.set_pixel_at(pos_x, pos_y, &color).ok();

    loop {
        color = color + &Color::new(0.0,0.0,0.01);
        match projectile.position.get() {
            (_, y, _, _) if y > 0.0 => {
                tick(&env, &mut projectile, TICK_RATE);
                let (pos_x, pos_y) = position_to_canvas_pixel_at(&projectile.position);
                canvas.set_pixel_at(pos_x, pos_y, &color).ok();
            }
            _ => break,
        }
    }

    let ppm_string = canvas.to_ppm();
    fs::write("./trajectory.ppm", ppm_string).unwrap();
}

fn tick(env: &Environment, proj: &mut Projectile, rate: f64) {
    proj.position = &proj.position + (&proj.velocity) * rate;
    proj.velocity = &proj.velocity + (&env.gravity + &env.wind) * rate;
}

fn position_to_canvas_pixel_at(position: &Tuple) -> (usize, usize) {
    let (x, y, _, _) = position.get();
    (x.round() as usize, CANVAS_HEIGHT - (y.round() as usize))
}
