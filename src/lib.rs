pub mod ifs;
pub mod ifs_presets;

#[cfg(test)]
mod tests {
    use crate::{ifs::*, ifs_presets::ifs_presets};

    #[test]
    fn ifs_test() {
        let ifs = ifs_presets("snowflake").unwrap();
        let v = ifs.generate(100_000_00, 1000);
        println!("{:?}", &v[..10]);

        let img = render(&v, 512, 512);

        if let Ok(_) = img.save("aaa.png") {
            println!("\nDONE");
        } else {
            println!("\nFAIL");
        }
    }
}
