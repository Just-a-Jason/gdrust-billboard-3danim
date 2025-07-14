use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Path, parse_macro_input, punctuated::Punctuated, token::Comma};

pub fn impl_godot_animation_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();

    let paths = parse_macro_input!(attr with Punctuated::<Path, Comma>::parse_terminated);

    let animation_path = paths
        .first()
        .expect("Expected #[GodotAnimatorClass(AnimationEnum)]")
        .clone();

    let expanded = quote! {
        use godot::prelude::*;

        #[derive(GodotClass)]
        #[class(base=Node3D)]
        pub struct #struct_name {
            #[export]
            loop_animation: bool,

            animator: gdrust_billboard_3danim::animators::BillboardAnimator<#animation_path>,
            base: Base<Node3D>,
        }

        #[godot_api]
        impl INode3D for #struct_name {
            fn init(base: Base<Self::Base>) -> Self {
                Self {
                    animator: gdrust_billboard_3danim::animators::BillboardAnimator::new(#animation_path::default()),
                    loop_animation: bool::default(),
                    base,
                }
            }

            fn ready(&mut self) {
                self.animator.set_looping(self.loop_animation);

                let mut emitter = self.base().clone();
                self.animator.on_animation_finished(move |animation| {
                    let _ = emitter.emit_signal("on_animation_finished", &[animation.to_variant()]);
                });
            }
        }

        #[godot_api]
        impl #struct_name {
            #[func]
            pub fn is_set_up(&self) -> bool {
                self.animator.is_setup()
            }

            #[func]
            pub fn change_animation(&mut self, animation: #animation_path) {
                self.animator.change_animation(animation);
            }

            #[func]
            pub fn set_looping(&mut self, looping: bool) {
                self.animator.set_looping(looping);
            }

            #[func]
            pub fn get_looping(&self) -> bool {
                self.animator.is_looping()
            }

            #[func]
            pub fn toggle_looping(&mut self) {
                let looping = !self.animator.is_looping();
                self.animator.set_looping(looping);
            }

            #[func]
            pub fn set_camera(&mut self, camera: Gd<godot::classes::Camera3D>) {
                self.animator.set_camera(camera);
            }

            #[func]
            pub fn set_sprite(&mut self, sprite: Gd<godot::classes::AnimatedSprite3D>) {
                self.animator.set_sprite(sprite);
            }

            #[func]
            pub fn update(&mut self) {
                self.animator.update();
            }

            #[func]
            pub fn pause(&mut self) {
                self.animator.pause();
            }

            #[func]
            pub fn play(&mut self) {
                self.animator.play();
            }

            #[func]
            pub fn play_one_shot(&mut self, animation: #animation_path) {
                self.animator.play_one_shot(animation);
            }

            #[func]
            pub fn is_paused(&self) -> bool {
                self.animator.is_paused()
            }

            #[signal]
            pub fn on_animation_finished(animation: #animation_path);
        }
    };

    TokenStream::from(expanded)
}
