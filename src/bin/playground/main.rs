use ifs::ifs::*;
use ifs::ifs_presets::*;
use ifs::util::io::render;

fn main() {
    //let ifs = ifs_presets("pentagon").unwrap();


    let s = 0.5;
    let ifs = IFS::new(vec![
        Transform::new(Affine::scale(s, s).then(Affine::rotate_deg(90.)).then(Affine::translate(0.2, 0.2)), 1.),
        Transform::new(Affine::scale(s, s).then(Affine::rotate_deg(-90.)).then(Affine::translate(0.2, 0.2)), 1.),
        Transform::new(Affine::scale(s, s).then(Affine::rotate_deg(180.).then(Affine::translate(0.5, 0.5))), 1.),
    ]);

    let ifs = n_gon_fractal(5);

    let pts = ifs.generate(&Point { x: 0.0, y: 0.0 }, 500000, 1000);
    let img = render(&pts, 256, 256);
    img.save("palyground.png").expect("Error");

}

pub fn n_gon_fractal(n: usize) -> IFS {
    // n角形の場合、適切なスケール因子 s はおよそ 1 / (1 + sin(π/n) / sin(π/2-π/n) ) 的な
    // 複雑な計算になりますが、n=5なら 0.382, n=6なら 1/3 (0.333) あたりが綺麗です。
    let s = if n == 5 { 0.382 } else { 1.0 / 3.0 };
    
    let mut transforms = Vec::new();
    for i in 0..n {
        // 頂点の角度を計算 (真上 90度からスタートして 360/n 度ずつ回す)
        let angle = (90.0 + i as f64 * (360.0 / n as f64)).to_radians();
        let tx = angle.cos();
        let ty = angle.sin();

        // 拡大縮小した自分自身を、頂点方向に移動
        // (1.0 - s) を掛けると、図形全体が半径1の円にぴったり収まります
        transforms.push(Transform::new(
            Affine::scale(s, s).then(Affine::translate(tx * (1.0 - s), ty * (1.0 - s))),
            1.0
        ));
    }
    IFS::new(transforms)
}