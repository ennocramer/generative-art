mod arguments;
mod lsystem;
mod pieces;

use self::arguments::*;
use self::lsystem::*;
use nannou::prelude::*;

fn model(app: &App) -> Arguments {
    let args = arguments::parse();
    app.main_window()
        .set_title(",.*' memfr0b '*.,_,.*' generative art '*.,");
    args
}

fn view(app: &App, arguments: &Arguments, frame: Frame) {
    if let Some(path) = &arguments.capture
        && app.main_window().elapsed_frames() == arguments.capture_frame
    {
        app.main_window().capture_frame(path)
    }

    frame.clear(arguments.background);

    match &arguments.command {
        Command::LSystem(ls_arguments) => view_lsystem(app, arguments, ls_arguments, frame),
        Command::Piece(piece_arguments) => {
            if piece_arguments.title == "koch" {
                pieces::koch::view(app, arguments, frame)
            } else if piece_arguments.title == "noise" {
                pieces::noise::view(app, arguments, frame)
            }
        }
    }
}

fn view_lsystem(app: &App, arguments: &Arguments, ls_arguments: &LSystemArguments, frame: Frame) {
    let window = app.window_rect();

    let lsystem = LSystem::new()
        .axiom(&ls_arguments.axiom)
        .rules(&ls_arguments.rules)
        .terminals(ls_arguments.terminals)
        .length(ls_arguments.length.0, ls_arguments.length.1)
        .rotation(ls_arguments.angles.0, ls_arguments.angles.1);

    let drawing = lsystem.measure(ls_arguments.depth);
    let scale = (window.wh() / drawing.wh()).min_element() * 0.9;

    let d = app.draw().scale(scale).xy(-drawing.xy());

    lsystem.generate(ls_arguments.depth, |from, to, _| {
        d.line()
            .caps_round()
            .weight(2.0 / scale)
            .color(arguments.foreground)
            .start(from)
            .end(to);
    });

    d.to_frame(app, &frame).unwrap()
}

fn main() {
    nannou::app(model).simple_window(view).run()
}
