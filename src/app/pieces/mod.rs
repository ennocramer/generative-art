pub mod koch;
pub mod noise;
pub mod square_spirals;
pub mod squares;

use nannou::wgpu::{TextureBuilder, TextureCapturer, TextureFormat, TextureUsages};
use nannou::{App, Draw, geom::Rect};

use crate::app::Application;
use crate::arguments::{GalleryArguments, GenericArguments};

pub struct PieceApplication {
    generic: GenericArguments,
    piece: Piece,
}

impl PieceApplication {
    pub fn new(_app: &App, generic: GenericArguments, piece: Piece) -> Box<Self> {
        Box::new(Self { generic, piece })
    }
}

impl Application for PieceApplication {
    fn render(&self, app: &App, draw: &Draw, window: Rect) {
        (self.piece.function)(app, &self.generic, draw, window.pad(20.0))
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub title: &'static str,
    pub description: &'static str,
    pub function: fn(
        app: &nannou::App,
        arguments: &crate::arguments::GenericArguments,
        draw: &nannou::Draw,
        window: nannou::geom::Rect,
    ),
}

pub static ALL_PIECES: [Piece; 4] = [
    Piece {
        title: "koch",
        description: "Animated Koch snowflake",
        function: koch::view,
    },
    Piece {
        title: "noise",
        description: "Examples of fractal brownian motion noise functions",
        function: noise::view,
    },
    Piece {
        title: "squares",
        description: "Noisy squares",
        function: squares::view,
    },
    Piece {
        title: "square-spirals",
        description: "Spirallig squares",
        function: square_spirals::view,
    },
];

pub fn render_gallery(
    app: &App,
    arguments: &GenericArguments,
    gallery_arguments: &GalleryArguments,
) {
    let window = app.main_window();
    let device = window.device();

    // Create a texture to use as render target.
    let texture = TextureBuilder::new()
        .size(gallery_arguments.resolution.into())
        .usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING)
        .sample_count(window.msaa_samples())
        .format(TextureFormat::Rgba16Float)
        .build(device);

    // And a renderer for the texture.
    let mut renderer = nannou::draw::RendererBuilder::new()
        .build_from_texture_descriptor(device, texture.descriptor());

    // And a capturer to retrieve texture data from the GPU.
    let capturer = TextureCapturer::default();

    for piece in &ALL_PIECES {
        let draw = Draw::new();
        let [w, h] = texture.size();
        let rect = Rect::from_w_h(w as f32, h as f32).pad(20.0);

        // Draw the piece to nannou's Draw abstraction.
        draw.background().color(arguments.background);
        (piece.function)(app, arguments, &draw, rect);

        // Encode the draw commands.
        let mut encoder = device.create_command_encoder(&Default::default());
        renderer.render_to_texture(device, &mut encoder, &draw, &texture);

        // Register a function for capturing the texture data at the end of the command stream.
        let path = gallery_arguments
            .path
            .join(piece.title)
            .with_extension("png");
        capturer
            .capture(device, &mut encoder, &texture)
            .read(move |result| {
                result
                    .expect("failed to map texture memory")
                    .to_owned()
                    .save(&path)
                    .expect("failed to save texture to png image");
            })
            .unwrap();

        // Submit the encoded draw commands to the GPU
        window.queue().submit(Some(encoder.finish()));
    }

    // Wait for all capture function to finish.
    capturer.await_active_snapshots(device).unwrap();
}
