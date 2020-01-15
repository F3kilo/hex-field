use glam::Vec2;

#[derive(Clone, Debug)]
pub struct UnitBlock {
    x: i32,
    y: i32,
}

impl UnitBlock {
    pub fn at(point: Vec2) -> Self {
        UnitBlock {
            x: point.x().floor() as i32,
            y: point.y().floor() as i32,
        }
    }

    pub fn origin(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn right_block(&self) -> Self {
        UnitBlock {
            x: self.x + 1,
            y: self.y,
        }
    }

    // pub fn left_block(&self) -> Self {
    //     UnitBlock {
    //         x: self.x - 1,
    //         y: self.y,
    //     }
    // }

    pub fn top_block(&self) -> Self {
        UnitBlock {
            x: self.x,
            y: self.y + 1,
        }
    }

    // pub fn bot_block(&self) -> Self {
    //     UnitBlock {
    //         x: self.x,
    //         y: self.y - 1,
    //     }
    // }

    pub fn central(&self) -> bool {
        ((self.x + self.y) % 2) == 0
    }

    pub fn point_in_left_part(&self, p: Vec2) -> bool {
        let (x, y) = self.to_unit_local(p).into();
        let abs_k = 3f32;
        let bc = 2f32;
        let bnc = -1f32;
        if self.central() {
            y < -abs_k * x + bc
        } else {
            y > abs_k * x + bnc
        }
    }

    fn to_unit_local(&self, p: Vec2) -> Vec2 {
        p - self.origin()
    }
}

#[cfg(test)]
mod tests {
    use super::UnitBlock;
    use glam::Vec2;

    fn init_test_unit_block() -> UnitBlock {
        UnitBlock::at(Vec2::new(3.2f32, 2.8f32))
    }

    fn init_negative_test_unit_block() -> UnitBlock {
        UnitBlock::at(Vec2::new(-2.2f32, -2.8f32))
    }

    #[test]
    fn origin() {
        let ub = init_test_unit_block();
        assert_eq!(ub.origin(), Vec2::new(3f32, 2f32));

        let nub = init_negative_test_unit_block();
        assert_eq!(nub.origin(), Vec2::new(-3f32, -3f32));
    }

    #[test]
    fn right_block() {
        let ub = init_test_unit_block();
        assert_eq!(ub.right_block().origin(), Vec2::new(4f32, 2f32));

        let nub = init_negative_test_unit_block();
        assert_eq!(nub.right_block().origin(), Vec2::new(-2f32, -3f32));
    }

    #[test]
    fn top_block() {
        let ub = init_test_unit_block();
        assert_eq!(ub.top_block().origin(), Vec2::new(3f32, 3f32));

        let nub = init_negative_test_unit_block();
        assert_eq!(nub.top_block().origin(), Vec2::new(-3f32, -2f32));
    }

    #[test]
    fn central() {
        let ub = init_test_unit_block();
        assert!(!ub.central());

        let nub = init_negative_test_unit_block();
        assert!(nub.central());
    }

    #[test]
    fn point_in_left_part() {
        let ub = init_test_unit_block();
        let p = Vec2::new(3.5f32, 2.8f32);
        assert!(ub.point_in_left_part(p));

        let p = Vec2::new(3.5f32, 2.2f32);
        assert!(!ub.point_in_left_part(p));

        let nub = init_negative_test_unit_block();
        let p = Vec2::new(-2.5f32, -2.8f32);
        assert!(nub.point_in_left_part(p));

        let p = Vec2::new(-2.5f32, -2.2f32);
        assert!(!nub.point_in_left_part(p));
    }
}
