mod test_dependencies;
use test_dependencies::*;
use rgskin::{image_proc::{generate_stage_background, to_osu_column_draw}, utils::io::join_paths_unix, Binary, Store};

#[test]
fn osu_mania_receptor_draw_test() -> Result<(), Box<dyn std::error::Error>> {
    let img_bytes = read_file_to_bytes("./tests/assets/blooc.png")?;
    let texture = Texture::from_bytes("blooc".to_string(), &img_bytes)?;

    let mut textures = TextureStore::new();
    textures.insert(texture);

    let texture_ref = textures.get_shared("blooc").unwrap();

    benchmark_closure(||
    {
        to_osu_column_draw(&texture_ref, 40)?;
        Ok(())
    }, "Osu! mania receptor draw", "to finish processing", "\x1b[0;34m")?;

    write_bytes_to_file(join_paths_unix(ASSET_PATH, "blooc.png").as_str(), texture_ref.read().unwrap().to_bytes()?.as_slice())?;
    Ok(())
}

#[test]
fn osu_mania_generate_stages_test() -> Result<(), Box<dyn std::error::Error>> {
    let osu_skin = import::osu::skin_from_dir("./tests/skins/Whatsapp", false)?;
    
    benchmark_closure(||
    {
        for keymode in &osu_skin.skin_ini.keymodes {
            let av_column_width = keymode.column_width.iter().sum::<f32>() / keymode.column_width.len() as f32;
            let stage_img = generate_stage_background(keymode.colours.clone(), av_column_width as u32);
            stage_img.save_with_format(join_paths_unix(ASSET_PATH,
                &format!("stage-{}k.png", keymode.keymode)
            ).as_str(), image::ImageFormat::Png)?
        }
        Ok(())
    }, "Generating Osu! mania stages", "to finish", "\x1b[0;34m")?;

    Ok(())
}