use rgskin_derive::GetAllTextures;
use crate::{common::color::Rgba, io::texture::Texture, Binary};
use image::imageops::FilterType;
use std::sync::{Arc, RwLock};

pub trait SkinElement: Sync + Send {
    fn as_texture(&self) -> Option<std::sync::RwLockReadGuard<'_, Texture>>;
    fn as_texture_mut(&self) -> Option<std::sync::RwLockWriteGuard<'_, Texture>>;
    
    fn with_texture<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&Texture) -> R,
    {
        let texture = self.as_texture()?;
        Some(f(&*texture))
    }
    
    fn with_texture_mut<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&mut Texture) -> R,
    {
        let mut texture = self.as_texture_mut()?;
        Some(f(&mut *texture))
    }
    
    fn get_path(&self) -> Option<String> {
        self.with_texture(|texture| texture.get_path().to_string())
    }
    
    fn has_data(&self) -> bool {
        self.with_texture(|texture| texture.has_data()).unwrap_or(false)
    }

    fn is_sheet(&self) -> bool { false }
    fn is_animatable(&self) -> bool { false }
    
    fn get_fps(&self) -> Option<f32> { None }
    fn set_fps(&mut self, _fps: Option<f32>) {}
    
    fn get_columns(&self) -> Option<u32> { None }
    fn set_columns(&mut self, _columns: Option<u32>) {}
    
    fn get_rows(&self) -> Option<u32> { None }
    fn set_rows(&mut self, _rows: Option<u32>) {}
    
    fn frame_count(&self) -> usize {
        let cols = self.get_columns().unwrap_or(1) as usize;
        let rows = self.get_rows().unwrap_or(1) as usize;
        if cols * rows > 1 {
            cols * rows
        } else if self.has_data() {
            1
        } else {
            0
        }
    }

    fn as_texture_frame(&self, index: usize) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
        let is_atlas = self.get_columns().unwrap_or(1) * self.get_rows().unwrap_or(1) > 1;
        if is_atlas || index == 0 { self.as_texture() } else { None }
    }

    fn as_texture_frame_mut(&self, index: usize) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
        let is_atlas = self.get_columns().unwrap_or(1) * self.get_rows().unwrap_or(1) > 1;
        if is_atlas || index == 0 { self.as_texture_mut() } else { None }
    }

    fn with_texture_frame<F, R>(&self, index: usize, f: F) -> Option<R>
    where 
        F: FnOnce(&Texture) -> R,
    {
        let texture = self.as_texture_frame(index)?;
        Some(f(&*texture))
    }

    fn with_texture_frame_mut<F, R>(&self, index: usize, f: F) -> Option<R>
    where 
        F: FnOnce(&mut Texture) -> R,
    {
        let mut texture = self.as_texture_frame_mut(index)?;
        Some(f(&mut *texture))
    }

    fn as_texture_frames(&self) -> Vec<std::sync::RwLockReadGuard<'_, Texture>> {
        (0..self.frame_count())
            .filter_map(|i| self.as_texture_frame(i))
            .collect()
    }

    fn as_texture_frames_mut(&self) -> Vec<std::sync::RwLockWriteGuard<'_, Texture>> {
        (0..self.frame_count())
            .filter_map(|i| self.as_texture_frame_mut(i))
            .collect()
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
        #[derive(Clone, GetAllTextures)]
        pub struct $name {
            pub $primary: Option<Arc<RwLock<Texture>>>,
            $(pub $extra: Option<Arc<RwLock<Texture>>>,)*
            $(pub $attr: $attr_type,)*
        }

        impl $name {
            pub fn new(
                $primary: Option<Arc<RwLock<Texture>>>
                $(, $extra: Option<Arc<RwLock<Texture>>>)*
                $(, $attr: $attr_type)*
            ) -> Self {
                Self { 
                    $primary 
                    $(, $extra)* 
                    $(, $attr)*
                }
            }
            
            pub fn with_texture_data(
                $primary: Option<Texture>
                $(, $extra: Option<Texture>)*
                $(, $attr: $attr_type)*
            ) -> Self {
                Self {
                    $primary: $primary.map(|t| Arc::new(RwLock::new(t))),
                    $($extra: $extra.map(|t| Arc::new(RwLock::new(t))),)*
                    $($attr,)*
                }
            }

            pub fn from_path(
                $primary: Option<String>
                $(, $extra: Option<String>)*
                $(, $attr: $attr_type)*
            ) -> Self {
                Self {
                    $primary: $primary.map(|p| Arc::new(RwLock::new(Texture::new(p)))),
                    $($extra: $extra.map(|p| Arc::new(RwLock::new(Texture::new(p)))),)*
                    $($attr,)*
                }
            }
            
            $(
                paste::paste! {
                    pub fn [<as_ $extra>](&self) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
                        self.$extra.as_ref().map(|t| t.read().unwrap())
                    }
                    
                    pub fn [<as_ $extra _mut>](&self) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
                        self.$extra.as_ref().map(|t| t.write().unwrap())
                    }
                    
                    pub fn [<with_ $extra>]<F, R>(&self, f: F) -> Option<R>
                    where 
                        F: FnOnce(&Texture) -> R,
                    {
                        let texture = self.[<as_ $extra>]()?;
                        Some(f(&*texture))
                    }
                    
                    pub fn [<with_ $extra _mut>]<F, R>(&self, f: F) -> Option<R>
                    where 
                        F: FnOnce(&mut Texture) -> R,
                    {
                        let mut texture = self.[<as_ $extra _mut>]()?;
                        Some(f(&mut *texture))
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
            fn as_texture(&self) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
                self.$primary.as_ref().map(|t| t.read().unwrap())
            }
            
            fn as_texture_mut(&self) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
                self.$primary.as_ref().map(|t| t.write().unwrap())
            }
        }
    };
}

macro_rules! skin_element_sheet {
    ($name:ident) => {
        skin_element_sheet!($name; frames; );
    };
    
    ($name:ident; $primary:ident $(, $extra:ident)*) => {
        skin_element_sheet!($name; $primary $(, $extra)*; );
    };
    
    ($name:ident; $primary:ident $(, $extra:ident)*; $($attr:ident: $attr_type:ty),*) => {
        #[derive(Clone, GetAllTextures)]
        pub struct $name {
            pub $primary: Vec<Arc<RwLock<Texture>>>,
            $(pub $extra: Vec<Arc<RwLock<Texture>>>,)*
            pub fps: Option<f32>,
            pub columns: Option<u32>,
            pub rows: Option<u32>,
            $(pub $attr: $attr_type,)*
        }

        impl $name {
            pub fn new(
                $primary: Vec<Arc<RwLock<Texture>>>
                $(, $extra: Vec<Arc<RwLock<Texture>>>)*
                , fps: Option<f32>
                , columns: Option<u32>
                , rows: Option<u32>
                $(, $attr: $attr_type)*
            ) -> Self {
                Self { 
                    $primary 
                    $(, $extra)* 
                    , fps
                    , columns
                    , rows
                    $(, $attr)*
                }
            }
            
            pub fn with_texture_data(
                $primary: Vec<Texture>
                $(, $extra: Vec<Texture>)*
                , fps: Option<f32>
                , columns: Option<u32>
                , rows: Option<u32>
                $(, $attr: $attr_type)*
            ) -> Self {
                Self {
                    $primary: $primary.into_iter().map(|t| Arc::new(RwLock::new(t))).collect(),
                    $($extra: $extra.into_iter().map(|t| Arc::new(RwLock::new(t))).collect(),)*
                    fps,
                    columns,
                    rows,
                    $($attr,)*
                }
            }

            pub fn from_paths(
                $primary: Vec<String>
                $(, $extra: Vec<String>)*
                , fps: Option<f32>
                , columns: Option<u32>
                , rows: Option<u32>
                $(, $attr: $attr_type)*
            ) -> Self {
                Self {
                    $primary: $primary.into_iter().map(|p| Arc::new(RwLock::new(Texture::new(p)))).collect(),
                    $($extra: $extra.into_iter().map(|p| Arc::new(RwLock::new(Texture::new(p)))).collect(),)*
                    fps,
                    columns,
                    rows,
                    $($attr,)*
                }
            }
            
            $(
                paste::paste! {
                    pub fn [<as_ $extra _frame>](&self, index: usize) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
                        if self.$extra.len() <= 1 {
                            self.$extra.first().map(|t| t.read().unwrap())
                        } else {
                            self.$extra.get(index).map(|t| t.read().unwrap())
                        }
                    }
                    
                    pub fn [<as_ $extra _frame_mut>](&self, index: usize) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
                        if self.$extra.len() <= 1 {
                            self.$extra.first().map(|t| t.write().unwrap())
                        } else {
                            self.$extra.get(index).map(|t| t.write().unwrap())
                        }
                    }
                    
                    pub fn [<as_ $extra _frames>](&self) -> Vec<std::sync::RwLockReadGuard<'_, Texture>> {
                        self.$extra.iter().map(|t| t.read().unwrap()).collect()
                    }
                    
                    pub fn [<as_ $extra _frames_mut>](&self) -> Vec<std::sync::RwLockWriteGuard<'_, Texture>> {
                        self.$extra.iter().map(|t| t.write().unwrap()).collect()
                    }
                    
                    pub fn [<with_ $extra _frame>]<F, R>(&self, index: usize, f: F) -> Option<R>
                    where 
                        F: FnOnce(&Texture) -> R,
                    {
                        let texture = self.[<as_ $extra _frame>](index)?;
                        Some(f(&*texture))
                    }
                    
                    pub fn [<with_ $extra _frame_mut>]<F, R>(&self, index: usize, f: F) -> Option<R>
                    where 
                        F: FnOnce(&mut Texture) -> R,
                    {
                        let mut texture = self.[<as_ $extra _frame_mut>](index)?;
                        Some(f(&mut *texture))
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
            fn as_texture(&self) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
                self.$primary.first().map(|t| t.read().unwrap())
            }
            
            fn as_texture_mut(&self) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
                self.$primary.first().map(|t| t.write().unwrap())
            }

            fn is_sheet(&self) -> bool { true }
            fn is_animatable(&self) -> bool { self.fps.is_some() }
            
            fn get_fps(&self) -> Option<f32> { self.fps }
            fn set_fps(&mut self, fps: Option<f32>) { self.fps = fps; }
            
            fn get_columns(&self) -> Option<u32> { 
                self.columns.or(Some(self.$primary.len() as u32)) 
            }
            fn set_columns(&mut self, columns: Option<u32>) { self.columns = columns; }
            
            fn get_rows(&self) -> Option<u32> { 
                self.rows.or(Some(1)) 
            }
            fn set_rows(&mut self, rows: Option<u32>) { self.rows = rows; }
            
            fn frame_count(&self) -> usize {
                (self.get_columns().unwrap_or(1) * self.get_rows().unwrap_or(1)) as usize
            }
            
            fn as_texture_frame(&self, index: usize) -> Option<std::sync::RwLockReadGuard<'_, Texture>> {
                if self.$primary.len() <= 1 {
                    self.$primary.first().map(|t| t.read().unwrap())
                } else {
                    self.$primary.get(index).map(|t| t.read().unwrap())
                }
            }
            
            fn as_texture_frame_mut(&self, index: usize) -> Option<std::sync::RwLockWriteGuard<'_, Texture>> {
                if self.$primary.len() <= 1 {
                    self.$primary.first().map(|t| t.write().unwrap())
                } else {
                    self.$primary.get(index).map(|t| t.write().unwrap())
                }
            }

            fn as_texture_frames(&self) -> Vec<std::sync::RwLockReadGuard<'_, Texture>> {
                self.$primary.iter().map(|t| t.read().unwrap()).collect()
            }

            fn as_texture_frames_mut(&self) -> Vec<std::sync::RwLockWriteGuard<'_, Texture>> {
                self.$primary.iter().map(|t| t.write().unwrap()).collect()
            }
        }
    };
}

skin_element!(ReceptorUp);
skin_element!(ReceptorDown);
skin_element!(NormalNote);
skin_element!(BaseNormalNote);
skin_element_sheet!(NormalNotesSnapColored; snaps; colors: Vec<Rgba>);
skin_element!(LongNoteHead);
skin_element!(BaseHoldHead);
skin_element_sheet!(LongNoteHeadsSnapColored; snaps; colors: Vec<Rgba>);
skin_element!(LongNoteTail);
skin_element!(LongNoteBody);
skin_element!(Healthbar; fill, background);
skin_element_sheet!(HitLightingNormal);
skin_element_sheet!(HitLightingHold);
skin_element!(ColumnLighting);
skin_element!(JudgementLine; texture; color: Rgba);
skin_element!(Judgement; flawless, perfect, great, good, bad, miss);
skin_element!(Stage; background, border_right, border_left);
skin_element!(Cursor; texture; centered: bool, rotate: bool);