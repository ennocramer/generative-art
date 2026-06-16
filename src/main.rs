mod app;
mod arguments;
mod lsystem;
mod texture;

use nannou::event::{Event, Update};
use nannou::{App, Frame};

use self::app::{Application, lsystem::LSystemApplication, pieces::PieceApplication};
use self::arguments::*;

struct Model {
    arguments: Arguments,
    application: Box<dyn Application>,
}

fn model(app: &App) -> Model {
    let args = arguments::parse();

    app.main_window()
        .set_title(",.*' memfr0b '*.,_,.*' generative art '*.,");

    let application: Box<dyn Application> = match &args.command {
        Command::LSystem(ls_arguments) => {
            LSystemApplication::new(app, args.generic.clone(), ls_arguments.clone())
        }
        Command::Piece(piece_arguments) => {
            PieceApplication::new(app, args.generic.clone(), piece_arguments.piece.clone())
        }
        Command::Gallery(gallery_arguments) => {
            std::fs::create_dir_all(&gallery_arguments.path)
                .expect("failed to create gallery directory");
            app::pieces::render_gallery(app, &args.generic, gallery_arguments);
            app.quit();
            Box::new(())
        }
    };

    Model {
        arguments: args,
        application: application,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if let Some(path) = &model.arguments.capture
        && app.main_window().elapsed_frames() == model.arguments.capture_frame
    {
        app.main_window().capture_frame(path)
    }

    frame.clear(model.arguments.generic.background);

    let draw = app.draw();
    let area = app.window_rect();

    model.application.render(app, &draw, area);
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
    app.main_window()
        .device()
        .poll(nannou::wgpu::Maintain::Wait);
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(window_event),
            ..
        } => model.application.window_event(app, &window_event),
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.application.update(app, &update)
}

fn exit(app: &App, mut model: Model) {
    model.application.exit(app)
}

fn main() {
    nannou::app(model)
        .simple_window(view)
        .event(event)
        .update(update)
        .exit(exit)
        .run()
}
