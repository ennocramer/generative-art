use nannou::color::{Gradient, Lch};
use nannou::prelude::*;
use noise::{NoiseFn, SuperSimplex};

use crate::arguments::Arguments;

pub fn view(app: &App, arguments: &Arguments, draw: &Draw, window: Rect) {
    let target = window.wh().min_element() / 25.0;
    let num_rects = (window.wh() / target).as_i32();
    let square_size = window.wh() / num_rects.as_f32();

    let line_width = target * 0.05;

    let noise_scale = 10.0 / window.wh().max_element();
    let noise = SuperSimplex::new(arguments.seed);

    let gradient = {
        let c1 = Lch::from(arguments.foreground.into_format::<f32>());
        let mut c2 = c1;
        c2.hue += 180.0;
        Gradient::new([c1, c2])
    };

    for i in 0..num_rects[0] {
        for j in 0..num_rects[1] {
            let position = ivec2(i, j).as_f32();
            let progress = (position / num_rects.as_f32()).max_element();
            let color = gradient.get(progress);
            let noise_intensity = progress * target / 2.0;

            let corners = Rect::from_wh(square_size * 0.9)
                .top_left_of(window)
                .shift((position * square_size + square_size * 0.05) * vec2(1.0, -1.0))
                .corners_iter()
                .map(|c| {
                    perturb(
                        &noise,
                        app.time / 5.0,
                        noise_scale,
                        noise_intensity,
                        c.into(),
                    )
                });

            draw.polyline()
                .caps_round()
                .weight(line_width)
                .color(color)
                .points_closed(corners);
        }
    }
}

fn perturb<N>(noise: &N, time: f32, scale: f32, intensity: f32, vertex: Vec2) -> Vec2
where
    N: NoiseFn<f64, 3>,
{
    let offset_x = noise.get((vertex * scale).extend(time).as_f64().into());
    let offset_y = noise.get((vertex * scale).extend(-time).as_f64().into());
    vertex + dvec2(offset_x, offset_y).as_f32() * intensity
}
