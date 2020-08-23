extern crate piston_window;
#[macro_use]
extern crate conrod_core;
extern crate conrod_piston;
extern crate find_folder;
extern crate specs;
#[macro_use]
extern crate specs_derive;

// piston_window re-exports glutin stuff I guess
use piston_window::*;

pub mod conrod_setup;
pub mod components;
pub mod systems;


fn main() {
    println!("Hello, world!");

    // make_a_window();
    // make_stuff_in_a_window();
    // make_scaling_stuff_in_window();
    // make_conrod_element_in_window();
    moveable_box_in_the_window();
}

fn make_a_window() {
    let mut window: PistonWindow = WindowSettings::new("Hello, a window exists", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g)
        });
    }
}

fn make_stuff_in_a_window() {
    let mut window: PistonWindow = WindowSettings::new("Hello, a with stuff", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();


    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], [20.0, 20.0, 100.0, 100.0], c.transform, g);
        });
        // println!("Hello, stuck in a loop {:?}", e)
    }
}

fn make_scaling_stuff_in_window() {
    let mut window: PistonWindow = WindowSettings::new("Hello, a with stuff", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    while let Some(e) = window.next() {
        // Draw a red box that is 10% of the size of the window in the x and y coord
        // TODO a function that basically will auto scale any shape to the window size?
        // TODO maybe one to scale size based on window, keeping top left anchored, and
        // TODO another where the total position somehow scales as well, to keep the ratio of distances all the same
        window.draw_2d(&e, |c, g, _d| {
            let size = c.get_view_size();
            let scale = 0.1;
            let topleft = [20.0, 20.0];
            clear([0.5, 1.0, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], [topleft[0], topleft[1], topleft[0] + scale*size[0], topleft[1] + scale*size[1]], c.transform, g);
        });
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        hello_text,
    }
}

impl conrod_setup::IdMarker for Ids {}

fn make_conrod_element_in_window() {
    let mut window: PistonWindow = WindowSettings::new("Hello, a with stuff", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/UbuntuMono-R.ttf");


    let conrod_setup_data = conrod_setup::setup_conrod_piston(&mut window, font_path);
    let mut ui = conrod_setup_data.ui; // Move Ui out of the struct
    let ids = Ids::new(ui.widget_id_generator());

    fn gui(ui: &mut conrod_core::UiCell, ids: &Ids) {
        use conrod_core::Widget;
        use conrod_core::Positionable;
        use conrod_core::Colorable;
        use conrod_core::Sizeable;
        use conrod_core::position::*;

        conrod_core::widget::Canvas::new()
            .pad(5.0)
            .color(conrod_core::color::LIGHT_GREEN)
            .x_position(conrod_core::Position::Relative(Relative::Align(Align::End), None))
            .y_position(conrod_core::Position::Relative(Relative::Align(Align::End), None))
            .h(100.0)
            .scroll_kids_vertically()
            .set(ids.canvas, ui);
        
        conrod_core::widget::Text::new("Hello World")
            .font_size(42)
            .mid_top_of(ids.canvas)
            .set(ids.hello_text, ui);
    }
    
    // Grab all the variables out of the `conrod_setup_data`.
    // TODO maybe this should be destructerable as a tuple or something?
    let mut text_vertex_data = conrod_setup_data.text_vertex_data;
    let mut texture_context = conrod_setup_data.texture_context;
    let mut text_texture_cache = conrod_setup_data.texture_cache;
    let mut glyph_cache = conrod_setup_data.glyph_cache;
    let image_map = conrod_setup_data.image_map;

    while let Some(event) = window.next() {
        let size = window.size();
        let (width, height) = (size.width as conrod_core::Scalar, size.height as conrod_core::Scalar);
        if let Some(e) = conrod_piston::event::convert(event.clone(), width, height) {
            ui.handle_event(e);
        }

        // Update UI
        event.update(|_| {
            let mut ui = ui.set_widgets();
            gui(&mut ui, &ids);
        });

        // Draw UI
        window.draw_2d(&event, |context, graphics, device| {
            
            if let Some(primitives) = ui.draw_if_changed() {
                clear([0.5, 0.5, 0.5, 1.0], graphics);
                rectangle([1.0, 0.0, 0.0, 1.0], [width - 100.0, height - 100.0, 80.0, 80.0], context.transform, graphics);

                let cache_queued_glyphs = | _graphics: &mut G2d,
                                             cache: &mut G2dTexture,
                                             rect: conrod_core::text::rt::Rect<u32>,
                                             data: &[u8]| {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    piston_window::texture::UpdateTexture::update(cache, &mut texture_context, format, &text_vertex_data[..], offset, size).expect("Failed to update Texture");
                };

                fn texture_from_image<T>(img: &T) -> &T { img }

                // Draw the conrod `render::Primitives`.
                conrod_piston::draw::primitives(
                    primitives,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image);
                
                texture_context.encoder.flush(device);
            }
        });
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MovementDirection {
    Left(f32),
    Right(f32),
    Up(f32),
    Down(f32),
    None
}

fn moveable_box_in_the_window() {
    use specs::{World, WorldExt, Builder, Join};
    use specs::shrev::EventChannel;

    let mut world = specs::World::new();
    world.register::<components::BoxShape>();
    world.register::<components::Controllable>();
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Renderable>();

    let _the_box = world.create_entity()
        .with(components::Controllable)
        .with(components::Renderable)
        .with(components::Position{x: 20.0, y: 200.0})
        .with(components::Velocity{x: 0.0, y: 0.0})
        .with(components::BoxShape{size: 80.0}).build();

    // Insert Empty movement resource
    let window_event_channel = EventChannel::<piston_window::Event>::new();
    world.insert(MovementDirection::None);
    world.insert(window_event_channel);

    let mut window: PistonWindow = WindowSettings::new("Hello, a with stuff", (640, 480))
        .exit_on_esc(true)
        .automatic_close(true)
        .build()
        .unwrap();

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(systems::ButtonSystem::new(&mut world), "button", &[])
        .with(systems::MoveVelocitySystem, "movement", &["button"])
        .build();


    while let Some(e) = window.next() {
        if let Some(channel) = world.get_mut::<EventChannel<piston_window::Event>>() {
            channel.single_write(e.clone());
        }
        dispatcher.dispatch(&world);
        world.maintain();

        let positions = world.read_storage::<components::Position>();
        let shapes = world.read_storage::<components::BoxShape>();
        let pos_shapes_joined = (&positions, &shapes).join();

        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);

            for (position, shape) in pos_shapes_joined {
                rectangle([1.0, 0.0, 0.0, 1.0], [position.x as f64, position.y as f64, shape.size as f64, shape.size as f64], c.transform, g);
            }

        });

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

