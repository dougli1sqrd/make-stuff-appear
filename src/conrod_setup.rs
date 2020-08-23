use std::path::PathBuf;

use conrod_core;
use conrod_piston;
use piston_window;
use piston_window::{Window, G2dTexture, G2dTextureContext};

pub trait IdMarker: Sized {}

pub struct ConrodSetupData {
    pub glyph_cache: conrod_core::text::GlyphCache<'static>,
    pub texture_cache: piston_window::G2dTexture,
    pub image_map: conrod_core::image::Map<G2dTexture>,
    pub ui: conrod_core::Ui,
    pub text_vertex_data: Vec<u8>,
    pub texture_context: G2dTextureContext
}

pub fn setup_conrod_piston(window: &mut piston_window::PistonWindow, font: PathBuf) -> ConrodSetupData {
    // Make UI object that stores all the widget info
    let size = window.size();
    let (w, h): (f64, f64) = size.into();
    let mut ui = conrod_core::UiBuilder::new([w, h])
        .build();
    
    // Insert font
    // TODO extend to take all fonts in the assets/fonts directory
    ui.fonts.insert_from_file(font).unwrap();

    let mut texture_context = window.create_texture_context();

    // Create a texture to use for efficiently caching text on the GPU.
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(640, 480)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len: usize = 640 * 480;
        let init = vec![128; buffer_len];
        let settings = piston_window::TextureSettings::new();
        let texture = piston_window::G2dTexture::from_memory_alpha(&mut texture_context, &init, 640, 480, &settings).unwrap();
        (cache, texture)
    };

    let image_map = conrod_core::image::Map::<G2dTexture>::new();
    
    ConrodSetupData {
        glyph_cache: glyph_cache,
        texture_cache: text_texture_cache,
        text_vertex_data: Vec::new(),
        image_map: image_map,
        ui: ui,
        texture_context: texture_context
    }
}

// fn update_ui_with_event<F>(event: piston_window::Event, width: f64, height: f64, update_gui: F)
//     where F: FnMut() {

// }