use std::io::Cursor;
use std::sync::Arc;

use itertools::Itertools;
use rand::Rng;

use crate::hittable::aa_rect::{XyRect, XzRect, YzRect};
use crate::hittable::box_obj::BoxObj;
use crate::hittable::bvh_node::BvhNode;
use crate::hittable::flip_face::FlipFace;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::medium::constant::ConstantMedium;
use crate::hittable::moving_sphere::MovingSphere;
use crate::hittable::rotate::RotateY;
use crate::hittable::sphere::Sphere;
use crate::hittable::translate::Translate;
use crate::hittable::Hittable;
use crate::material::dielectric::Dielectric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::texture::checker::CheckerTexture;
use crate::texture::image::ImageTexture;
use crate::texture::noise::NoiseTexture;
use crate::vec3::Vec3;
use crate::{random_vec, random_vec_range, Color, Point3};

pub fn random_scene() -> Box<dyn Hittable> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen_range(0.0..1.0);
            let center = Point3::new(
                a as f32 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rand::thread_rng().gen_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Vec3 = random_vec().component_mul(&random_vec()).into();
                    let sphere_material = Arc::new(Lambertian::new_color(albedo));
                    let center2 =
                        center + Vec3::new(0.0, rand::thread_rng().gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
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
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    Box::new(world)
}

pub fn two_spheres() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let checker_mat = Arc::new(Lambertian::new(checker));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker_mat.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker_mat,
    )));

    Box::new(objects)
}

pub fn two_perlin_spheres() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let perlin_mat = Arc::new(Lambertian::new(pertext));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_mat.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_mat,
    )));

    Box::new(objects)
}

pub fn earth() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    const EARTH_DATA: &[u8] = include_bytes!("texture/image/earthmap.jpg");
    let earth_tex = Arc::new(ImageTexture::new(&mut Cursor::new(EARTH_DATA)));

    let earth_mat = Arc::new(Lambertian::new(earth_tex));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_mat,
    )));

    Box::new(objects)
}

pub fn simple_light() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    let permat = Arc::new(Lambertian::new(pertext));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        permat.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        permat,
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    Box::new(objects)
}

pub fn cornell_box() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(Box::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let box1 = Box::new(BoxObj::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        aluminum,
    ));
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    objects.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    Box::new(objects)
}

pub fn cornell_smoke() -> Box<dyn Hittable> {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(Box::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )))));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let box1 = Box::new(BoxObj::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(Arc::new(ConstantMedium::new_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    let box2 = Box::new(BoxObj::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Box::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(Arc::new(ConstantMedium::new_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    Box::new(objects)
}

pub fn final_scene() -> Box<dyn Hittable> {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new_color(Color::new(0.48, 0.83, 0.53)));

    const boxes_per_side: usize = 20;
    (0..boxes_per_side)
        .cartesian_product(0..boxes_per_side)
        .for_each(|(i, j)| {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(BoxObj::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        });

    let mut objects = HittableList::new();
    objects.add(Arc::new(BvhNode::new(boxes1.children(), 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(FlipFace::new(Box::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )))));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new_color(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new_color(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    const ns: usize = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            random_vec_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(BvhNode::new(boxes2.children(), 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    Box::new(objects)
}
