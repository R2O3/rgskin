mod test_dependencies;
use test_dependencies::*;

#[test]
fn osu_skin_ini_test() -> Result<(), Box<dyn std::error::Error>> {
    benchmark_closure(||
    {
        let raw_skin_ini = read_file_to_string("./tests/configs/osu/bubbleskin.ini").unwrap();
        let _skin_ini = rgc_skin::load::osu::skin_ini(&raw_skin_ini)?;
        Ok(())
    }, "Osu! skin ini", "to finish skin ini parsing", "\x1b[0;33m")?;
    Ok(())
}