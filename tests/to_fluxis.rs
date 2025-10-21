mod test_dependencies;
use rgc_skin::utils::io::join_paths_unix;
use test_dependencies::*;

#[test]
pub fn osu_to_fluxis_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let osu_skin = import::osu::skin_from_dir("./tests/skins/BubbleSkin")?;
        let generic_skin = load::osu::to_generic(osu_skin)?;
        let fluxis_from_generic = load::fluxis::from_generic(generic_skin)?;
        export::fluxis::skin_to_dir(&fluxis_from_generic.0.skin_json, Some(&fluxis_from_generic.0.textures), SKIN_PATH)?;
        export::fluxis::layout_to_dir(&fluxis_from_generic.1, &join_paths_unix(SKIN_PATH, "layout.json"))?;
        Ok(())
    }, "Osu! mania to fluXis", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}