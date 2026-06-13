mod arguments;
mod lsystem;
mod pieces;
mod texture;

use self::arguments::*;
use self::lsystem::*;
use nannou::prelude::*;

fn model(app: &App) -> Arguments {
    let args = arguments::parse();

    app.main_window()
        .set_title(",.*' memfr0b '*.,_,.*' generative art '*.,");

    if let Command::Gallery(gallery_arguments) = &args.command {
        std::fs::create_dir_all(&gallery_arguments.path)
            .expect("failed to create gallery directory");
        render_gallery(app, &args, gallery_arguments);
        app.quit()
    }

    args
}

fn view(app: &App, arguments: &Arguments, frame: Frame) {
    if let Some(path) = &arguments.capture
        && app.main_window().elapsed_frames() == arguments.capture_frame
    {
        app.main_window().capture_frame(path)
    }

    frame.clear(arguments.background);

    let draw = app.draw();
    let window = app.window_rect().pad(20.0);

    match &arguments.command {
        Command::LSystem(ls_arguments) => view_lsystem(arguments, &draw, window, ls_arguments),
        Command::Piece(piece_arguments) => {
            (piece_arguments.piece.function)(app, arguments, &draw, window)
        }
        Command::Gallery(_) => unreachable!(),
    }

    draw.to_frame(app, &frame).unwrap();

    // Force waiting until drawing is complete.
    //
    // Hack to avoid texture ids being recycled too early. If
    // temporary texture is created and dropped within a view
    // function, it's texture id can be reused in the next iteration,
    // before the previous incarnation is fully drawn. This can lead
    // to one frame's texture content to show up in the previous
    // frame's output.
    //
    // This does not prevent texture content leaking between textures
    // if they are created and dropped in sequence within one frame.
    app.main_window().device().poll(wgpu::Maintain::Wait);
}

fn view_lsystem(arguments: &Arguments, draw: &Draw, window: Rect, ls_arguments: &LSystemArguments) {
    let lsystem = LSystem::new()
        .axiom(&ls_arguments.axiom)
        .rules(&ls_arguments.rules)
        .terminals(ls_arguments.terminals)
        .length(ls_arguments.length.0, ls_arguments.length.1)
        .rotation(ls_arguments.angles.0, ls_arguments.angles.1);

    let drawing = lsystem.measure(ls_arguments.depth);
    let scale = (window.wh() / drawing.wh()).min_element();

    let d = draw.scale(scale).xy(-drawing.xy());

    lsystem.generate(ls_arguments.depth, |from, to, _| {
        d.line()
            .caps_round()
            .weight(2.0 / scale)
            .color(arguments.foreground)
            .start(from)
            .end(to);
    });
}

fn render_gallery(app: &App, arguments: &Arguments, gallery_arguments: &GalleryArguments) {
    let window = app.main_window();
    let device = window.device();

    // Create a texture to use as render target.
    let texture = wgpu::TextureBuilder::new()
        .size(gallery_arguments.resolution.into())
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        .sample_count(window.msaa_samples())
        .format(wgpu::TextureFormat::Rgba16Float)
        .build(device);

    // And a renderer for the texture.
    let mut renderer = nannou::draw::RendererBuilder::new()
        .build_from_texture_descriptor(device, texture.descriptor());

    // And a capturer to retrieve texture data from the GPU.
    let capturer = wgpu::TextureCapturer::default();

    for piece in &pieces::ALL_PIECES {
        let draw = nannou::Draw::new();
        let [w, h] = texture.size();
        let rect = geom::Rect::from_w_h(w as f32, h as f32).pad(20.0);

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

fn main() {
    nannou::app(model).simple_window(view).run()
}
