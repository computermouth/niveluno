// HotDog { src: Vec2(89.92254, 110.92717), srcv3: Vec3(89.92254, 96.15472, 110.92717), dst: Vec2(89.92245, 110.97825), skin: 0.001, radius: 1.0, y_dir: Vec2(-0.0017923795, 0.9999984), x_dir: Vec2(-0.9999984, -0.0017923795), window: Window { x_min: -1.0, x_max: 1.0, y_min: 0.0, y_max: 0.051078878343105316 }, original_dir: Vec2(-0.0017964393, 0.99999845) }
// HotDog { src: Vec2(89.92252, 110.934685), srcv3: Vec3(89.92252, 96.15472, 110.934685), dst: Vec2(89.92252, 110.97825), skin: 0.001, radius: 1.0, y_dir: Vec2(0.0, 1.0), x_dir: Vec2(-1.0, 0.0), window: Window { x_min: -1.0, x_max: 1.0, y_min: 0.0, y_max: 0.0435638427734375 }, original_dir: Vec2(-0.0017964393, 0.99999845) }
// thread 'main' panicked at src/li

use mcap::{HotDog, Vec2, Vec3, Surface, get_face_normal};
use raylib::prelude::*;
mod triangles;

trait ToVec3 {
    fn to_mcapv3(&self) -> Vec3;
}

trait ToVector3 {
    fn to_rayv3(&self) -> Vector3;
}

impl ToVec3 for Vector3 {
    fn to_mcapv3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl ToVector3 for Vec3 {
    fn to_rayv3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

pub fn at_origin(v: Vector3) -> Vector3 {
    v + Vector3::one() * 100.
}

fn main(){

    let origin = at_origin(Vector3::zero());

    let collison_triangles =
        triangles::get_triangles(modelz::Model3D::load("res/nmap.glb").unwrap());

    let surfaces: Vec<_> = collison_triangles
        .iter()
        .map(|t| {
            Surface::new(
                [
                    (t[0] + origin).to_mcapv3(),
                    (t[1] + origin).to_mcapv3(),
                    (t[2] + origin).to_mcapv3(),
                ],
                get_face_normal(t[0].to_mcapv3(), t[1].to_mcapv3(), t[2].to_mcapv3()),
            )
        })
        .collect();

    let surfaces: Vec<_> = surfaces.iter().map(|s| s).collect();

    let src = Vec3::new(89.92254, 96.15472, 110.92717);
    let dst = Vec3::new(89.92245, src.y, 110.97825);
    let org = Vec3::new(-0.0017964393, 0., 0.99999845);

    let hotdog = HotDog::new(src, dst, 1.0, org);
    let s = hotdog.check_walls_c2(&surfaces);

    let hdc = s.unwrap();
    let n_src = Vec3::new(hdc.dest_xz.x, src.y, hdc.dest_xz.y);
    let n_dst = n_src + Vec3::new(hdc.next_move.x, 0., hdc.next_move.y);

    eprintln!("src: {:?}", src);
    eprintln!("dst: {:?}", dst);
    eprintln!("n_s: {:?}", n_src);
    eprintln!("n_d: {:?}", n_dst);
    eprintln!("hdcnm: {:?}", hdc.next_move);

    let hotdog = HotDog::new(n_src, n_dst, 1.0, org);
    let s = hotdog.check_walls_c2(&surfaces);

    eprintln!("s: {:?}", s);

}