pub mod koch;
pub mod noise;

#[derive(Clone, Debug)]
pub struct Piece {
    pub title: &'static str,
    pub description: &'static str,
    pub function: fn(&nannou::App, &crate::arguments::Arguments, nannou::Frame),
}

pub static ALL_PIECES: [Piece; 2] = [
    Piece {
        title: "koch",
        description: "Animated Koch snowflake",
        function: koch::view,
    },
    Piece {
        title: "noise",
        description: "Examples of fractal brownian motion noise functions",
        function: noise::view,
    },
];
