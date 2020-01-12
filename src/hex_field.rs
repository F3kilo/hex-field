use crate::unit_block::UnitBlock;
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Config {
    pub hex_size: Vec2,
    pub offset: Vec2,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hex_size: Vec2::new(1f32, 1f32),
            offset: Vec2::zero(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HexField {
    config: Config,
}

impl HexField {
    pub fn new(config: Config) -> Self {
        HexField { config }
    }

    pub fn hex_size(&self) -> Vec2 {
        self.config.hex_size
    }

    pub fn hex_center_by_containing_point(&self, p: Vec2) -> Vec2 {
        let p = self.to_local_coords(p);
        let unit_block = UnitBlock::at(p);
        let central = unit_block.central();
        let in_left = unit_block.point_in_left_part(&p);
        match (central, in_left) {
            (true, true) => self.to_global_coords(unit_block.origin()),
            (true, false) => self.to_global_coords(unit_block.right_block().top_block().origin()),
            (false, true) => self.to_global_coords(unit_block.top_block().origin()),
            (false, false) => self.to_global_coords(unit_block.right_block().origin()),
        }
    }

    fn unit_size(&self) -> Vec2 {
        let unit_scales = Vec2::new(0.75f32, 0.5f32);
        self.config.hex_size * unit_scales
    }

    fn to_local_coords(&self, p: Vec2) -> Vec2 {
        (p - self.config.offset) / self.unit_size()
    }

    fn to_global_coords(&self, p: Vec2) -> Vec2 {
        (p * self.unit_size()) + self.config.offset
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, HexField};
    use glam::Vec2;

    fn default_hex_field() -> HexField {
        let width = 19f32;
        let height = 17f32;
        let offset_x = width / 2f32;
        let offset_y = height / 2f32;
        HexField::new(Config {
            hex_size: Vec2::new(width, height),
            offset: Vec2::new(offset_x, offset_y),
        })
    }

    #[test]
    fn hex_at() {
        let hf = default_hex_field();
        let p = hf.hex_center_by_containing_point(Vec2::new(45f32, 23f32));
        assert_eq!(p, Vec2::new(38f32, 25.5f32));

        let p = hf.hex_center_by_containing_point(Vec2::new(45f32, 22f32));
        assert_eq!(p, Vec2::new(38f32, 25.5f32));

        let p = hf.hex_center_by_containing_point(Vec2::new(45f32, 18f32));
        assert_eq!(p, Vec2::new(52.25f32, 17f32));

        let p = hf.hex_center_by_containing_point(Vec2::new(-23f32, -42f32));
        assert_eq!(p, Vec2::new(-19f32, -42.5f32));
    }
}
