use raytracer::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    // projectile starts one unit above the origin.
    // velocity is normalized to 1 unit/tick.
    let mut projectile = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 0.5, 0.0).normalize(),
    };

    // gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let env = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.01, 0.0, 0.0)
    };

    loop {
        match projectile.position.get() {
            (_, y, _, _) if y > 0.0 => {
                tick(&env, &mut projectile);
                let pos = &projectile.position.get();
                let vel = &projectile.velocity.get();
                print!("Position: ({:>8.4}, {:>8.4}, {:>8.4})", pos.0, pos.1, pos.2);
                println!(" |  Velocity: ({:>8.4}, {:>8.4}, {:>8.4})", vel.0, vel.1, vel.2);
            },
            _ => break,
        }
    }

}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = &proj.position + &proj.velocity;
    proj.velocity = &proj.velocity + &env.gravity + &env.wind;
}
