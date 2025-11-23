use crate::io::Texture;
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
    
    fn path(&self) -> String {
        self.with_texture(|texture| texture.path().to_string())
    }
    
    fn has_data(&self) -> bool {
        self.with_texture(|texture| texture.has_data())
    }
}

macro_rules! skin_element {
    ($name:ident) => {
        skin_element!($name; texture);
    };
    
    ($name:ident; $primary:ident $(, $extra:ident)*) => {
        #[derive(Clone)]
        pub struct $name {
            pub $primary: Arc<RwLock<Texture>>,
            $(pub $extra: Arc<RwLock<Texture>>,)*
        }

        impl $name {
            pub fn new($primary: Arc<RwLock<Texture>> $(, $extra: Arc<RwLock<Texture>>)*) -> Self {
                Self { $primary $(, $extra)* }
            }
            
            pub fn with_texture_data($primary: Texture $(, $extra: Texture)*) -> Self {
                Self {
                    $primary: Arc::new(RwLock::new($primary)),
                    $($extra: Arc::new(RwLock::new($extra)),)*
                }
            }
            
            pub fn from_path($primary: String $(, $extra: String)*) -> Self {
                Self {
                    $primary: Arc::new(RwLock::new(Texture::new($primary))),
                    $($extra: Arc::new(RwLock::new(Texture::new($extra))),)*
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