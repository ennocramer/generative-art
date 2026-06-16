pub mod lsystem;
pub mod mandelbrot;
pub mod pieces;

use nannou::event::{Update, WindowEvent};
use nannou::{App, Draw, geom::Rect};

pub trait Application {
    fn window_event(&mut self, _app: &App, _event: &WindowEvent) {}
    fn update(&mut self, _app: &App, _update: &Update) {}
    fn render(&self, _app: &App, _draw: &Draw, _area: Rect) {}
    fn exit(&mut self, _app: &App) {}
}

impl Application for () {}
