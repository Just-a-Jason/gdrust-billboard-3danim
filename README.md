![Image](https://github.com/Just-a-Jason/gdrust-bilboard-3danim/blob/main/preview.gif)

`gdrust-bilboard-3danim` is a Rust library for animating 3D sprites based on the camera's viewing direction—similar to the classic Doom-style visuals. It allows for smooth sprite angle switching and animation control depending on where the player is looking.

# Animator trait

```rs
pub trait Animator<A> where A: BillboardAnimation, {
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

```

> [!IMPORTANT]
> This crate was tested in godot environment with a few characters like this one on the preview bellow it's very powerfull and very fast.

![PREVIEW](https://github.com/Just-a-Jason/gdrust-sprite3d-angle-animator/blob/main/preview.gif)

> [!WARNING] > `To be able to use it you have to add all animation variants like for example by using this derive macro to generate all animation names at compile time`

```rs
use gdrust_billboard_3danim::prelude::*;

#[GodotAnimationEnum]
enum MyAnimationStates {
    #[default]
    Idle, // Mark this member as the default animation
    Walk
}
```

```rs
use gdrust_billboard_3danim::prelude::*;
use godot::prelude::*;

// This attribute macro generates a fully working Bilboard animator class for you.
#[GodotAnimatorClass(MyAnimationStates)]
pub struct MyAnimatorClass;
```

> [!WARNING]
> To use `GodotAnimationEnum` and `GodotAnimatorClass` macros you need to enable `macros` feature in your `Cargo.toml` file.

> [!WARNING]
> You have to create all of those animations on your `3DAnimatedSprite` node.
>
> - `idle_front`
> - `idle_side`
> - `idle_back`
> - `walk_front`
> - `walk_side`
> - `walk_back` \
>   The derive macro `SidedAnimation` compiles `&'static str` reference to all of your animations with direction prefix.
>   The `Left/Right` direction compiles to => `{your animation name}_side` and then the `Animator` struct flips it.
