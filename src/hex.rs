use crate::unit_block::UnitBlock;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Hex {
    center: (f32, f32),
    size: (f32, f32),
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        let center = (
            (self.center.0 / self.size.0).floor() as i32,
            (self.center.1 / self.size.1).floor() as i32,
        );
        let other_center = (
            (other.center.0 / other.size.0).floor() as i32,
            (other.center.1 / other.size.1).floor() as i32,
        );
        center == other_center
    }
}

impl Eq for Hex {}

impl Hash for Hex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let center = (
            (self.center.0 / self.size.0).floor() as i32,
            (self.center.1 / self.size.1).floor() as i32,
        );
        center.hash(state);
    }
}

impl Hex {
    pub fn new(center: (f32, f32), size: (f32, f32)) -> Self {
        UnitBlock::at(0f32, 0f32);
        Hex { center, size }
    }

    pub fn top_hex(&self) -> Self {
        let x = self.center.0;
        let y = self.center.1 + self.size.1;
        Hex {
            center: (x, y),
            ..*self
        }
    }

    pub fn top_right_hex(&self) -> Self {
        let x = self.center.0 + self.size.0 * 0.75f32;
        let y = self.center.1 + self.size.1 / 2f32;
        Hex {
            center: (x, y),
            ..*self
        }
    }

    pub fn bot_right_hex(&self) -> Self {
        let x = self.center.0 + self.size.0 * 0.75f32;
        let y = self.center.1 - self.size.1 / 2f32;
        Hex {
            center: (x, y),
            ..*self
        }
    }

    pub fn bot_hex(&self) -> Self {
        let x = self.center.0;
        let y = self.center.1 - self.size.1;
        Hex {
            center: (x, y),
            ..*self
        }
    }

    pub fn bot_left_hex(&self) -> Self {
        let x = self.center.0 - self.size.0 * 0.75f32;
        let y = self.center.1 - self.size.1 / 2f32;
        Hex {
            center: (x, y),
            ..*self
        }
    }

    pub fn top_left_hex(&self) -> Self {
        let x = self.center.0 - self.size.0 * 0.75f32;
        let y = self.center.1 + self.size.1 / 2f32;
        Hex {
            center: (x, y),
            ..*self
        }
    }

    pub fn neighbors(&self) -> Vec<Hex> {
        vec![
            self.top_hex(),
            self.top_right_hex(),
            self.bot_right_hex(),
            self.bot_hex(),
            self.bot_left_hex(),
            self.top_left_hex(),
        ]
    }

    pub fn top_left(&self) -> (f32, f32) {
        let x = self.center.0 - self.size.0 / 4f32;
        let y = self.center.1 + self.size.1 / 2f32;
        (x, y)
    }

    pub fn top_right(&self) -> (f32, f32) {
        let x = self.center.0 + self.size.0 / 4f32;
        let y = self.center.1 + self.size.1 / 2f32;
        (x, y)
    }

    pub fn right(&self) -> (f32, f32) {
        let x = self.center.0 + self.size.0 / 2f32;
        let y = self.center.1;
        (x, y)
    }

    pub fn bot_right(&self) -> (f32, f32) {
        let x = self.center.0 + self.size.0 / 4f32;
        let y = self.center.1 - self.size.1 / 2f32;
        (x, y)
    }

    pub fn bot_left(&self) -> (f32, f32) {
        let x = self.center.0 - self.size.0 / 4f32;
        let y = self.center.1 - self.size.1 / 2f32;
        (x, y)
    }

    pub fn left(&self) -> (f32, f32) {
        let x = self.center.0 - self.size.0 / 2f32;
        let y = self.center.1;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::Hex;

    fn default_hex() -> Hex {
        let center = (1f32, 1f32);
        let size = (2f32, 1f32);
        Hex::new(center, size)
    }

    #[test]
    fn top() {
        let h = default_hex();
        assert_eq!(h.top_hex().bot_left(), h.top_left());
        assert_eq!(h.top_hex().bot_right(), h.top_right());
    }

    #[test]
    fn top_right() {
        let h = default_hex();
        assert_eq!(h.top_right_hex().bot_left(), h.right());
        assert_eq!(h.top_right_hex().left(), h.top_right());
    }

    #[test]
    fn bot_right() {
        let h = default_hex();
        assert_eq!(h.bot_right_hex().top_left(), h.right());
        assert_eq!(h.bot_right_hex().left(), h.bot_right());
    }

    #[test]
    fn bot() {
        let h = default_hex();
        assert_eq!(h.bot_hex().top_left(), h.bot_left());
        assert_eq!(h.bot_hex().top_right(), h.bot_right());
    }

    #[test]
    fn bot_left() {
        let h = default_hex();
        assert_eq!(h.bot_left_hex().right(), h.bot_left());
        assert_eq!(h.bot_left_hex().top_right(), h.left());
    }

    #[test]
    fn top_left() {
        let h = default_hex();
        assert_eq!(h.top_left_hex().right(), h.top_left());
        assert_eq!(h.top_left_hex().bot_right(), h.left());
    }
}
