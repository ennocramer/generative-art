use nannou::color::Gradient;
use nannou::image::{DynamicImage, Rgb, RgbImage};
use nannou::prelude::*;
use noise::{Fbm, NoiseFn, OpenSimplex, Perlin, SuperSimplex, Worley};

use crate::arguments::Arguments;

pub fn view(app: &App, arguments: &Arguments, frame: Frame) {
    let window = app.window_rect();
    let w = window.w() as u32;
    let h = window.h() as u32;

    let draw = app.draw();

    let mut image = RgbImage::new(w, h);

    render_noise(
        &Fbm::<Perlin>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(0, h / 2),
        &mut image,
    );

    render_noise(
        &Fbm::<Worley>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(w / 2, h / 2),
        &mut image,
    );

    render_noise(
        &Fbm::<OpenSimplex>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(0, 0),
        &mut image,
    );

    render_noise(
        &Fbm::<SuperSimplex>::new(arguments.seed),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(w / 2, 0),
        &mut image,
    );

    let dyn_image = DynamicImage::ImageRgb8(image);
    let texture = wgpu::Texture::from_image(&app.main_window(), &dyn_image);

    draw.texture(&texture);

    let labels = ["Perlin", "Worley", "OpenSimplex", "SuperSimplex"];
    let quadrants = window.subdivisions();
    for (quadrant, label) in quadrants.into_iter().zip(labels) {
        draw.text(label)
            .color(GOLDENROD)
            .font_size(24)
            .wh(quadrant.pad(20.0).wh())
            .xy(quadrant.xy())
            .align_text_bottom()
            .right_justify();
    }
    draw.to_frame(app, &frame).unwrap()
}

fn render_noise<N>(
    noise: &N,
    time: f64,
    noise_area: DVec2,
    image_area: UVec2,
    image_offset: UVec2,
    image: &mut RgbImage,
) where
    N: NoiseFn<f64, 3>,
{
    let gradient =
        Gradient::new([NAVY, DODGERBLUE, ALICEBLUE].map(|c| c.into_format().into_linear()));

    for x in 0..image_area[0] {
        for y in 0..image_area[1] {
            let pixel = UVec2::new(x, y);
            let point = pixel.as_f64() / image_area.as_f64() * noise_area;
            let value = noise.get(point.extend(time).into());
            let rgb = Rgb(gradient
                .get(value as f32 * 0.5 + 0.5)
                .into_format::<u8>()
                .into_components()
                .into());
            image.put_pixel(image_offset[0] + pixel[0], image_offset[1] + pixel[1], rgb)
        }
    }
}
