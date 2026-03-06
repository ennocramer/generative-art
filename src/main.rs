mod arguments;

use self::arguments::*;
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
}

fn main() {
    nannou::app(model).simple_window(view).run()
}
