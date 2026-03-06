use clap::Parser;
use nannou::color::Rgb8;
use palette;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, default_value_t = 0xdeadbeef)]
    pub seed: u32,

    #[arg(short, long)]
    pub capture: Option<PathBuf>,

    #[arg(long, default_value_t = 0)]
    pub capture_frame: u64,

    #[arg(long, value_parser=parse_color, default_value="darkslategray")]
    pub background: Rgb8,

    #[arg(long, value_parser=parse_color, default_value="lightblue")]
    pub foreground: Rgb8,
}

fn parse_color(s: &str) -> Result<Rgb8, Box<dyn Error + Send + Sync + 'static>> {
    let rgb = palette::named::from_str(s)
        .ok_or(())
        .or_else(|_| s.parse::<palette::rgb::Srgb<u8>>())
        .map_err(|e| e.to_string())?;

    // The seemingly redundant conversion via components is required
    // to move between palette::Rgb objects from different library
    // versions.
    Ok(Rgb8::from_components(rgb.into_components()))
}

pub fn parse() -> Arguments {
    Arguments::parse()
}
