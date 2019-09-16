use amethyst::{
    core::{
        nalgebra::{ Vector3 },
        transform::Transform
    },
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::lighting_demo::Collider2D;
use crate::lighting_demo::Light2D;

pub struct VolumetricLightsSystem;

impl<'s> System<'s> for VolumetricLightsSystem {

    type SystemData = (
        ReadStorage<'s, Collider2D>,
        ReadStorage<'s, Light2D>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (colliders, lights, transforms): Self::SystemData) {

        let mut scene_lights = vec![];
        let vertical: Vector3<f32> = Vector3::new(0.0, 1.0, 0.5);

        for (light, transform) in (&lights, &transforms).join() {
            scene_lights.push((light, transform));
        }

        for (collider, local) in (&colliders, &transforms).join() {
            // println!("{:?}", collider.shape);

            // loop over lights and cast rays to all the corners.
            for (light, transform) in scene_lights.iter() {

                let points = collider.get_exposed_verticies(transform.translation(), light.falloff);

                println!("{:?}", points);
            }
        }
    }
}
