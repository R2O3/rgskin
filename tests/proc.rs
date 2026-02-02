mod test_dependencies;
use test_dependencies::*;
use rgskin::{image_proc::proc::to_osu_column_draw, {Binary, Store}, utils::io::join_paths_unix};

#[test]
fn osu_mania_receptor_draw() -> Result<(), Box<dyn std::error::Error>> {
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