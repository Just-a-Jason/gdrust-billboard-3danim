use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ItemStruct, Meta, MetaNameValue, parse::Parser, parse_macro_input,
    punctuated::Punctuated, token::Comma,
};

pub fn impl_godot_animation_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();

    let args = Punctuated::<Meta, Comma>::parse_terminated
        .parse(attr)
        .expect("Failed to parse attribute args");

    let mut animation_enum = None;

    for meta in args {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if path.is_ident("animation") {
                if let Expr::Lit(expr_lit) = value {
                    if let syn::Lit::Str(lit_str) = expr_lit.lit {
                        animation_enum = Some(lit_str.value());
                    }
                }
            }
        }
    }

    let animation_enum =
        animation_enum.expect("Expected #[animation = \"BilboardAnimation Enum\"]");
    let animation_ident = syn::Ident::new(&animation_enum, proc_macro2::Span::call_site());
    // To change that in the future
    let expanded = quote! {
        use godot::prelude::*;

        #[derive(GodotClass)]
        #[class(base=Node3D)]
        pub struct #struct_name {
            #[export]
            loop_animation: bool,

            animator: gdrust_billboard_3danim::animators::BillboardAnimator<#animation_ident>,
            base: Base<Node3D>,
        }

        #[godot_api]
        impl INode3D for #struct_name {
            fn init(base: Base<Self::Base>) -> Self {
                Self {
                    animator: gdrust_billboard_3danim::animators::BillboardAnimator::new(#animation_ident::default()),
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
            pub fn change_animation(&mut self, animation: #animation_ident) {
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
            pub fn play_one_shot(&mut self, animation: #animation_ident) {
                self.animator.play_one_shot(animation);
            }

            #[func]
            pub fn is_paused(&self) -> bool {
                self.animator.is_paused()
            }

            #[signal]
            pub fn on_animation_finished(animation: #animation_ident);
        }
    };

    TokenStream::from(expanded)
}
