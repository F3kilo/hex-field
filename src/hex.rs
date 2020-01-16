use glam::Vec2;
use std::hash::{Hash, Hasher};

pub enum VertexDir {
    LeftTop,
    RightTop,
    Right,
    RightBot,
    LeftBot,
    Left,
}

pub struct VertexDirIter {
    counter: usize,
}

impl VertexDirIter {
    fn new() -> Self {
        VertexDirIter { counter: 0 }
    }
}

impl Iterator for VertexDirIter {
    type Item = VertexDir;

    fn next(&mut self) -> Option<VertexDir> {
        match self.counter {
            0 => Some(VertexDir::LeftTop),
            1 => Some(VertexDir::RightTop),
            2 => Some(VertexDir::Right),
            3 => Some(VertexDir::RightBot),
            4 => Some(VertexDir::LeftBot),
            5 => Some(VertexDir::Left),
            _ => None,
        }
    }
}

pub enum HexDir {
    Top,
    RightTop,
    RightBot,
    Bot,
    LeftBot,
    LeftTop,
}

#[derive(Debug)]
pub struct HexDirIter {
    counter: usize,
}

impl HexDirIter {
    fn new() -> Self {
        HexDirIter { counter: 0 }
    }
}

impl Iterator for HexDirIter {
    type Item = HexDir;

    fn next(&mut self) -> Option<HexDir> {
        let result = match self.counter {
            0 => Some(HexDir::Top),
            1 => Some(HexDir::RightTop),
            2 => Some(HexDir::RightBot),
            3 => Some(HexDir::Bot),
            4 => Some(HexDir::LeftBot),
            5 => Some(HexDir::LeftTop),
            _ => None,
        };
        self.counter += 1;
        result
    }
}

#[derive(Clone, Debug)]
pub struct Hex {
    center: Vec2,
    size: Vec2,
}

#[derive(Debug)]
pub struct NeighborIterator {
    hex: Hex,
    dir_iter: HexDirIter,
}

impl NeighborIterator {
    fn new(hex: Hex) -> Self {
        NeighborIterator {
            hex,
            dir_iter: HexDirIter::new(),
        }
    }
}

impl Iterator for NeighborIterator {
    type Item = Hex;

    fn next(&mut self) -> Option<Hex> {
        match self.dir_iter.next() {
            Some(dir) => Some(self.hex.neighbor(dir)),
            None => None,
        }
    }
}

pub struct VertexIterator {
    hex: Hex,
    dir_iter: VertexDirIter,
}

impl VertexIterator {
    fn new(hex: Hex) -> Self {
        VertexIterator {
            hex,
            dir_iter: VertexDirIter::new(),
        }
    }
}

impl Iterator for VertexIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Vec2> {
        match self.dir_iter.next() {
            Some(dir) => Some(self.hex.vertex(dir)),
            None => None,
        }
    }
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        self.center_in_units() == other.center_in_units()
    }
}

impl Eq for Hex {}

impl Hash for Hex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center_in_units().hash(state);
    }
}

impl Hex {
    pub fn new(center: Vec2, size: Vec2) -> Self {
        Hex { center, size }
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    fn unit_size(&self) -> Vec2 {
        self.size * Vec2::new(0.75f32, 0.5f32)
    }

    fn center_in_units(&self) -> (i32, i32) {
        let (x, y) = (self.center / self.unit_size()).floor().into();
        (x as i32, y as i32)
    }

    pub fn hex_dir_to_vec(&self, dir: HexDir) -> Vec2 {
        let (_, h) = self.size.into();
        let (uw, uh) = self.unit_size().into();
        match dir {
            HexDir::Top => Vec2::new(0f32, h),
            HexDir::RightTop => Vec2::new(uw, uh),
            HexDir::RightBot => Vec2::new(uw, -uh),
            HexDir::Bot => Vec2::new(0f32, -h),
            HexDir::LeftBot => Vec2::new(-uw, -uh),
            HexDir::LeftTop => Vec2::new(-uw, uh),
        }
    }

    pub fn neighbor(&self, dir: HexDir) -> Self {
        Hex {
            center: self.center + self.hex_dir_to_vec(dir),
            size: self.size,
        }
    }

    pub fn neighbors(&self) -> NeighborIterator {
        NeighborIterator::new(self.clone())
    }

    pub fn top_hex(&self) -> Self {
        self.neighbor(HexDir::Top)
    }

    pub fn right_top_hex(&self) -> Self {
        self.neighbor(HexDir::RightTop)
    }

    pub fn right_bot_hex(&self) -> Self {
        self.neighbor(HexDir::RightBot)
    }

    pub fn bot_hex(&self) -> Self {
        self.neighbor(HexDir::Bot)
    }

    pub fn left_bot_hex(&self) -> Self {
        self.neighbor(HexDir::LeftBot)
    }

    pub fn left_top_hex(&self) -> Self {
        self.neighbor(HexDir::LeftTop)
    }

    pub fn vec_to_vertex(&self, dir: VertexDir) -> Vec2 {
        let (hw, hh) = (self.size / 2f32).into();
        let qw = hw / 2f32;

        match dir {
            VertexDir::LeftTop => Vec2::new(-qw, hh),
            VertexDir::RightTop => Vec2::new(qw, hh),
            VertexDir::Right => Vec2::new(hw, 0f32),
            VertexDir::RightBot => Vec2::new(qw, -hh),
            VertexDir::LeftBot => Vec2::new(-qw, -hh),
            VertexDir::Left => Vec2::new(-hw, 0f32),
        }
    }

    pub fn vertex(&self, dir: VertexDir) -> Vec2 {
        self.center + self.vec_to_vertex(dir)
    }

    pub fn vertices(&self) -> VertexIterator {
        VertexIterator::new(self.clone())
    }

    pub fn left_top_vert(&self) -> Vec2 {
        self.vertex(VertexDir::LeftTop)
    }

    pub fn right_top_vert(&self) -> Vec2 {
        self.vertex(VertexDir::RightTop)
    }

    pub fn right_vert(&self) -> Vec2 {
        self.vertex(VertexDir::Right)
    }

    pub fn right_bot_vert(&self) -> Vec2 {
        self.vertex(VertexDir::RightBot)
    }

    pub fn left_bot_vert(&self) -> Vec2 {
        self.vertex(VertexDir::LeftBot)
    }

    pub fn left_vert(&self) -> Vec2 {
        self.vertex(VertexDir::Left)
    }
}

#[cfg(test)]
mod tests {
    use super::Hex;
    use glam::Vec2;

    fn default_hex() -> Hex {
        let center = Vec2::new(1f32, 1f32);
        let size = Vec2::new(2f32, 1f32);
        Hex::new(center, size)
    }

    #[test]
    fn top() {
        let h = default_hex();
        assert_eq!(h.top_hex().left_bot_vert(), h.left_top_vert());
        assert_eq!(h.top_hex().right_bot_vert(), h.right_top_vert());
    }

    #[test]
    fn top_right() {
        let h = default_hex();
        assert_eq!(h.right_top_hex().left_bot_vert(), h.right_vert());
        assert_eq!(h.right_top_hex().left_vert(), h.right_top_vert());
    }

    #[test]
    fn bot_right() {
        let h = default_hex();
        assert_eq!(h.right_bot_hex().left_top_vert(), h.right_vert());
        assert_eq!(h.right_bot_hex().left_vert(), h.right_bot_vert());
    }

    #[test]
    fn bot() {
        let h = default_hex();
        assert_eq!(h.bot_hex().left_top_vert(), h.left_bot_vert());
        assert_eq!(h.bot_hex().right_top_vert(), h.right_bot_vert());
    }

    #[test]
    fn bot_left() {
        let h = default_hex();
        assert_eq!(h.left_bot_hex().right_vert(), h.left_bot_vert());
        assert_eq!(h.left_bot_hex().right_top_vert(), h.left_vert());
    }

    #[test]
    fn top_left() {
        let h = default_hex();
        assert_eq!(h.left_top_hex().right_vert(), h.left_top_vert());
        assert_eq!(h.left_top_hex().right_bot_vert(), h.left_vert());
    }
}
