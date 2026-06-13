use nannou::color::Rgba8;
use nannou::glam::{Affine2, UVec2, Vec2, uvec2};
use nannou::image::{DynamicImage, Rgba, RgbaImage};
use nannou::wgpu::{Texture, WithDeviceQueuePair};

pub fn render_into_image<F, P>(image: &mut RgbaImage, shade: F)
where
    F: Fn(UVec2) -> P,
    P: Into<Rgba8>,
{
    for y in 0..image.height() {
        for x in 0..image.width() {
            let color: Rgba8 = shade(uvec2(x, image.height() - y)).into();
            image.put_pixel(x, y, Rgba(color.into_components().into()))
        }
    }
}

pub fn render_into_image_with_transform<F, P>(image: &mut RgbaImage, transform: Affine2, shade: F)
where
    F: Fn(Vec2) -> P,
    P: Into<Rgba8>,
{
    let dimensions = UVec2::from(image.dimensions()).as_f32();
    let transform = transform
        * Affine2::from_scale(2.0 * dimensions.recip())
        * Affine2::from_translation(-0.5 * dimensions + Vec2::splat(0.5));

    render_into_image(image, |pixel| {
        shade(transform.transform_point2(pixel.as_f32()))
    })
}

pub fn create_texture<S, F, P>(src: S, dimensions: UVec2, shade: F) -> Texture
where
    S: WithDeviceQueuePair,
    F: Fn(UVec2) -> P,
    P: Into<Rgba8>,
{
    let mut image = RgbaImage::new(dimensions[0], dimensions[1]);
    render_into_image(&mut image, shade);
    Texture::from_image(src, &DynamicImage::ImageRgba8(image))
}

pub fn create_texture_with_transform<S, F, P>(
    src: S,
    dimensions: UVec2,
    transform: Affine2,
    shade: F,
) -> Texture
where
    S: WithDeviceQueuePair,
    F: Fn(Vec2) -> P,
    P: Into<Rgba8>,
{
    let mut image = RgbaImage::new(dimensions[0], dimensions[1]);
    render_into_image_with_transform(&mut image, transform, shade);
    Texture::from_image(src, &DynamicImage::ImageRgba8(image))
}
