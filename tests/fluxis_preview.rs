#![cfg(not(target_arch = "wasm32"))]

mod test_dependencies;
use rgskin::{image_proc::generate_fluxis_preview, utils::io::join_paths_unix};
use test_dependencies::*;

#[test]
pub fn fluxis_to_osu_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let osu_skin = import::osu::skin_from_dir("./tests/skins/Pl0x", false)?;
        let generic_skin = osu_skin.to_generic_mania(())?;
        let (fluxis_skin, _) = FluXisSkin::from_generic_mania(&generic_skin)?;
        
        let img = generate_fluxis_preview(&fluxis_skin.skin_json, &fluxis_skin.textures, 1024, 1024)?;
        img.save_with_format(join_paths_unix(ASSET_PATH, "preview.png"), image::ImageFormat::Png)?;
        Ok(())
    }, "fluXis to Osu! mania", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}