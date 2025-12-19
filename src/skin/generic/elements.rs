use crate::{common::color::Rgba, io::texture::Texture, Binary};
use image::imageops::FilterType;
use std::sync::{Arc, RwLock};

pub trait SkinElement: Sync + Send {
    fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture>;
    fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture>;
    
    fn with_texture<F, R>(&self, f: F) -> R
    where 
        F: FnOnce(&Texture) -> R,
    {
        let texture = self.as_texture();
        f(&*texture)
    }
    
    fn with_texture_mut<F, R>(&self, f: F) -> R
    where 
        F: FnOnce(&mut Texture) -> R,
    {
        let mut texture = self.as_texture_mut();
        f(&mut *texture)
    }
    
    fn get_path(&self) -> String {
        self.with_texture(|texture| texture.path().to_string())
    }
    
    fn has_data(&self) -> bool {
        self.with_texture(|texture| texture.has_data())
    }
}

macro_rules! skin_element {
    ($name:ident) => {
        skin_element!($name; texture; );
    };
    
    ($name:ident; $primary:ident $(, $extra:ident)*) => {
        skin_element!($name; $primary $(, $extra)*; );
    };
    
    ($name:ident; $primary:ident $(, $extra:ident)*; $($attr:ident: $attr_type:ty),*) => {
        #[derive(Clone)]
        pub struct $name {
            pub $primary: Arc<RwLock<Texture>>,
            $(pub $extra: Arc<RwLock<Texture>>,)*
            $(pub $attr: $attr_type,)*
        }

        impl $name {
            pub fn new(
                $primary: Arc<RwLock<Texture>>
                $(, $extra: Arc<RwLock<Texture>>)*
                $(, $attr: $attr_type)*
            ) -> Self {
                Self { 
                    $primary 
                    $(, $extra)* 
                    $(, $attr)*
                }
            }
            
            pub fn with_texture_data(
                $primary: Texture 
                $(, $extra: Texture)*
                $(, $attr: $attr_type)*
            ) -> Self {
                Self {
                    $primary: Arc::new(RwLock::new($primary)),
                    $($extra: Arc::new(RwLock::new($extra)),)*
                    $( $attr)*
                }
            }
            
            pub fn from_path(
                $primary: String 
                $(, $extra: String)*
                $(, $attr: $attr_type)*
            ) -> Self {
                Self {
                    $primary: Arc::new(RwLock::new(Texture::new($primary))),
                    $($extra: Arc::new(RwLock::new(Texture::new($extra))),)*
                    $( $attr)*
                }
            }
            
            $(
                paste::paste! {
                    pub fn [<as_ $extra>](&self) -> std::sync::RwLockReadGuard<'_, Texture> {
                        self.$extra.read().unwrap()
                    }
                    
                    pub fn [<as_ $extra _mut>](&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
                        self.$extra.write().unwrap()
                    }
                    
                    pub fn [<with_ $extra>]<F, R>(&self, f: F) -> R
                    where 
                        F: FnOnce(&Texture) -> R,
                    {
                        let texture = self.[<as_ $extra>]();
                        f(&*texture)
                    }
                    
                    pub fn [<with_ $extra _mut>]<F, R>(&self, f: F) -> R
                    where 
                        F: FnOnce(&mut Texture) -> R,
                    {
                        let mut texture = self.[<as_ $extra _mut>]();
                        f(&mut *texture)
                    }
                }
            )*
            
            $(
                paste::paste! {
                    pub fn [<get_ $attr>](&self) -> &$attr_type {
                        &self.$attr
                    }
                    
                    pub fn [<set_ $attr>](&mut self, value: $attr_type) {
                        self.$attr = value;
                    }
                }
            )*
        }

        impl SkinElement for $name {
            fn as_texture(&self) -> std::sync::RwLockReadGuard<'_, Texture> {
                self.$primary.read().unwrap()
            }
            
            fn as_texture_mut(&self) -> std::sync::RwLockWriteGuard<'_, Texture> {
                self.$primary.write().unwrap()
            }
        }
    };
}

skin_element!(ReceptorUp);
skin_element!(ReceptorDown);
skin_element!(NormalNote);
skin_element!(LongNoteHead);
skin_element!(LongNoteTail);
skin_element!(LongNoteBody);
skin_element!(Healthbar; fill, background);
skin_element!(HitLighting; normal, hold);
skin_element!(ColumnLighting);
skin_element!(JudgementLine; texture; color: Rgba);
skin_element!(Stage; background, border_right, border_left);