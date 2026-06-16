use nannou::color::{Gradient, Lch};
use nannou::geom::Quad;
use nannou::prelude::*;
use nannou::rand::rand::{RngCore, SeedableRng, rngs::SmallRng};

use crate::arguments::GenericArguments;

pub fn view(_app: &App, arguments: &GenericArguments, draw: &Draw, window: Rect) {
    let mut rng = SmallRng::seed_from_u64(arguments.seed as u64);
    subdivide(draw, arguments, &mut rng, 1, window);
}

fn subdivide(draw: &Draw, arguments: &GenericArguments, rng: &mut SmallRng, l: u32, r: Rect) {
    r.subdivisions().iter().for_each(|s| {
        if rng.next_u32().is_multiple_of(l) {
            subdivide(draw, arguments, rng, l * 2, *s)
        } else {
            spiral(draw, arguments, 32 - l, *s)
        }
    })
}

fn spiral(draw: &Draw, arguments: &GenericArguments, l: u32, r: Rect) {
    let line_width = 1.0;

    let gradient = {
        let c1 = Lch::from(arguments.foreground.into_format::<f32>());
        let mut c2 = c1;
        c2.hue += 180.0;
        Gradient::new([c1, c2])
    };

    let mut quad: Quad<Vec2> = r.corners().map_vertices(From::from);
    for i in 0..l {
        let s = i as f32 / 31.0;
        let color = gradient.get(s);

        draw.polyline()
            .caps_round()
            .weight(line_width)
            .color(color)
            .points_closed(quad.iter().cloned());

        quad = spiral_step(quad, 0.1);
    }
}

fn spiral_step(quad: Quad<Vec2>, t: f32) -> Quad<Vec2> {
    let mut newquad = quad;
    for i in 0..=3 {
        newquad.0[i] = quad.0[i].lerp(quad.0[(i + 1) % 4], t)
    }
    newquad
}
