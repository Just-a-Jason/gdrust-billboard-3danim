use crate::enums::Direction;

/// ```
/// use gdrust_bilboard_3danim_core::enums::Direction;
/// use gdrust_bilboard_3danim_core::traits::BilboardAnimation;
///
/// enum States {
///     Idle,
/// }
///
/// impl BilboardAnimation for States {
///     fn rotate(&self, direction: Direction) -> &'static str {
///         match self {
///             States::Idle => match direction {
///                 Direction::Front => "idle_front",
///                 Direction::Left | Direction::Right => "idle_side",
///                 Direction::Back => "idle_back",
///             },
///         }
///     }
/// }
///
/// let animation = States::Idle;
/// assert_eq!("idle_front", animation.rotate(Direction::Front));
/// ```
pub trait BilboardAnimation {
    fn rotate(&self, direction: Direction) -> &'static str;
}
