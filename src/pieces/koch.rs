use nannou::prelude::*;

use crate::arguments::Arguments;
use crate::lsystem::*;

pub fn view(app: &App, arguments: &Arguments, draw: &Draw, window: Rect) {
    let angle = deg_to_rad(70.0) * (app.time * 2.0 * PI / 30.0).sin();

    {
        let lsystem = LSystem::new()
            .axiom("F")
            .rule('F', "F+F--F+F")
            .terminals(true)
            .rotation(-angle, angle)
            .length(10.0, 1.0);

        let depth = 4;
        let drawing = lsystem.measure(depth);

        let d = draw
            .scale((window.wh() / drawing.wh()).min_element() * 0.9)
            .xy(-drawing.xy());

        lsystem.generate(depth, |from, to, _| {
            d.line()
                .caps_round()
                .weight(3.0)
                .color(arguments.foreground)
                .start(from)
                .end(to);
        });
    }
}
