use bevy_egui::egui;
use bevy_egui::egui::{Pos2, Ui};

pub struct Turtle<'a> {
    ui: &'a mut Ui,
    pos: Pos2,
    direction: Direction,
}

#[derive(Debug, Default)]
enum Direction {
    #[default]
    X,
    XN,
    Y,
    YN,
}

impl<'a> Turtle<'a> {
    pub fn new(ui: &'a mut Ui, start: Pos2) -> Self {
        Self {
            ui,
            pos: start,
            direction: Direction::default(),
        }
    }
    pub fn left(&mut self) {
        match self.direction {
            Direction::X => self.direction = Direction::YN,
            Direction::XN => self.direction = Direction::Y,
            Direction::Y => self.direction = Direction::X,
            Direction::YN => self.direction = Direction::XN,
        }
    }

    pub fn right(&mut self) {
        match self.direction {
            Direction::X => self.direction = Direction::Y,
            Direction::XN => self.direction = Direction::YN,
            Direction::Y => self.direction = Direction::XN,
            Direction::YN => self.direction = Direction::X,
        }
    }

    pub fn forward(&mut self, distance: f32) {
        match self.direction {
            Direction::X => self.x(distance),
            Direction::XN => self.xn(distance),
            Direction::Y => self.y(distance),
            Direction::YN => self.yn(distance),
        }
    }

    fn x(&mut self, distance: f32) {
        let mut x = self.pos.x;
        x += distance;
        let next = Pos2::new(x, self.pos.y);
        self.paint(next);
        self.pos = next;
    }

    fn xn(&mut self, distance: f32) {
        let mut x = self.pos.x;
        x -= distance;
        let next = Pos2::new(x, self.pos.y);
        self.paint(next);
        self.pos = next;
    }

    fn y(&mut self, distance: f32) {
        let mut y = self.pos.y;
        y += distance;
        let next = Pos2::new(self.pos.x, y);
        self.paint(next);
        self.pos = next;
    }

    fn yn(&mut self, distance: f32) {
        let mut y = self.pos.y;
        y -= distance;
        let next = Pos2::new(self.pos.x, y);
        self.paint(next);
        self.pos = next;
    }

    fn paint(&self, next: Pos2) {
        self.ui.painter().line_segment(
            [
                self.pos,
                next
            ],
            egui::Stroke::new(1., egui::Color32::WHITE),
        )
    }
}