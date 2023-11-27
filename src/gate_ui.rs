use crate::wire::*;
use macroquad::prelude::*;

pub struct Rec {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
}

#[derive(Default, Debug, PartialEq)]
enum State {
    #[default]
    Idle,
    DragStart,
    Dragging,
    DragStop,
}

#[derive(Debug, Clone)]
pub struct Connection {
    wire: Option<Box<Wire>>,
    pub pos: (f32, f32),
    pub radius: f32,
    pub color: Color,
    pub on: bool,
}

impl Rec {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rec { x, y, w, h, color }
    }

    fn contains(&self, pos: (f32, f32)) -> bool {
        pos.0 >= self.x && pos.0 <= self.x + self.w && pos.1 >= self.y && pos.1 <= self.y + self.h
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}

impl Connection {
    pub fn new(radius: f32, pos: (f32, f32), color: Color, on: bool) -> Connection {
        Connection {
            wire: None,
            pos,
            radius,
            color,
            on,
        }
    }

    pub fn draw(&self) {
        draw_circle(
            self.pos.0,
            self.pos.1,
            self.radius,
            if self.on { self.color } else { GRAY },
        );
    }

    fn contains(&self, pos: (f32, f32)) -> bool {
        pos.0 <= self.pos.0 + self.radius
            && pos.1 <= self.pos.1 + self.radius
            && pos.0 >= self.pos.0 - self.radius
            && pos.1 >= self.pos.1 - self.radius
    }
}

pub struct Gate<'a> {
    rect: Rec,
    input: Vec<Connection>,
    output: Connection,
    label: &'a str,
    state: State,
}

impl<'a> Gate<'a> {
    pub fn new(rect: Rec, input: Vec<Connection>, label: &'a str, output: Connection) -> Gate {
        Gate {
            rect,
            input,
            output,
            label,
            state: State::default(),
        }
    }

    pub fn draw(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) && self.rect.contains(mouse_position()) {
            self.state = State::DragStart;
        } else if is_mouse_button_down(MouseButton::Left)
            && self.rect.contains(mouse_position())
            && self.state == State::DragStart
            || self.state == State::Dragging
        {
            self.state = State::Dragging;
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.state = State::DragStop;
        }

        match self.state {
            State::Dragging => {
                self.rect.x = mouse_position().0 - self.rect.w / 2.0;
                self.rect.y = mouse_position().1 - self.rect.h / 2.0;
                self.draw_helper();
            }
            _ => {
                self.draw_helper();
            }
        }
    }

    fn draw_helper(&mut self) {
        // draws the structure of the gate
        self.rect.draw();

        // draws input
        for (cnt, node) in self.input.iter_mut().enumerate() {
            node.pos = (self.rect.x, self.rect.y + 25. + (30. * cnt as f32));
            node.draw();
        }

        // draws label
        draw_text(
            self.label,
            self.rect.x + self.rect.w / 2.0,
            self.rect.y + self.rect.h / 2.0,
            20.0,
            WHITE,
        );

        // draws output
        self.output.pos = (self.rect.x + self.rect.w, self.rect.y + self.rect.h / 2.);
        self.output.draw();
    }
}
