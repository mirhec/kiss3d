extern crate native;
extern crate rand;
extern crate kiss3d;
extern crate nalgebra;

use std::rand::random;
use nalgebra::na::{Vec3, Translation};
use kiss3d::window::Window;
use kiss3d::light;
use kiss3d::post_processing::{Waves, Grayscales, SobelEdgeHighlight};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let mut window = Window::new("Kiss3d: post_processing");

    let mut c = window.add_cube(1.0, 1.0, 1.0);
    let mut b = window.add_sphere(0.5);
    let mut p = window.add_cone(0.5, 1.0);
    let mut y = window.add_cylinder(0.5, 1.0);
    let mut a = window.add_capsule(0.5, 1.0);

    c.append_translation(&Vec3::new(2.0, 0.0, 0.0));
    b.append_translation(&Vec3::new(4.0, 0.0, 0.0));
    p.append_translation(&Vec3::new(-2.0, 0.0, 0.0));
    y.append_translation(&Vec3::new(-4.0, 0.0, 0.0));
    a.append_translation(&Vec3::new(0.0, 0.0, 0.0));

    c.set_color(random(), random(), random());
    b.set_color(random(), random(), random());
    p.set_color(random(), random(), random());
    y.set_color(random(), random(), random());
    a.set_color(random(), random(), random());

    let mut sobel = SobelEdgeHighlight::new(4.0);
    let mut waves = Waves::new();
    let mut grays = Grayscales::new();

    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(light::StickToCamera);
    window.set_framerate_limit(Some(60));

    let mut time    = 0u;
    let mut counter = 0u;

    for mut frame in window.iter() {
        if time % 200 == 0 {
            time    = 0;
            counter = (counter + 1) % 4;
        }

        match counter {
            0 => { },
            1 => frame.set_post_processing_effect(&mut grays),
            2 => frame.set_post_processing_effect(&mut waves),
            3 => frame.set_post_processing_effect(&mut sobel),
            _ => unreachable!()
        }

        time = time + 1;
    }
}
