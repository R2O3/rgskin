#![cfg(not(target_arch = "wasm32"))]

mod test_dependencies;
use rgskin::utils::io::join_paths_unix;
use test_dependencies::*;

#[test]
pub fn osu_to_fluxis_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let osu_skin = import::osu::skin_from_dir("./tests/skins/Whatsapp", false)?;
        let generic_skin = osu_skin.to_generic_mania(())?;
        let fluxis_from_generic = FluXisSkin::from_generic_mania(&generic_skin)?;
        export::fluxis::skin_to_dir(&fluxis_from_generic.0, SKIN_PATH)?;
        export::fluxis::layout_to_dir(&fluxis_from_generic.1, &join_paths_unix(SKIN_PATH, "layout.json"))?;
        Ok(())
    }, "Osu! mania to fluXis", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}