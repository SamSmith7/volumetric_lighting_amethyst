use amethyst::{
    core::transform::Transform,
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

        for (_, transform) in (&lights, &transforms).join() {
            scene_lights.push(transform);
        }

        for (collider, local) in (&colliders, &transforms).join() {
            // println!("{:?}", collider.shape);

            // loop over lights and cast rays to all the corners.
            // for light in scene_lights {
            //
            // }
        }
    }
}
