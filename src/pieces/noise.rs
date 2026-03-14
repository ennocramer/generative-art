use nannou::color::Gradient;
use nannou::image::{DynamicImage, Rgb, RgbImage};
use nannou::noise::{NoiseFn, OpenSimplex, Perlin, Seedable, SuperSimplex, Worley};
use nannou::prelude::*;

use crate::arguments::Arguments;

#[derive(Copy, Clone, Debug)]
pub struct FbmSettings {
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Default for FbmSettings {
    fn default() -> Self {
        Self {
            octaves: 3,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
        }
    }
}

pub struct Fbm<T> {
    settings: FbmSettings,
    sources: Vec<T>,
    scale_factor: f64,
}

impl<T> Fbm<T>
where
    T: NoiseFn<[f64; 3]> + Seedable + Default,
{
    pub fn new(seed: u32, settings: FbmSettings) -> Self {
        let mut sources = Vec::with_capacity(settings.octaves);
        for n in 0..settings.octaves {
            sources.push(T::default().set_seed(seed + n as u32))
        }

        let denom =
            (1..=settings.octaves).fold(0.0, |acc, x| acc + settings.persistence.powi(x as i32));
        let scale_factor = 1.0 / denom;

        Self {
            settings,
            sources,
            scale_factor,
        }
    }
}

impl<T> NoiseFn<[f64; 3]> for Fbm<T>
where
    T: NoiseFn<[f64; 3]>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let mut result = 0.0;

        let mut attenuation = self.settings.persistence;

        let mut point = DVec3::from(point);
        point *= self.settings.frequency;

        for source in &self.sources {
            result += source.get(point.into()) * attenuation;

            attenuation *= self.settings.persistence;
            point *= self.settings.lacunarity;
        }

        result * self.scale_factor
    }
}

pub fn view(app: &App, arguments: &Arguments, frame: Frame) {
    let window = app.window_rect();
    let w = window.w() as u32;
    let h = window.h() as u32;

    let draw = app.draw();

    let mut image = RgbImage::new(w, h);

    let settings = FbmSettings::default();

    render_noise(
        &Fbm::<Perlin>::new(arguments.seed, settings),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(0, h / 2),
        &mut image,
    );

    render_noise(
        &Fbm::<Worley>::new(arguments.seed, settings),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(w / 2, h / 2),
        &mut image,
    );

    render_noise(
        &Fbm::<OpenSimplex>::new(arguments.seed, settings),
        app.time as f64,
        DVec2::new(16.0, 16.0),
        UVec2::new(w / 2, h / 2),
        UVec2::new(0, 0),
        &mut image,
    );

    render_noise(
        &Fbm::<SuperSimplex>::new(arguments.seed, settings),
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
    N: NoiseFn<[f64; 3]>,
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
