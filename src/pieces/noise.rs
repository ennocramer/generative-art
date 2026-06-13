use nannou::color::Gradient;
use nannou::prelude::*;
use nannou::wgpu::{Texture, WithDeviceQueuePair};
use noise::{Fbm, NoiseFn, OpenSimplex, Perlin, SuperSimplex, Worley};

use crate::arguments::Arguments;
use crate::texture::create_texture;

pub fn view(app: &App, arguments: &Arguments, draw: &Draw, window: Rect) {
    let w = window.w() as u32;
    let h = window.h() as u32;

    let perlin = render_noise(
        app,
        &Fbm::<Perlin>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
    );

    let worley = render_noise(
        app,
        &Fbm::<Worley>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
    );

    let opensimplex = render_noise(
        app,
        &Fbm::<OpenSimplex>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
    );

    let supersimplex = render_noise(
        app,
        &Fbm::<SuperSimplex>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
    );

    let labels = ["Perlin", "Worley", "OpenSimplex", "SuperSimplex"];
    let textures = [perlin, worley, opensimplex, supersimplex];
    let quadrants = window.subdivisions();
    for ((quadrant, label), texture) in quadrants.into_iter().zip(labels).zip(&textures) {
        draw.texture(&texture)
            .wh(quadrant.pad(10.0).wh())
            .xy(quadrant.xy());
        draw.text(label)
            .color(GOLDENROD)
            .font_size(24)
            .wh(quadrant.pad(20.0).wh())
            .xy(quadrant.xy())
            .align_text_bottom()
            .right_justify();
    }
}

fn render_noise<S, N>(src: S, noise: &N, time: f64, noise_area: DVec2, image_area: UVec2) -> Texture
where
    S: WithDeviceQueuePair,
    N: NoiseFn<f64, 3>,
{
    let gradient: Gradient<LinSrgb> =
        Gradient::new([NAVY, DODGERBLUE, ALICEBLUE].map(|c| c.into_format().into_linear()));

    create_texture(src, image_area, |pixel| {
        let point = pixel.as_f64() / image_area.as_f64() * noise_area;
        let value = noise.get(point.extend(time).into());
        Srgb::from_linear(gradient.get(value as f32 * 0.5 + 0.5)).into_format()
    })
}
