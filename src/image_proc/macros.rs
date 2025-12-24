#[macro_export]
macro_rules! process_texture {
    ($texture:expr, $processor:expr) => {
        {
            use crate::Binary;
            let processed_image = {
                let mut write_guard = $texture.write().unwrap();
                if let Some(img) = write_guard.take_data() {
                    $processor(img)
                } else {
                    return Err("No texture data available".into());
                }
            };
            
            let mut write_guard = $texture.write().unwrap();
            write_guard.set_data(processed_image);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! process_texture_mut {
    ($texture:expr, $processor:expr) => {
        {
            use crate::BinaryState;
            let mut write_guard = $texture.write().unwrap();
            if let BinaryState::Loaded(ref mut img) = write_guard.data {
                $processor(img);
            }
        }
    };
}
