use nannou::{App, Draw, geom::Rect};

use crate::app::Application;
use crate::arguments::{GenericArguments, LSystemArguments};
use crate::lsystem::LSystem;

pub struct LSystemApplication {
    generic: GenericArguments,
    specific: LSystemArguments,
}

impl LSystemApplication {
    pub fn new(
        _app: &App,
        generic: GenericArguments,
        specific: LSystemArguments,
    ) -> Box<LSystemApplication> {
        Box::new(Self { generic, specific })
    }
}

impl Application for LSystemApplication {
    fn render(&self, _app: &App, draw: &Draw, window: Rect) {
        let lsystem = LSystem::new()
            .axiom(&self.specific.axiom)
            .rules(&self.specific.rules)
            .terminals(self.specific.terminals)
            .length(self.specific.length.0, self.specific.length.1)
            .rotation(self.specific.angles.0, self.specific.angles.1);

        let drawing = lsystem.measure(self.specific.depth);
        let scale = (window.wh() / drawing.wh()).min_element();

        let d = draw.scale(scale).xy(-drawing.xy());

        lsystem.generate(self.specific.depth, |from, to, _| {
            d.line()
                .caps_round()
                .weight(2.0 / scale)
                .color(self.generic.foreground)
                .start(from)
                .end(to);
        });
    }
}
