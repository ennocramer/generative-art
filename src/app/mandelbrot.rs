use nannou::color::{BLACK, Gradient, IntoColor, Lch, Rgb8, Srgb, WHITE};
use nannou::glam::{Affine2, UVec2, Vec2, vec2};
use nannou::{App, Draw, event::WindowEvent, geom::Rect, wgpu::Texture};

use num::Complex;

use crate::app::Application;
use crate::arguments::{GenericArguments, MandelbrotArguments};
use crate::texture::create_texture_with_transform;

pub struct MandelbrotApplication {
    generic: GenericArguments,
    background: Texture,
}

impl MandelbrotApplication {
    pub fn new(app: &App, generic: GenericArguments, _specific: MandelbrotArguments) -> Box<Self> {
        let background = render_mandelbrot_texture(
            app,
            app.window_rect().wh().as_u32(),
            generic.background,
            generic.foreground,
        );
        Box::new(Self {
            generic,
            background,
        })
    }
}

impl Application for MandelbrotApplication {
    fn render(&self, app: &App, draw: &Draw, window: Rect) {
        let window_size = window.wh();
        let window_aspect = window_size / window_size.min_element();

        draw.texture(&self.background);

        {
            let transform = Affine2::from_scale(2.0 * window_aspect)
                * Affine2::from_scale(2.0 * window_size.recip())
                * Affine2::from_translation(Vec2::splat(0.5));

            let inverse = transform.inverse();

            let point = transform.transform_point2(vec2(app.mouse.x, app.mouse.y));
            let init = Complex::new(point[0], point[1]);

            {
                // julia detail
                let scale = 1.0 / 5.0;
                let size = Vec2::splat(window.wh().min_element() * scale);

                let gradient =
                    Gradient::new([WHITE, BLACK].map(|c| c.into_format::<f32>().into_lch()));

                let texture = create_texture_with_transform(
                    app,
                    size.as_u32(),
                    Affine2::from_scale(Vec2::splat(2.0)),
                    |point| {
                        let n = julia(Complex::new(point[0], point[1]), init)
                            .take(256)
                            .count();
                        shade(n, &gradient)
                    },
                );

                draw.texture(&texture)
                    .wh(size)
                    .xy((window_size - size) * 0.5 - Vec2::splat(10.0));

                let streak: Vec<Vec2> = std::iter::chain(std::iter::once(init), mandelbrot(init))
                    .map(|z| inverse.transform_point2(vec2(z.re, z.im)))
                    .take(16)
                    .collect();

                for arrow in streak.windows(2) {
                    draw.arrow()
                        .weight(1.5)
                        .color(WHITE)
                        .points(arrow[0], arrow[1]);
                }
            }
        }
    }

    fn window_event(&mut self, app: &App, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.background = render_mandelbrot_texture(
                    app,
                    size.as_u32(),
                    self.generic.background,
                    self.generic.foreground,
                )
            }
            _ => {}
        }
    }
}

fn render_mandelbrot_texture(
    app: &App,
    size: UVec2,
    background: Rgb8,
    foreground: Rgb8,
) -> Texture {
    let aspect = size.as_f32() / size.as_f32().min_element();

    let gradient = Gradient::new([
        background.into_format::<f32>().into_lch(),
        foreground.into_format::<f32>().into_lch(),
    ]);

    create_texture_with_transform(app, size, Affine2::from_scale(2.0 * aspect), |point| {
        let n = mandelbrot(Complex::new(point[0], point[1]))
            .take(256)
            .count();
        shade(n, &gradient)
    })
}

fn shade(n: usize, gradient: &Gradient<Lch>) -> Rgb8 {
    if n >= 256 {
        BLACK
    } else {
        Srgb::from_linear(
            gradient
                .get((n as f32 / 255.0).powf(0.5))
                .into_rgb::<nannou::color::encoding::Srgb>(),
        )
        .into_format::<u8>()
    }
}

fn mandelbrot(init: Complex<f32>) -> impl Iterator<Item = Complex<f32>> {
    julia(init, init)
}

fn julia(init: Complex<f32>, c: Complex<f32>) -> impl Iterator<Item = Complex<f32>> {
    let mut z = init;
    std::iter::from_fn(move || {
        if z.norm_sqr() > 4.0 {
            None
        } else {
            z = z * z + c;
            Some(z)
        }
    })
}
