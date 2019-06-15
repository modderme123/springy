extern crate nalgebra;
extern crate piston_window;
use piston_window::*;
use nalgebra::*;
use std::collections::HashSet;

struct Ball {
    pos: Vector2<f64>,
    vel: Vector2<f64>,
}
struct Car {
    pos: Vector2<f64>,
    vel: Vector2<f64>,
    ang: f64,
    rot: f64,
    speed: f64,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Springy Thingy!", (640, 480))
        .build()
        .unwrap();
    let mut ball = Ball {
        pos: Vector2::new(320.0, 440.0),
        vel: Vector2::new(0.0, 0.0),
    };
    let mut car = Car {
        pos: Vector2::new(320.0, 240.0),
        vel: Vector2::new(0.0, 0.0),
        ang: 0.0,
        rot: 0.0,
        speed: 0.0,
    };
    let mut keys = HashSet::new();
    while let Some(e) = window.next() {
        e.press(|b| {
            if let Button::Keyboard(key) = b {
                keys.insert(key);
            }
        });
        e.release(|b| {
            if let Button::Keyboard(key) = b {
                keys.remove(&key);
            }
        });
        window.draw_2d(&e, |c, g| {
            if keys.contains(&Key::Left) {
                car.rot += 0.005
            }
            if keys.contains(&Key::Right) {
                car.rot -= 0.005
            }
            if keys.contains(&Key::Up) {
                car.speed += 0.02
            }
            if keys.contains(&Key::Down) {
                car.speed -= 0.02
            }

            let behind = Vector2::new(car.ang.sin(), car.ang.cos());
            ball.vel += (car.pos - ball.pos + behind * 200.0) * 0.005;

            car.ang += car.rot;

            car.vel.x -= car.ang.sin() * car.speed;
            car.vel.y -= car.ang.cos() * car.speed;

            ball.vel *= 0.90;
            car.vel *= 0.95;

            car.rot *= 0.95;
            car.speed *= 0.95;

            ball.pos += ball.vel;
            car.pos += car.vel;

            car.pos.x = car.pos.x.min(640.0).max(0.0);
            car.pos.y = car.pos.y.min(480.0).max(0.0);

            clear([0.89, 0.92, 0.93, 1.0], g);
            line(
                [0.77, 0.82, 0.84, 1.0],
                1.0,
                [
                    ball.pos.x,
                    ball.pos.y,
                    car.pos.x + car.ang.sin() * 20.0,
                    car.pos.y + car.ang.cos() * 20.0,
                ],
                c.transform,
                g,
            );
            ellipse(
                [0.90, 0.71, 0.86, 1.0],
                [
                    car.pos.x + car.ang.sin() * 15.0 - 5.0,
                    car.pos.y + car.ang.cos() * 15.0 - 5.0,
                    10.0,
                    10.0,
                ],
                c.transform,
                g,
            );
            ellipse(
                [0.73, 0.38, 0.65, 1.0],
                [ball.pos.x - 25.0, ball.pos.y - 25.0, 50.0, 50.0],
                c.transform,
                g,
            );
            polygon(
                [0.73, 0.38, 0.65, 1.0],
                &[[-10.0, 10.0], [0.0, -20.0], [10.0, 10.0]],
                c.transform.trans(car.pos.x, car.pos.y).rot_rad(-car.ang),
                g,
            );
        });
    }
}
