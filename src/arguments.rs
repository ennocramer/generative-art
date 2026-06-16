use core::fmt::Write;
use std::error::Error;
use std::path::PathBuf;

use clap::builder::StyledStr;
use clap::{Args, Parser, Subcommand};
use nannou::{color::Rgb8, prelude::deg_to_rad};

use crate::app::pieces::{ALL_PIECES, Piece};

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long)]
    pub capture: Option<PathBuf>,

    #[arg(long, default_value_t = 0)]
    pub capture_frame: u64,

    #[command(flatten)]
    pub generic: GenericArguments,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Clone, Debug)]
pub struct GenericArguments {
    #[arg(short, long, default_value_t = 0xdeadbeef)]
    pub seed: u32,

    #[arg(long, value_parser=parse_color, default_value="darkslategray")]
    pub background: Rgb8,

    #[arg(long, value_parser=parse_color, default_value="lightblue")]
    pub foreground: Rgb8,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    LSystem(LSystemArguments),
    Piece(PieceArguments),
    Gallery(GalleryArguments),
}

#[derive(Parser, Clone, Debug)]
pub struct LSystemArguments {
    #[arg(short, long)]
    pub axiom: String,

    #[arg(short, long="rule", value_name="S=PRODUCTION", value_parser=parse_key_val::<char, String>)]
    pub rules: Vec<(char, String)>,

    #[arg(short, long, default_value_t = false)]
    pub terminals: bool,

    #[arg(short, long, value_parser=parse_length, default_value="10")]
    pub length: (f32, f32),

    #[arg(short, long, value_parser=parse_angles, default_value="45º")]
    pub angles: (f32, f32),

    #[arg(short, long, default_value_t = 4)]
    pub depth: u32,
}

#[derive(Parser, Clone, Debug)]
#[command(after_help=all_pieces())]
pub struct PieceArguments {
    #[arg(value_parser=parse_piece)]
    pub piece: Piece,
}

#[derive(Parser, Clone, Debug)]
pub struct GalleryArguments {
    pub path: PathBuf,

    #[arg(short, long, value_parser=parse_resolution, default_value="512x512")]
    pub resolution: (u32, u32),
}

fn all_pieces() -> StyledStr {
    let styles = clap::builder::Styles::styled();
    let mut text = StyledStr::new();
    writeln!(
        text,
        "{}Pieces:{}",
        styles.get_header().render(),
        styles.get_header().render_reset()
    )
    .unwrap();
    ALL_PIECES.iter().for_each(|piece| {
        writeln!(
            text,
            "  * {}{}{} - {}",
            styles.get_literal().render(),
            piece.title,
            styles.get_literal().render_reset(),
            piece.description
        )
        .unwrap();
    });
    text
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

fn parse_length(s: &str) -> Result<(f32, f32), Box<dyn Error + Send + Sync + 'static>> {
    if let Some((l, r)) = s.split_once(",") {
        return Ok((l.parse()?, r.parse()?));
    }

    Ok((s.parse()?, 1.0))
}

fn parse_angle(s: &str) -> Result<f32, Box<dyn Error + Send + Sync + 'static>> {
    if let Some(degrees) = s.strip_suffix('º') {
        return Ok(deg_to_rad(degrees.parse()?));
    }

    if let Some(degrees) = s.strip_suffix("deg") {
        return Ok(deg_to_rad(degrees.trim().parse()?));
    }

    Ok(s.parse()?)
}

fn parse_angles(s: &str) -> Result<(f32, f32), Box<dyn Error + Send + Sync + 'static>> {
    if let Some((l, r)) = s.split_once(",") {
        return Ok((parse_angle(l)?, parse_angle(r)?));
    }

    let angle = parse_angle(s)?;
    Ok((-angle, angle))
}

fn parse_resolution(s: &str) -> Result<(u32, u32), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s.find('x').ok_or("no `x` found")?;

    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s.find('=').ok_or("no `=` found")?;

    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn parse_piece(s: &str) -> Result<Piece, Box<dyn Error + Send + Sync + 'static>> {
    let piece = ALL_PIECES.iter().find(|piece| piece.title == s);
    Ok(piece.ok_or("Unknown piece")?.clone())
}

pub fn parse() -> Arguments {
    Arguments::parse()
}
