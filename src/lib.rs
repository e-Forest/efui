pub mod interactor;
pub use interactor::*;
pub use macroquad::prelude::*;

pub trait EfuiItem {
    fn update(&mut self, iam: &mut InterActorManager);
    fn draw(&self);
}

pub struct Frame {
    pub rect: Rect,
    pub positioner_id: usize,
    pub scaler_id: usize,
}

impl Frame {
    pub fn new(rect: Rect, iam: &mut InterActorManager) -> Self {
        let positioner_id = iam.add_interactor();
        let scaler_id = iam.add_interactor();
        Self {
            rect,
            positioner_id,
            scaler_id,
        }
    }
}
impl EfuiItem for Frame {
    fn update(&mut self, iam: &mut InterActorManager) {
        let mut r = self.rect;
        iam.set_rect(self.positioner_id, Rect::new(r.x, r.y, 10., 10.));
        iam.set_rect(self.scaler_id, Rect::new(r.x + r.w, r.y + r.h, 10., 10.));

        let m = mouse_position();
        if iam.is_draged(self.positioner_id) {
            r.x = m.0;
            r.y = m.1;
        }

        if iam.is_draged(self.scaler_id) {
            r.w = m.0 - r.x;
            r.h = m.1 - r.y;
        }
        self.rect = r;
    }

    fn draw(&self) {
        let r = self.rect;
        draw_rectangle(r.x, r.y, r.w, r.h, BEIGE);
    }
}
