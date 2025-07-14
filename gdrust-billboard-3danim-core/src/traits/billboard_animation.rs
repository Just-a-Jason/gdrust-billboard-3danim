/// ```
/// use gdrust_billboard_3danim_core::enums::Direction;
/// use gdrust_billboard_3danim_core::traits::BillboardAnimation;
///
/// enum States {
///     Idle,
/// }
///
/// impl BillboardAnimation for States {
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
pub trait BillboardAnimation {
    fn rotate(&self, direction: crate::enums::Direction) -> &'static str;
}
