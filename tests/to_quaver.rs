#![cfg(not(target_arch = "wasm32"))]

mod test_dependencies;
use test_dependencies::*;

#[test]
pub fn osu_to_quaver_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let osu_skin = import::osu::skin_from_dir("./tests/skins/Nanachi", false)?;
        let generic_skin = osu_skin.to_generic_mania(())?;
        let quaver_from_generic = QuaSkin::from_generic_mania(&generic_skin)?;
        export::quaver::skin_to_dir(&quaver_from_generic, SKIN_PATH)?;
        Ok(())
    }, "Osu! mania to Quaver", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}

#[test]
pub fn fluxis_to_quaver_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let fluxis_skin = import::fluxis::skin_from_dir("./tests/skins/KoriPick", false)?;
        let generic_skin = fluxis_skin.to_generic_mania(None)?;
        let quaver_from_generic = QuaSkin::from_generic_mania(&generic_skin)?;
        export::quaver::skin_to_dir(&quaver_from_generic, SKIN_PATH)?;
        Ok(())
    }, "fluXis to Quaver", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}
