extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{ ALPHA, ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage },
    utils::application_root_dir,
};

mod lighting_demo;
mod systems;

use crate::lighting_demo::LightingDemo;


fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(
                    DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)
                )
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_visibility_sorting(&[])
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with(systems::VolumetricLightsSystem, "volumetric_lights_system", &[]);

    let mut game = Application::new("./", LightingDemo, game_data)?;

    game.run();

    Ok(())
}
