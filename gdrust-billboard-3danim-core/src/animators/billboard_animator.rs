use godot::{
    classes::{AnimatedSprite3D, Camera3D},
    prelude::*,
};

use crate::{core::calculate_angle, traits::BillboardAnimation};
use crate::{enums::Direction, traits::Animator};

/// BillboardAnimator is a struct used to wrap AnimatedSprite3D.
/// It automatictly manages all the roation for you.
/// You just need to set the camera and your sprite and you are ready to go!
pub struct BillboardAnimator<A: BillboardAnimation + Copy> {
    // Godot objects
    sprite: Option<Gd<AnimatedSprite3D>>,
    camera: Option<Gd<Camera3D>>,

    // Animator inner state
    current_direction: Direction,
    current_animation: A,
    play_one_shot: bool,
    looping: bool,
    paused: bool,

    // Callbacks
    on_animation_finished_callback: Option<Box<dyn FnMut(A)>>,
}

// Assoc functions
impl<A: BillboardAnimation + Copy> BillboardAnimator<A> {
    pub fn new(default_animation: A) -> Self {
        BillboardAnimator {
            // Godot objects
            sprite: Option::default(),
            camera: Option::default(),

            // Animator inner state
            looping: bool::default(),
            play_one_shot: bool::default(),
            paused: bool::default(),
            current_direction: Direction::default(),
            current_animation: default_animation,
            on_animation_finished_callback: Option::default(),
        }
    }
}

// Private functions
impl<A: BillboardAnimation + Copy> BillboardAnimator<A> {
    fn update_animation(&mut self) {
        let sprite = self.sprite.as_mut().unwrap();

        // Compare if the same animation is playing already. If it does leave it as it is.
        // In other case just update the animation.
        {
            let animation_name = self.current_animation.rotate(self.current_direction);
            let animation = StringName::from(animation_name);
            let current = sprite.get_animation();

            if animation != current {
                sprite.set_animation(animation_name);
            }
        }

        if self.looping && !sprite.is_playing() {
            sprite.play();
        }
    }
}

// Public api
impl<A: BillboardAnimation + Copy> BillboardAnimator<A> {
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }

    pub fn set_camera(&mut self, camera: Gd<Camera3D>) {
        // Clone the cameras Gd<T> smartpointer it is cheap to clone.
        self.camera = Some(camera);
    }

    pub fn set_sprite(&mut self, sprite: Gd<AnimatedSprite3D>) {
        self.sprite = Some(sprite);
    }

    pub fn on_animation_finished<F>(&mut self, callback: F)
    where
        F: FnMut(A) + 'static,
    {
        self.on_animation_finished_callback = Some(Box::new(callback));
    }
}

// Animator trait impl
impl<A: BillboardAnimation + Copy> Animator<A> for BillboardAnimator<A> {
    fn update(&mut self) {
        if !self.is_setup() {
            godot_warn!(
                "BilboardAnimator must be fully setup before use. Consider using animator.set_camera() or animator.set_sprite() to fix that."
            );
            return;
        }

        if self.is_paused() {
            return;
        }

        // Calculate the facing direction.
        // Do not roate the sprite when the animation is playing.
        if !self.play_one_shot {
            let camera = self.camera.as_ref().unwrap();
            let sprite = self.sprite.as_mut().unwrap();
            self.current_direction = calculate_angle(camera, sprite);

            // Flip the Sprite if it is facing Right or Left direction.
            match self.current_direction {
                Direction::Right => {
                    sprite.set_flip_h(false);
                }
                Direction::Left => {
                    sprite.set_flip_h(true);
                }
                _ => (),
            }
        }

        if self.play_one_shot {
            let sprite = self.sprite.as_ref().unwrap();
            if !sprite.is_playing() {
                self.play_one_shot = false;

                if let Some(callback) = self.on_animation_finished_callback.as_mut() {
                    callback(self.current_animation);
                }

                return;
            }
        }

        // Update animation
        self.update_animation();
    }

    fn is_setup(&self) -> bool {
        self.camera.is_some() && self.sprite.is_some()
    }

    fn is_looping(&self) -> bool {
        self.looping
    }

    fn is_paused(&self) -> bool {
        self.paused
    }

    fn pause(&mut self) {
        // Do not try to pause the animator if it's already paused.
        if self.paused {
            return;
        }

        if let Some(sprite) = self.sprite.as_mut() {
            sprite.pause();
            self.paused = true;
        }
    }

    fn play(&mut self) {
        if let Some(sprite) = self.sprite.as_mut() {
            sprite.play();
            self.paused = bool::default();
        }
    }

    fn play_one_shot(&mut self, animation: A) {
        if let Some(sprite) = self.sprite.as_mut() {
            self.play_one_shot = true;
            self.current_animation = animation;

            if sprite.is_playing() {
                sprite.stop();
            }

            sprite.play();
        }
    }

    fn get_direction(&self) -> Direction {
        self.current_direction
    }

    fn get_animation(&self) -> &'static str {
        self.current_animation.rotate(self.current_direction)
    }

    fn change_animation(&mut self, animation: A) {
        self.current_animation = animation;
        if !self.paused {
            if let Some(sprite) = self.sprite.as_mut() {
                sprite.play();
            }
        }
    }
}
