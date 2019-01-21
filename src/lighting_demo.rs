use amethyst::{
    assets::{ AssetStorage, Loader },
    core::{
        transform::Transform,
        nalgebra::Vector3,
    },
    ecs::prelude::{ Component, DenseVecStorage },
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
        Transparent
    },
};


pub struct LightingDemo;

pub const WORLD_WIDTH: f32 = 120.0;
pub const WORLD_HEIGHT: f32 = 80.0;

fn initialise_camera(world: &mut World) {

    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            WORLD_WIDTH,
            0.0,
            WORLD_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, texture_path: &str, sheet_def: &str) -> SpriteSheetHandle {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            texture_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        sheet_def,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store
    )
}

fn create_body(world: &mut World, sheet: &SpriteSheetHandle, index: usize, x: f32, y: f32, collider_points: Vec<(f32, f32)>) {

    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);
    transform.set_scale(0.1, 0.1, 1.0);

    let mut collider = Collider2D::default();
    collider.set_polygon(collider_points);

    let sprite = SpriteRender {
        sprite_sheet: sheet.clone(),
        sprite_number: index
    };

    world.create_entity()
        .with(sprite)
        .with(transform)
        .with(collider)
        .with(Transparent)
        .build();
}

fn init_bodies(world: &mut World) {

    let bodies_sprites = load_sprite_sheet(world, "textures/bodies.png", "textures/bodies_spritesheet.ron");
    println!("[USER_DEBUG] CALL INIT BODIES !@!@!@");

    let body1_collider_points = vec![(50.0, 100.0), (300.0, 50.0), (350.0, 200.0), (75.0, 150.0)];
    create_body(world, &bodies_sprites, 0, 30.0, 30.0, body1_collider_points);
    let body2_collider_points = vec![(75.0, 350.0), (225.0, 25.0), (350.0, 200.0), (200.0, 200.0)];
    create_body(world, &bodies_sprites, 1, 70.0, 70.0, body2_collider_points);
}

impl SimpleState for LightingDemo {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        initialise_camera(world);

        world.register::<Collider2D>();

        init_bodies(world);
    }
}

#[derive(Debug)]
pub enum Collider2DShape {
    Polygon(Vec<Vector3<f32>>),
    Circle(Vector3<f32>, i32),
    Default
}

#[derive(Debug)]
pub struct Collider2D {
    pub shape: Collider2DShape
}

impl Collider2D {
    pub fn set_polygon(&mut self, points: Vec<(f32, f32)>) -> &mut Self {

        let parsed_points: Vec<Vector3<f32>> = points.into_iter()
            .map(|(x, y)| { Vector3::new(x, y, 0.5) })
            .collect();

        self.shape = Collider2DShape::Polygon(parsed_points);
        self
    }
}

impl Default for Collider2D {
    fn default() -> Self {
        Collider2D {
            shape: Collider2DShape::Default
        }
    }
}

impl Component for Collider2D {
    type Storage = DenseVecStorage<Self>;
}
