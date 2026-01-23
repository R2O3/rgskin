#![cfg(not(target_arch = "wasm32"))]

mod test_dependencies;
use test_dependencies::*;

#[test]
pub fn fluxis_to_osu_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let fluxis_skin = import::fluxis::skin_from_dir("./tests/skins/KoriPick", false)?;
        let generic_skin = fluxis_skin.to_generic_mania(None)?;
        let osu_from_generic = OsuSkin::from_generic_mania(&generic_skin)?;
        export::osu::skin_to_dir(&osu_from_generic, SKIN_PATH)?;
        Ok(())
    }, "fluXis to Osu! mania", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}