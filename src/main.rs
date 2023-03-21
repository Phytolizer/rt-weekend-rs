#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::f64;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use color::write_color;
use hittable::moving_sphere::MovingSphere;
use hittable::xy_rect::XyRect;
use hittable::Hittable;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use material::diffuse_light::DiffuseLight;
use material::ScatterRecord;
use rand::Rng;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use texture::checker::CheckerTexture;
use texture::image::ImageTexture;
use texture::noise::NoiseTexture;

use crate::material::dielectric::Dielectric;
use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::{lambertian::Lambertian, metal::Metal},
};

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

fn random_scene() -> Box<dyn Hittable> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new_tex(checker));
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
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Vec3 = random_vec().component_mul(&random_vec()).into();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 =
                        center + Vec3::new(0.0, rand::thread_rng().gen_range(0.0..0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
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

fn two_spheres() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let checker_mat = Arc::new(Lambertian::new_tex(checker));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker_mat.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker_mat,
    )));

    Box::new(objects)
}

fn two_perlin_spheres() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let perlin_mat = Arc::new(Lambertian::new_tex(pertext));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_mat.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_mat,
    )));

    Box::new(objects)
}

fn earth() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    const EARTH_DATA: &[u8] = include_bytes!("texture/image/earthmap.jpg");
    let earth_tex = Arc::new(ImageTexture::new(&mut Cursor::new(EARTH_DATA)));

    let earth_mat = Arc::new(Lambertian::new_tex(earth_tex));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_mat,
    )));

    Box::new(objects)
}

fn simple_light() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    let permat = Arc::new(Lambertian::new_tex(pertext));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        permat.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        permat,
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Box::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    Box::new(objects)
}

fn main() {
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: usize = 400;
    const image_height: usize = (image_width as f64 / aspect_ratio) as usize;
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
                random_scene(),
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
                two_spheres(),
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
                two_perlin_spheres(),
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
                earth(),
            )
        }
        _ => {
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
                simple_light(),
            )
        }
    };

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
