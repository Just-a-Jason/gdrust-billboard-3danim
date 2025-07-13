#[cfg(test)]
mod test {
    use crate::prelude::*;
    use godot::prelude::*;

    #[GodotAnimationEnum]
    enum CharacterAnimations {
        #[default]
        Idle,
        Walk,
        Crawl,
    }

    #[test]
    fn test_animation_rotation() {
        let animation = CharacterAnimations::Idle;

        assert_eq!(animation.rotate(Direction::Front), "idle_front");
        assert_eq!(animation.rotate(Direction::Back), "idle_back");
        assert_eq!(animation.rotate(Direction::Left), "idle_side");
        assert_eq!(animation.rotate(Direction::Right), "idle_side");
    }
}
