use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::lighting_demo::Collider2D;
use crate::lighting_demo::Collider2DShape;

pub struct VolumetricLightsSystem;

impl<'s> System<'s> for VolumetricLightsSystem {

    type SystemData = (
        ReadStorage<'s, Collider2D>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (colliders, transforms): Self::SystemData) {

        for (collider, local) in (&colliders, &transforms).join() {
            println!("{:?}", collider.shape);
        }
    }
}
