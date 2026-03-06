use nannou::color::IntoLinSrgba;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;

#[derive(Debug, Clone)]
struct State {
    position: Vec2,
    orientation: f32,
    weight: f32,
    color: LinSrgba,
}

impl Default for State {
    fn default() -> Self {
        State {
            position: vec2(0.0, 0.0),
            orientation: 0.0,
            weight: 1.0,
            color: nannou::color::IntoLinSrgba::into_lin_srgba(WHITE),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Turtle {
    draw: Draw,
    state: State,
}

impl Turtle {
    pub fn new(draw: Draw) -> Self {
        Turtle {
            draw,
            state: State::default(),
        }
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.state.position = position;
        self
    }

    pub fn orientation(mut self, orientation: f32) -> Self {
        self.state.orientation = orientation;
        self
    }

    pub fn weight(mut self, weight: f32) -> Self {
        self.state.weight = weight;
        self
    }

    pub fn color<C>(mut self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar>,
    {
        self.state.color = color.into_lin_srgba();
        self
    }

    pub fn walk(self, distance: f32) -> Self {
        let start = self.state.position;

        let new = self.skip(distance);
        new.draw
            .line()
            .caps_round()
            .weight(new.state.weight)
            .color(new.state.color)
            .start(start)
            .end(new.state.position);

        new
    }

    pub fn skip(mut self, distance: f32) -> Self {
        self.state.position += vec2(distance, 0.0).rotate(self.state.orientation);
        self
    }

    pub fn turn(mut self, angle: f32) -> Self {
        self.state.orientation += angle;
        self
    }
}
