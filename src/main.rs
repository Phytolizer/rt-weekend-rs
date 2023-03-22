#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::f64;
use std::time::Instant;

use clap::Parser;
use color::write_color;
use hittable::Hittable;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use material::ScatterRecord;
use rand::Rng;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::camera::Camera;

mod vec3;

use vec3::Vec3;
type Point3 = Vec3;
type Color = Vec3;

mod aabb;
mod camera;
mod color;
mod hittable;
mod material;
mod perlin;
mod ray;
mod texture;

fn random_vec() -> Vec3 {
    random_vec_range(0.0, 1.0)
}

fn random_vec_range(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_vec_in_unit_sphere().normalize().into()
}

fn random_vec_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            0.0,
        );
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if let Some(ScatterRecord {
            attenuation,
            scattered,
        }) = rec.mat_ptr.scatter(r, &rec)
        {
            return emitted
                + attenuation
                    .component_mul(&ray_color(&scattered, background, world, depth - 1))
                    .into();
        }
        return emitted;
    }
    background
}

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(short, long)]
    live: bool,
}

struct SdlState {
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
}

enum State {
    Online(SdlState),
    Offline,
}

mod scenes;

fn main() {
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    const max_depth: usize = 50;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let mut background = Color::new(0.0, 0.0, 0.0);
    let mut samples_per_pixel = 100;

    let (camera, world) = match 0 {
        1 => {
            background = Color::new(0.7, 0.8, 1.0);
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::random_scene(),
            )
        }
        2 => {
            background = Color::new(0.7, 0.8, 1.0);
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::two_spheres(),
            )
        }
        3 => {
            background = Color::new(0.7, 0.8, 1.0);
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::two_perlin_spheres(),
            )
        }
        4 => {
            background = Color::new(0.7, 0.8, 1.0);
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::earth(),
            )
        }
        5 => {
            samples_per_pixel = 400;
            let lookfrom = Point3::new(26.0, 3.0, 6.0);
            let lookat = Point3::new(0.0, 2.0, 0.0);
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::simple_light(),
            )
        }
        6 => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            let lookfrom = Point3::new(278.0, 278.0, -800.0);
            let lookat = Point3::new(278.0, 278.0, 0.0);
            let vfov = 40.0;
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::cornell_box(),
            )
        }
        7 => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            let lookfrom = Point3::new(278.0, 278.0, -800.0);
            let lookat = Point3::new(278.0, 278.0, 0.0);
            let vfov = 40.0;
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::cornell_smoke(),
            )
        }
        _ => {
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            let lookfrom = Point3::new(478.0, 278.0, -600.0);
            let lookat = Point3::new(278.0, 278.0, 0.0);
            let vfov = 40.0;
            (
                Camera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                    0.0,
                    1.0,
                ),
                scenes::final_scene(),
            )
        }
    };

    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let options = Options::parse();
    let state = if options.live {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window(
                "Ray Tracing in One Weekend",
                image_width as u32,
                image_height as u32,
            )
            .build()
            .unwrap()
            .into_canvas()
            .present_vsync()
            .build()
            .unwrap();
        let event_pump = sdl.event_pump().unwrap();
        State::Online(SdlState {
            canvas: window,
            event_pump,
        })
    } else {
        State::Offline
    };

    let mut img = RgbImage::new(image_width as u32, image_height as u32);

    let render_start = Instant::now();
    let (tx, rx) = crossbeam_channel::unbounded::<(usize, usize, Color)>();
    (0..image_height)
        .into_par_iter()
        .progress_with_style(
            indicatif::ProgressStyle::default_bar()
                .progress_chars("█▉▊▋▌▍▎▏ ")
                .template("[{elapsed_precise}/{duration_precise}] [{wide_bar}] {pos:>7}/{len:7} ({percent:>3}%) {msg}")
                .unwrap(),
        )
        .with_message(format!("Rendering {image_width}x{image_height}"))
        .for_each_with(tx, |tx, j| {
            for i in 0..image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rand::thread_rng().gen_range(0.0..1.0))
                        / (image_width - 1) as f64;
                    let v = (j as f64 + rand::thread_rng().gen_range(0.0..1.0))
                        / (image_height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, background, world.as_ref(), max_depth);
                }
                tx.send((i, image_height - j - 1, pixel_color)).unwrap();
            }
        });
    while let Ok((x, y, color)) = rx.recv() {
        write_color(&mut img, (x, y), &color, samples_per_pixel);
    }
    let render_end = Instant::now();
    let render_time = render_end - render_start;
    println!("Rendering took {} seconds", render_time.as_secs_f64());

    match state {
        State::Online(mut state) => {
            let texture_creator = state.canvas.texture_creator();
            let mut texture = texture_creator
                .create_texture_streaming(
                    sdl2::pixels::PixelFormatEnum::RGB24,
                    image_width as u32,
                    image_height as u32,
                )
                .unwrap();
            texture.update(None, &img, image_width * 3).unwrap();
            'sdl_loop: loop {
                for event in state.event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'sdl_loop,
                        _ => {}
                    }
                }
                state.canvas.copy(&texture, None, None).unwrap();
                state.canvas.present();
            }
        }
        State::Offline => img.save("output.png").unwrap(),
    }
}
