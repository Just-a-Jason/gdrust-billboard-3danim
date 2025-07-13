use crate::enums::Direction;
use crate::traits::BillboardAnimation;

pub trait Animator<A>
where
    A: BillboardAnimation,
{
    // Updates animator state
    fn update(&mut self);
    fn change_animation(&mut self, animation: A);

    // Get
    fn is_setup(&self) -> bool;
    fn is_paused(&self) -> bool;
    fn is_looping(&self) -> bool;

    // Cheap to copy pratictly a u8
    fn get_direction(&self) -> Direction;
    fn get_animation(&self) -> &'static str;

    // Functions to play, pause or play an animation one shot.
    fn play_one_shot(&mut self, animation: A);
    fn play(&mut self);
    fn pause(&mut self);
}
