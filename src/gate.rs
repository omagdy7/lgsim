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

pub struct Connection {
    radius: f32,
    color: Color,
}

impl Rec {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rec { x, y, w, h, color }
    }

    fn contains(&self, pos: (f32, f32)) -> bool {
        pos.0 >= self.x && pos.0 <= self.x + self.w && pos.1 >= self.y && pos.1 <= self.y + self.h
    }
}

impl Connection {
    pub fn new(radius: f32, color: Color) -> Connection {
        Connection { radius, color }
    }
}

pub struct Gate {
    rect: Rec,
    input: Vec<Connection>,
    output: Connection,
    state: State,
}

impl Gate {
    pub fn new(rect: Rec, input: Vec<Connection>, output: Connection) -> Gate {
        Gate {
            rect,
            input,
            output,
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

        println!("{:?}", self.state);

        match self.state {
            State::Dragging => {
                self.rect.x = mouse_position().0;
                self.rect.y = mouse_position().1;
                self.draw_helper();
            }
            _ => {
                self.draw_helper();
            }
        }
    }

    fn draw_helper(&mut self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.rect.color,
        );

        for (cnt, node) in self.input.iter().enumerate() {
            draw_circle(
                self.rect.x,
                self.rect.y + 25. + (30. * cnt as f32),
                node.radius,
                node.color,
            );
        }

        draw_circle(
            self.rect.x + self.rect.w,
            self.rect.y + self.rect.h / 2.,
            self.output.radius,
            self.output.color,
        );
    }
}
