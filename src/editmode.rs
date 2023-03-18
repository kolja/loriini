
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


#[cfg(test)]
mod tests {
    use crate::EditMode;
    use crate::Mode;

    #[test]
    fn edit_mode_next() {
        let modes = vec![Mode::Hue, Mode::Lightness, Mode::Saturation];
        let mut edit_mode = EditMode {modes};
        edit_mode.next();
        assert!(edit_mode.is_active(Mode::Lightness));
    }

    #[test]
    fn edit_mode_previous() {
        let modes = vec![Mode::Hue, Mode::Lightness, Mode::Saturation];
        let mut edit_mode = EditMode {modes};
        edit_mode.previous();
        assert!(edit_mode.is_active(Mode::Saturation));
    }
}
