#![allow(non_upper_case_globals)]

use std::f64;
use std::sync::Arc;

use clap::Parser;
use color::write_color;
use hittable::Hittable;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressBar;
use itertools::Itertools;
use material::ScatterRecord;
use rand::Rng;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::material::dielectric::Dielectric;
use crate::material::Material;
use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::{lambertian::Lambertian, metal::Metal},
};

mod vec3;

use vec3::Vec3;
type Point3 = Vec3;
type Color = Vec3;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;

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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(ScatterRecord {
            attenuation,
            scattered,
        }) = rec.mat_ptr.scatter(r, &rec)
        {
            return attenuation
                .component_mul(&ray_color(&scattered, world, depth - 1))
                .into();
        }
        return Color::zeros();
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
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

fn random_scene() -> Box<dyn Hittable> {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Vec3 = random_vec().component_mul(&random_vec()).into();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    Box::new(world)
}

fn main() {
    const aspect_ratio: f64 = 3.0 / 2.0;
    const image_width: usize = 1200;
    const image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    const samples_per_pixel: usize = 500;
    const max_depth: usize = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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

    let progress = ProgressBar::new((image_height * image_width) as u64)
        .with_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}/{duration_precise}] {bar:40} {pos:>7}/{len:7} {msg}")
                .unwrap(),
        )
        .with_message(format!("Rendering {image_width}x{image_height}"));
    let (tx, rx) = crossbeam_channel::unbounded::<(usize, usize, Color)>();
    (0..image_height)
        .cartesian_product(0..image_width)
        .par_bridge()
        .into_par_iter()
        .progress_with(progress)
        .for_each_with(tx, |tx, (j, i)| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u =
                    (i as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v =
                    (j as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, world.as_ref(), max_depth);
            }
            tx.send((i, image_height - j - 1, pixel_color)).unwrap();
        });
    while let Ok((x, y, color)) = rx.recv() {
        write_color(&mut img, (x, y), &color, samples_per_pixel);
    }

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
