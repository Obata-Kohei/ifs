/*
pub mod ifs;
pub mod ifs_presets;
pub mod fractaldb;
pub mod gacha;

#[cfg(test)]
mod tests {
    use crate::{fractaldb::make_db, gacha, ifs::*, ifs_presets::ifs_presets};

    #[test]
    fn ifs_test() {
        let ifs = ifs_presets("snowflake").unwrap();
        let v = ifs.generate(&Point {x: 0.0, y: 0.0}, 100_000_00, 1000);
        println!("{:?}", &v[..10]);

        let img = render(&v, 512, 512);

        if let Ok(_) = img.save("aaa.png") {
            println!("\nDONE");
        } else {
            println!("\nFAIL");
        }
    }

    #[test]
    fn db_test() {
        make_db(100, 2, 256, 256);
    }

    #[test]
    fn gacha_test() {
        gacha::exe_gacha(100, 128, 128);
    }
}
*/