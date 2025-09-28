mod test_dependencies;
use test_dependencies::*;

#[test]
pub fn to_osu_test() -> Result<(), Box<dyn std::error::Error>> {

    benchmark_closure(||
    {
        let osu_skin = import::osu::skin_from_dir("./tests/skins/BubbleSkin")?;
        let generic_skin = load::osu::to_generic(osu_skin)?;
        let osu_from_generic = load::osu::from_generic(generic_skin)?;
        export::osu::skin_to_dir(&osu_from_generic.skin_ini, Some(&osu_from_generic.textures), SKIN_PATH)?;
        Ok(())
    }, "Osu! mania to Osu! mania", "to finish converting", "\x1b[0;32m")?;
    
    Ok(())
}