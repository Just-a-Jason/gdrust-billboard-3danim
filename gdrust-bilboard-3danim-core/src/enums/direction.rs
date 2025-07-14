/// Represents all 4-directions of the sprite.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Front,
    Back,
    Left,
    Right,
}
