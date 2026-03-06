use std::collections::HashMap;

use nannou::geom::Rect;
use nannou::glam::{Vec2, vec2};
use nannou::math::deg_to_rad;
use nannou::prelude::Vec2Rotate;

#[derive(Debug, Clone)]
pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, String>,
    pub terminals: bool,
    pub length: (f32, f32),
    pub rotation: (f32, f32),
}

impl Default for LSystem {
    fn default() -> Self {
        Self {
            axiom: String::new(),
            rules: HashMap::new(),
            terminals: false,
            length: (10.0, 1.0),
            rotation: (deg_to_rad(-45.0), deg_to_rad(45.0)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct State {
    position: Vec2,
    step: Vec2,
}

impl LSystem {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn axiom(mut self, axiom: &str) -> Self {
        self.axiom = String::from(axiom);
        self
    }

    pub fn rule(mut self, id: char, rule: &str) -> Self {
        self.rules.insert(id, String::from(rule));
        self
    }

    pub fn rules(mut self, rules: &Vec<(char, String)>) -> Self {
        for (k, v) in rules {
            self = self.rule(*k, v)
        }
        self
    }

    pub fn terminals(mut self, terminals: bool) -> Self {
        self.terminals = terminals;
        self
    }

    pub fn length(mut self, initial: f32, factor: f32) -> Self {
        self.length = (initial, factor);
        self
    }

    pub fn rotation(mut self, left: f32, right: f32) -> Self {
        self.rotation = (left, right);
        self
    }

    pub fn measure(&self, depth: u32) -> Rect {
        let extent = &mut Rect::from_wh(Vec2::ZERO);

        self.generate(depth, |from: Vec2, to: Vec2, _| {
            *extent = extent.stretch_to(from).stretch_to(to)
        });

        *extent
    }

    pub fn generate<F>(&self, depth: u32, mut func: F)
    where
        F: FnMut(Vec2, Vec2, u32),
    {
        let mut state = State {
            position: Vec2::ZERO,
            step: vec2(self.length.0, 0.0),
        };
        let mut stack = Vec::new();
        self.generate_rec(&mut func, depth, &self.axiom, &mut state, &mut stack)
    }

    fn generate_rec<F>(
        &self,
        func: &mut F,
        depth: u32,
        rule: &str,
        state: &mut State,
        stack: &mut Vec<State>,
    ) where
        F: FnMut(Vec2, Vec2, u32),
    {
        for c in rule.chars() {
            match c {
                '.' => state.position += state.step,
                '|' => {
                    let from = state.position;
                    state.position += state.step;
                    func(from, state.position, depth)
                }
                '-' => state.step = state.step.rotate(self.rotation.0),
                '+' => state.step = state.step.rotate(self.rotation.1),
                '[' => {
                    stack.push(*state);
                    state.step *= self.length.1
                }
                ']' => {
                    if let Some(s) = stack.pop() {
                        *state = s
                    }
                }
                _ => {
                    if let Some(r) = self.rules.get(&c)
                        && depth > 0
                    {
                        self.generate_rec(func, depth - 1, r, state, stack)
                    } else {
                        if self.terminals {
                            let from = state.position;
                            state.position += state.step;
                            func(from, state.position, depth)
                        }
                    }
                }
            }
        }
    }
}
