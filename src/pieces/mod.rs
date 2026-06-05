pub mod koch;
pub mod noise;
pub mod square_spirals;
pub mod squares;

#[derive(Clone, Debug)]
pub struct Piece {
    pub title: &'static str,
    pub description: &'static str,
    pub function: fn(
        app: &nannou::App,
        arguments: &crate::arguments::Arguments,
        draw: &nannou::Draw,
        window: nannou::geom::Rect,
    ),
}

pub static ALL_PIECES: [Piece; 4] = [
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
    Piece {
        title: "squares",
        description: "Noisy squares",
        function: squares::view,
    },
    Piece {
        title: "square-spirals",
        description: "Spirallig squares",
        function: square_spirals::view,
    },
];
