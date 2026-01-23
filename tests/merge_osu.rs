mod test_dependencies;
use test_dependencies::*;

#[test]
pub fn fluxis_to_osu_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut s1 = import::osu::skin_from_dir("./tests/skins/BubbleSkin", false)?;
    let s2 = import::osu::skin_from_dir("./tests/skins/CrewKMix", false)?;

    benchmark_closure(||
    {
        s1.merge(s2.clone());
        export::osu::skin_to_dir(&s1, SKIN_PATH)?;
        Ok(())
    }, "Merging Two Osu Skins", "to finish merging", "\x1b[0;32m")?;
    
    Ok(())
}