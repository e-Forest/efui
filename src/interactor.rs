use macroquad::prelude::*;

#[derive(Default)]
pub struct InterActor {
    id: usize,
    pub rect: Rect,
}
impl InterActor {
    pub fn id(&self) -> usize {
        self.id
    }
}
#[derive(Default)]
pub struct InterActorManager {
    last_id: usize,
    ias: Vec<InterActor>,
    hovered: Option<usize>,
    selected: Option<usize>,
    draged: Option<usize>,
    droped: Option<usize>,
}

impl InterActorManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_interactor(&mut self) -> usize {
        let id = self.last_id;
        self.last_id += 1;
        self.ias.push(InterActor {
            id,
            ..Default::default()
        });
        id
    }

    pub fn is_selected(&self, id: usize) -> bool {
        self.selected == Some(id)
    }
    pub fn is_hovered(&self, id: usize) -> bool {
        self.hovered == Some(id)
    }
    pub fn is_draged(&self, id: usize) -> bool {
        self.draged == Some(id)
    }
    pub fn is_droped(&self, id: usize) -> bool {
        self.droped == Some(id)
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.selected = None;
            self.hovered = None;
            self.draged = None;
            self.droped = None;
        }
        let ms = mouse_position();
        let ms_rect = Rect::new(ms.0, ms.1, 1., 1.);
        self.droped = None;
        self.hovered = None;
        for ia in &self.ias {
            // hovered
            if ia.rect.overlaps(&ms_rect) {
                self.hovered = Some(ia.id);
            }
            // selected & draged
            if is_mouse_button_pressed(MouseButton::Left) {
                if self.hovered == Some(ia.id) {
                    self.selected = Some(ia.id);
                    self.draged = Some(ia.id);
                }
            }
            // droped
            if !is_mouse_button_down(MouseButton::Left) {
                if self.draged == Some(ia.id) {
                    self.droped = Some(ia.id);
                    self.draged = None;
                }
            }
        }
    }
    pub fn set_rect(&mut self, id: usize, rect: Rect) {
        if let Some(ia) = self.ias.get_mut(id) {
            ia.rect = rect;
        }
    }
    pub fn draw(&self) {
        self.dbg_draw();
        for ia in &self.ias {
            let col = if self.hovered == Some(ia.id) {
                YELLOW
            } else if self.selected == Some(ia.id) {
                BLUE
            } else if self.draged == Some(ia.id) {
                GREEN
            } else if self.droped == Some(ia.id) {
                BROWN
            } else {
                MAGENTA
            };
            draw_rectangle(ia.rect.x, ia.rect.y, ia.rect.w, ia.rect.h, col);
        }
    }
    pub fn dbg_draw(&self) {
        let text = format!(
            "hovered: {:?} | selected: {:?} | draged: {:?} | droped: {:?}",
            self.hovered, self.selected, self.draged, self.droped
        );
        draw_text(&text, 0., screen_height() - 16., 16., BLACK);
    }
}
