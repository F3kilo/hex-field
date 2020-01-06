use crate::unit_block::UnitBlock;

#[derive(Clone, Debug)]
pub struct Config {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 1f32,
            height: 1f32,
            offset_x: 0f32,
            offset_y: 0f32,
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

    pub fn hex_size(&self) -> (f32, f32) {
        (self.config.width, self.config.height)
    }

    pub fn hex_center_by_containing_point(&self, x: f32, y: f32) -> (f32, f32) {
        let (x, y) = self.to_local_coords(x, y);
        let unit_block = UnitBlock::at(x, y);
        let central = unit_block.central();
        let in_left = unit_block.point_in_left_part(x, y);
        match (central, in_left) {
            (true, true) => self.to_global_coords(unit_block.origin()),
            (true, false) => self.to_global_coords(unit_block.right_block().top_block().origin()),
            (false, true) => self.to_global_coords(unit_block.top_block().origin()),
            (false, false) => self.to_global_coords(unit_block.right_block().origin()),
        }
    }

    fn unit_size(&self) -> (f32, f32) {
        (self.config.width * 3f32 / 4f32, self.config.height / 2f32)
    }

    fn to_local_coords(&self, x: f32, y: f32) -> (f32, f32) {
        let (unit_width, unit_height) = self.unit_size();
        let x = (x - self.config.offset_x) / unit_width;
        let y = (y - self.config.offset_y) / unit_height;
        (x, y)
    }

    fn to_global_coords(&self, coords: (f32, f32)) -> (f32, f32) {
        let (x, y) = coords;
        let (unit_width, unit_height) = self.unit_size();
        let x = (x * unit_width) + self.config.offset_x;
        let y = (y * unit_height) + self.config.offset_y;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, HexField};

    fn default_hex_field() -> HexField {
        let width = 19f32;
        let height = 17f32;
        let offset_x = width / 2f32;
        let offset_y = height / 2f32;
        HexField::new(Config {
            width,
            height,
            offset_x,
            offset_y,
        })
    }

    #[test]
    fn hex_at() {
        let hf = default_hex_field();
        let (x, y) = hf.hex_center_by_containing_point(45f32, 23f32);
        assert_eq!(x, 38f32);
        assert_eq!(y, 25.5f32);

        let (x, y) = hf.hex_center_by_containing_point(45f32, 22f32);
        assert_eq!(x, 38f32);
        assert_eq!(y, 25.5f32);

        let (x, y) = hf.hex_center_by_containing_point(45f32, 18f32);
        assert_eq!(x, 52.25f32);
        assert_eq!(y, 17f32);

        let (x, y) = hf.hex_center_by_containing_point(-23f32, -42f32);
        assert_eq!(x, -19f32);
        assert_eq!(y, -42.5f32);
    }
}
