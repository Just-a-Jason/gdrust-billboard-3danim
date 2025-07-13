extern crate proc_macro;
mod attribute;
mod derive;

use proc_macro::TokenStream;

/// This macro derives BilboardAnimation for any enum.
/// It's required to work with `BilboardAnimator`
/// If you need to use it in Rust codebase without strong integration with GDScript you can use this macro. If you need to export it to godots engine and use it to export it in the inspector use `GodotAnimationEnum` attribute macro in that case.
/// Example usage:
/// ```
/// // use statement of the DirectionEnum...
///
/// #[derive(BilboardAnimation)]
/// enum AnimationStates {
///     Idle,
///     Walk
/// }
///
/// fn main() {
///     let animation = AnimationStates::Idle;
///     
///     assert_eq!(animation.rotate(Direction::Left), "idle_left");
/// }
/// ```
///
#[proc_macro_derive(BilboardAnimation)]
pub fn derive_bilboard_animation(input: TokenStream) -> TokenStream {
    derive::derive_bilboard_animation_impl(input)
}

/// This attribute macro converts any enum to `GodotEnum` and also derives `BilboardAnimation` trait.
/// IMPORTANT! - To use it you have to mark one of the enum members as `#[default]`
/// Example usage:
///
/// ```
///     #[GodotAnimationEnum]
///     pub enum AnimationStates {
///         #[default] // mark any member as default.
///         Idle,
///         Walk,
///         Jump
///     }
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GodotAnimationEnum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute::expand_godot_enum(item)
}

/// This macro converts a struct into full working animator in one line!
/// Creates new `GodotAnimatorClass` which implements `Animator` trait.
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GodotAnimatorClass(attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute::impl_godot_animation_class(attr, item)
}
