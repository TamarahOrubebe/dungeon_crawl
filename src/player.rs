use crate::prelude::*;

struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Player {
            position, 
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@')
        );
    }
}