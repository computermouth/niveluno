// HotDog { src: Vec2(89.92254, 110.92717), srcv3: Vec3(89.92254, 96.15472, 110.92717), dst: Vec2(89.92245, 110.97825), skin: 0.001, radius: 1.0, y_dir: Vec2(-0.0017923795, 0.9999984), x_dir: Vec2(-0.9999984, -0.0017923795), window: Window { x_min: -1.0, x_max: 1.0, y_min: 0.0, y_max: 0.051078878343105316 }, original_dir: Vec2(-0.0017964393, 0.99999845) }
// HotDog { src: Vec2(89.92252, 110.934685), srcv3: Vec3(89.92252, 96.15472, 110.934685), dst: Vec2(89.92252, 110.97825), skin: 0.001, radius: 1.0, y_dir: Vec2(0.0, 1.0), x_dir: Vec2(-1.0, 0.0), window: Window { x_min: -1.0, x_max: 1.0, y_min: 0.0, y_max: 0.0435638427734375 }, original_dir: Vec2(-0.0017964393, 0.99999845) }
// thread 'main' panicked at src/li

use mcap::scrap as mcap;
use mcap::{HotDog, Surface, Vec2, Vec3, get_face_normal};
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

fn main() {
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

    let walls: Vec<&Surface> = surfaces
        .iter()
        .filter_map(|s| match s {
            Surface::Wall(_) => Some(s),
            _ => None,
        })
        .collect();

    let surfaces = vec![walls[122]];

    let src = Vec3::new(89.92254, 96.15472, 110.92717);
    let dst = Vec3::new(89.92245, src.y, 110.97825);
    let org = Vec3::new(-0.0017964393, 0., 0.99999845);

    let hd1 = HotDog::new(src, dst, 1.0, org);
    let hdc1 = hd1.check_walls_c2(&surfaces).unwrap();
    let src1 = Vec3::new(hdc1.dest_xz.x, src.y, hdc1.dest_xz.y);
    let dst1 = src1 + Vec3::new(hdc1.next_move.x, 0., hdc1.next_move.y);

    eprintln!("src: {:?}", src);
    eprintln!("dst: {:?}", dst);
    eprintln!("n_s: {:?}", src1);
    eprintln!("n_d: {:?}", dst1);
    eprintln!("hdcnm: {:?}", hdc1.next_move);

    let hd2 = HotDog::new(src1, dst1, 1.0, org);
    let hdc2 = hd2.check_walls_c2(&surfaces).unwrap();
    let src2 = Vec3::new(hdc2.dest_xz.x, src.y, hdc2.dest_xz.y);
    let dst2 = src2 + Vec3::new(hdc2.next_move.x, 0., hdc2.next_move.y);

    eprintln!("src: {:?}", src);
    eprintln!("dst: {:?}", dst);
    eprintln!("n_s: {:?}", src2);
    eprintln!("n_d: {:?}", dst2);
    eprintln!("hdcnm: {:?}", hdc2.next_move);

    let wall = match surfaces[0] {
        Surface::Wall(w) => w,
        _ => unreachable!(),
    };

    let verts = wall.verts;
    let p0 = Vector2::new(verts[0].x, verts[0].z);
    let p1 = Vector2::new(verts[1].x, verts[1].z);
    let p2 = Vector2::new(verts[2].x, verts[2].z);

    const SCREEN_W: i32 = 1640;
    const SCREEN_H: i32 = 1480;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("gui showcase")
        .build();

    // Center on midpoint between src and wall center so both are visible
    let src2d = Vector2::new(src.x, src.z);
    let dst2d = Vector2::new(dst.x, dst.z);
    let wall_center = (p0 + p1 + p2) / 3.;
    let center = (src2d + wall_center) / 2.;
    let screen_center = Vector2::new(SCREEN_W as f32 / 2., SCREEN_H as f32 / 2.);

    // Uniform scale - no exaggeration, just make it fit
    let scale = 150.;
    let radius_scale = 0.2; // shrink everything proportionally to see intersection better
    let to_screen = |v: Vector2| -> Vector2 { screen_center + (v - center) * scale * radius_scale };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::WHITESMOKE);

            // Draw triangle
            let sp0 = to_screen(p0);
            let sp1 = to_screen(p1);
            let sp2 = to_screen(p2);
            d.draw_line_ex(sp0, sp1, 2., Color::BLACK);
            d.draw_line_ex(sp1, sp2, 2., Color::BLACK);
            d.draw_line_ex(sp2, sp0, 2., Color::BLACK);

            // Draw src and dst circles (radius 1.0 in world units)
            let ssrc = to_screen(Vector2::new(src.x, src.z));
            let sdst = to_screen(Vector2::new(dst.x, dst.z));
            d.draw_circle_lines_v(ssrc, 1.0 * scale * radius_scale, Color::BLUE);
            d.draw_circle_lines_v(sdst, 1.0 * scale * radius_scale, Color::RED);

            // Draw labels
            d.draw_text(
                &format!("src: ({:.6}, {:.6})", src.x, src.z),
                10,
                10,
                20,
                Color::BLUE,
            );
            d.draw_text(
                &format!("dst: ({:.6}, {:.6})", dst.x, dst.z),
                10,
                35,
                20,
                Color::RED,
            );
            d.draw_text(&format!("scale: {}x", scale), 10, 60, 20, Color::DARKGRAY);
        }
    }
}
