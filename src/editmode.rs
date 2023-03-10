
#[derive(PartialEq, Clone)]
pub enum Mode {
    Hue,
    // Alpha,
    Lightness,
    Saturation,
}

pub struct EditMode {
    pub modes: Vec<Mode>,
}

impl EditMode {

    pub fn next(&mut self) {
        let active_item = self.modes.remove(0);
        self.modes.push(active_item);
    }

    pub fn previous(&mut self) {
        let last_active = self.modes.remove(self.modes.len() - 1);
        self.modes.insert(0, last_active);
    }

    pub fn active(&mut self) -> Mode {
        self.modes[0].clone()
    }

    pub fn is_active(&mut self, mode: Mode) -> bool {
        let act = self.active();
        mode == act
    }

}
