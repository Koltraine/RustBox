// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use conrod::{self, widget, Positionable, Widget};
use piston_window::{G2d, G2dTexture, TextureSettings};
use piston_window::{Input, UpdateArgs, Context};
use piston_window::texture::{UpdateTexture, Format};
use piston_window;
use find_folder;
use std;
use gfx::{Factory, UpdateError};
use gfx_device_gl::Resources;
use conrod::Colorable;
use conrod::text::GlyphCache;
use conrod::image::Map;
use conrod::backend::piston::draw;
use objects::{Renderable, EventHandler, Updatable};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        text,
    }
}


pub struct Ui {
    ui: conrod::Ui,
    text_texture_cache: G2dTexture,
    glyph_cache: GlyphCache,
    image_map: Map<G2dTexture>,
}

impl Ui {
    pub fn new<F: Factory<Resources>>(factory: &mut F) -> Ui {
        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([640.0, 480.0])
            .theme(theme())
            .build();

        // TODO: we dont really need this
        // or at least lazy_static! it
        let fonts = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("fonts").unwrap();
        println!("{:?}", fonts);

        let ref font = fonts.join("FiraSans-Regular.ttf");
        ui.fonts.insert_from_file(font).unwrap();

        // Create a texture to use for efficiently caching text on the GPU.
        let (glyph_cache, text_texture_cache) = {
            const SCALE_TOLERANCE: f32 = 0.1;
            const POSITION_TOLERANCE: f32 = 0.1;
            let cache = GlyphCache::new(640, 480, SCALE_TOLERANCE, POSITION_TOLERANCE);
            let buffer_len = 640 * 480;
            let init = vec![128; buffer_len];
            let settings = TextureSettings::new();
            let texture = G2dTexture::from_memory_alpha(factory, &init, 640, 480, &settings).unwrap();
            (cache, texture)
        };
        let image_map = conrod::image::Map::new();

        // Instantiate the generated list of widget identifiers.
        let ids = Ids::new(ui.widget_id_generator());


        // Instantiate all widgets in the GUI.
        {
            let ui = &mut ui.set_widgets();

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Hello World!")
                .middle_of(ui.window)
                .color(conrod::color::WHITE)
                .font_size(64)
                .set(ids.text, ui);
        }

        Ui {
            glyph_cache,
            image_map,
            text_texture_cache,
            ui,
        }
    }
}

impl Renderable for Ui {
    fn render(&mut self, c: Context, g: &mut G2d) {
        let primitives = self.ui.draw();

        // Draw the conrod `render::Primitives`.
        draw::primitives(
            primitives,
            c,
            g,
            &mut self.text_texture_cache,
            &mut self.glyph_cache,
            &self.image_map,
            Ui::cache_queued_glyphs,
            Ui::texture_from_image
        );

    }
}
impl Ui {
    // Specify how to get the drawable texture from the image.
    // In this case, the image *is* the texture.
    fn texture_from_image<T>(img: &T) -> &T { img }

    // A function used for caching glyphs to the texture cache.
    fn cache_queued_glyphs(
        graphics: &mut G2d,
        cache: &mut G2dTexture,
        rect: conrod::text::rt::Rect<u32>,
        data: &[u8]
    ) {
            let mut text_vertex_data = Vec::new();
            let offset = [rect.min.x, rect.min.y];
            let size = [rect.width(), rect.height()];
            let encoder = &mut graphics.encoder;
            text_vertex_data.clear();
            text_vertex_data.extend(data.iter()
                                    .flat_map(
                                        |&b| vec![255, 255, 255, b]
                                    ));

            UpdateTexture::update(
                cache,
                encoder,
                Format::Rgba8,
                &text_vertex_data[..],
                offset,
                size
            ).expect("failed to update texture");
   }
}

impl Updatable for Ui {
    fn update(&mut self, upd: UpdateArgs) {
    }
}

impl EventHandler for Ui {
    fn event(&mut self, event: &Input) {
        if let Some(event) = conrod::backend::piston::event::convert(
            event.clone(),
            640.0,
            480.0
            ) {
            self.ui.handle_event(event);
        }
    }
}


/// A set of reasonable stylistic defaults
pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

