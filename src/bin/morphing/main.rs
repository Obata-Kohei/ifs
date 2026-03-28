use ifs::ifs::*;
use ifs::ifs_presets::*;

fn main() {
    let fern = ifs_presets("fern").expect("Preset name should be valid");
    let tree = ifs_presets("tree").expect("Preset name should be valid");

    println!("{}", fern.transforms.len());
    println!("{}", tree.transforms.len());



}

fn lerp_param(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn interpolate_ifs(ifs1: &IFS, ifs2: &IFS, t: f64) -> IFS {
    let mut ifs1 = ifs1.clone();
    let mut ifs2 = ifs2.clone();
    let n_transforms1 = ifs1.transforms.len();
    let n_transforms2 = ifs2.transforms.len();

    let n_transforms = n_transforms2.max(n_transforms1);
    // ifs間のtransformsの数をそろえる
    if n_transforms1 > n_transforms2 {
        ifs2.add_transform(ifs1.transforms.last().unwrap_or(&Transform { affine: Affine::id(), weight: 0.0 }));
    } else {
        ifs1.add_transform(ifs2.transforms.last().unwrap_or(&Transform { affine: Affine::id(), weight: 0.0 }));
    }

    // 各パラメタを補間する
    

    ifs_presets("fern").unwrap()
}