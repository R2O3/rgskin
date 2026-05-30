#[macro_export]
macro_rules! impl_skin_importer {
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, filtered) => {
        pub fn $fn_name(
            path: &str,
            import_all: bool,
        ) -> Result<$skin_type, Box<dyn std::error::Error>> {
            let config_content = read_str_from_path(&node::join_path(path, $config_file));
            let config = <$config_type>::from_str(&config_content)?;
            let texture_paths = config.get_required_texture_paths();
            let sample_paths  = config.get_required_sample_paths();
            let textures = if import_all {
                import_all_textures_from_dir(path, Some(&texture_paths))?
            } else {
                import_textures_from_dir(path, &texture_paths)?
            };
            let samples = if import_all {
                import_all_samples_from_dir(path)?
            } else {
                import_samples_from_dir(path, &sample_paths)?
            };
            Ok(<$skin_type>::new(config, Some(textures), Some(samples)))
        }
    };
    ($fn_name:ident, $config_file:expr, $config_type:ty, $skin_type:ty, all) => {
        pub fn $fn_name(
            path: &str,
            import_all: bool,
        ) -> Result<$skin_type, Box<dyn std::error::Error>> {
            let config_content = read_str_from_path(&node::join_path(path, $config_file));
            let config = <$config_type>::from_str(&config_content)?;
            let sample_paths = config.get_required_sample_paths();
            let textures = import_all_textures_from_dir(path, None)?;
            let samples = if import_all {
                import_all_samples_from_dir(path)?
            } else {
                import_samples_from_dir(path, &sample_paths)?
            };
            Ok(<$skin_type>::new(config, Some(textures), Some(samples)))
        }
    };
}
