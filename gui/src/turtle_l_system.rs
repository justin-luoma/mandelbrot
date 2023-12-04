use crate::l_system::{LStr, LSystemIter};
use crate::turtle::Turtle;

pub struct TurtleLSystem<'a> {
    turtle: &'a mut Turtle<'a>,
    // l_system: &'a mut LSystem<'a>,
    iter: &'a mut LSystemIter<'a>,
    rules: &'a Rules,
}

pub struct Rules {
    pub(crate) forward: Vec<char>,
    pub(crate) left: Vec<char>,
    pub(crate) right: Vec<char>,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            forward: ['f'].to_vec(),
            left: ['-'].to_vec(),
            right: ['+'].to_vec(),
        }
    }
}

impl Rules {
    pub fn reversed() -> Self {
        Self {
            forward: ['f'].to_vec(),
            left: ['+'].to_vec(),
            right: ['-'].to_vec(),
        }
    }
}

impl<'a> TurtleLSystem<'a> {
    pub fn new(
        turtle: &'a mut Turtle<'a>,
        l_system_iter: &'a mut LSystemIter<'a>,
        rules: &'a Rules,
    ) -> Self {
        Self {
            turtle,
            // l_system,
            iter: l_system_iter,
            rules,
        }
    }

    pub fn draw(&mut self, n: usize, distance: f32) {
        // let mut l_str = LStr::new();
        while self.iter.n() < n {
            let l_str = self.iter.next().unwrap();
            // dbg!(&l_str);
            for c in l_str.chars() {
                if self.rules.forward.contains(&c) {
                    self.turtle.forward(distance);
                } else if self.rules.right.contains(&c) {
                    self.turtle.right();
                } else if self.rules.left.contains(&c) {
                    self.turtle.left();
                }
            }
        }
    }
}