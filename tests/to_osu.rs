mod test_dependencies;
use test_dependencies::*;

#[test]
pub fn fluxis_to_osu_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let fluxis_skin = import::fluxis::skin_from_dir("./tests/skins/KoriPick")?;
        let generic_skin = load::fluxis::to_generic(&fluxis_skin, None)?;
        let osu_from_generic = load::osu::from_generic(&generic_skin)?;
        export::osu::skin_to_dir(&osu_from_generic, SKIN_PATH)?;
        Ok(())
    }, "fluXis to Osu! mania", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}