use amethyst::{
    assets::{ AssetStorage, Loader },
    core::{
        transform::Transform,
        nalgebra::{ Vector2, Vector3 },
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

fn create_light(world: &mut World, x: f32, y: f32) {

    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);

    let light = Light2D::default();

    world.create_entity()
        .with(transform)
        .with(light)
        .build();
}

fn init_bodies(world: &mut World) {

    let bodies_sprites = load_sprite_sheet(world, "textures/bodies.png", "textures/bodies_spritesheet.ron");

    let body1_collider_points = vec![(50.0, 100.0), (300.0, 50.0), (350.0, 200.0), (75.0, 150.0)];
    create_body(world, &bodies_sprites, 0, 30.0, 30.0, body1_collider_points);
    let body2_collider_points = vec![(75.0, 350.0), (225.0, 25.0), (350.0, 200.0), (200.0, 200.0)];
    create_body(world, &bodies_sprites, 1, 70.0, 70.0, body2_collider_points);

    create_light(world, 90.0, 90.0);
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
    Polygon {
        internal_rays: Vec<(Vector3<f32>, Vector3<f32>)>,
        vertices: Vec<Vector3<f32>>
    },
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

        let len = parsed_points.len();
        let mut rays: Vec<(Vector3<f32>, Vector3<f32>)> = vec![];

        for (idx, point1) in parsed_points.iter().enumerate() {

            let adjusted_idx = idx + len;

            for (inner_idx, point2) in parsed_points.iter().enumerate() {

                if (adjusted_idx % len) == inner_idx || (adjusted_idx + 1) % len == inner_idx || (adjusted_idx - 1) % len == inner_idx {
                    continue;
                }

                match rays.iter().find(|(pt1, pt2)| { pt1 == point2 && pt2 == point1 }) {
                    Some(_) => continue,
                    None => rays.push((*point1, *point2))
                };
            }
        }

        self.shape = Collider2DShape::Polygon {
            internal_rays: rays,
            vertices: parsed_points
        };
        dbg!(self)
    }

    pub fn ray_traverses_body(&self, ray: (Vector3<f32>, Vector3<f32>)) -> bool {

        match &self.shape {
            Collider2DShape::Polygon { internal_rays, vertices } => {

                let mut count = 0;

                for internal_ray in internal_rays.iter() {
                    if do_lines_intersect(&ray, internal_ray) {
                        count = count + 1;
                    }
                }

                count > 1
            },
            Collider2DShape::Circle(center, radius) => {
                false
            },
            Collider2DShape::Default => {
                false
            }
        }
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


#[derive(Debug)]
pub struct Light2D {
    falloff: f32,
}

impl Default for Light2D {
    fn default() -> Self {
        Light2D {
            falloff: 50.0
        }
    }
}

impl Component for Light2D {
    type Storage = DenseVecStorage<Self>;
}

fn overlap(p: Vector2<f32>, p2: Vector2<f32>, q: Vector2<f32>, q2: Vector2<f32>) -> bool {

    let start_x = q.x - p.x < 0.0;
    let x = [
        q.x - p2.x < 0.0,
        q2.x - p.x < 0.0,
        q2.x - p2.x < 0.0,
    ];

    let start_y = q.y - p.y < 0.0;
    let y = [
        q.y - p2.y < 0.0,
        q2.y - p.y < 0.0,
        q2.y - p2.y < 0.0,
    ];

    let x_all = x.iter().fold(true, |acc, x| { acc && *x == start_x });
    let y_all = y.iter().fold(true, |acc, y| { acc && *y == start_y });

    !x_all || !y_all
}

fn do_lines_intersect(line1: &(Vector3<f32>, Vector3<f32>), line2: &(Vector3<f32>, Vector3<f32>)) -> bool {

    let p = line1.0.xy();
    let p2 = line1.1.xy();
    let q = line2.0.xy();
    let q2 = line2.1.xy();

    let r = p2 - p;
    let s = q2 - q;

    let uNumerator = (q - p).perp(&r);
    let denominator = r.perp(&s);

    if uNumerator == 0.0 && denominator == 0.0 {

        if p == q || p == q2 || p2 == q || p2 == q2 {
            return true;
        }

        return overlap(p, p2, q, q2);
    }

    if denominator == 0.0 {
        return false;
    }

    let u = uNumerator / denominator;
    let t = (q - p).perp(&s) / denominator;

    (t >= 0.0) && (t <= 1.0) && (u >= 0.0) && (u <= 1.0)
}
