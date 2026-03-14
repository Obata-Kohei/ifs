pub mod ifs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn ifs_test() {
        use crate::ifs::*;
        let ifs = IFS {
            transforms: vec![
                Affine { a:0.5,b:0.0,c:0.0, d:0.0,e:0.5,f:0.0 },
                Affine { a:0.5,b:0.0,c:0.5, d:0.0,e:0.5,f:0.0 },
                Affine { a:0.5,b:0.0,c:0.25, d:0.0,e:0.5,f:0.433 },
            ],
        };
        let v = ifs.generate(100_000, 1000);
        println!("{:?}", &v[..10]);

        let img = render(&v, 512, 512);

        if let Ok(_) = img.save("aaa.png") {
            println!("\nDONE");
        } else {
            println!("\nFAIL");
        }
    }
}
