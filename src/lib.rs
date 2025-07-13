mod tests;

#[cfg(feature = "macros")]
pub mod macros {
    pub use gdrust_billboard_3danim_derive::BillboardAnimation;
    pub use gdrust_billboard_3danim_derive::GodotAnimationEnum;
    pub use gdrust_billboard_3danim_derive::GodotAnimatorClass;
}

pub mod traits {
    pub use gdrust_billboard_3danim_core::traits::*;
}

pub mod enums {
    pub use gdrust_billboard_3danim_core::enums::*;
}

pub mod animators {
    pub use gdrust_billboard_3danim_core::animators::*;
}

pub mod core {
    pub use gdrust_billboard_3danim_core::core::*;
}

pub mod prelude;
