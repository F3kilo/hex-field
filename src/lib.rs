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
    fn new(config: Config) -> Self {
        HexField { config }
    }

    fn hex_at(self, x: f32, y: f32) -> Hex {
        Hex::new(self.config, 0, 0)
    }
}

#[derive(Clone, Debug)]
pub struct Hex {
    config: Config,
    x: i32,
    y: i32,
}

impl Hex {
    pub fn new(config: Config, x: i32, y: i32) -> Self {
        Hex { config, x, y }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
