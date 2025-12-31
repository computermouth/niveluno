use mcap::{Surface, Triangle, Vec3, get_face_normal, get_step_push};
use modelz;
use raylib::prelude::*;

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

struct HotDogResult {
    out_pos: Vector3,
    collisions: Vec<Vector3>,
    rect_bounds: Vec<(Vector3, Vector3)>
}

fn hotdog(){

}

fn main() {
    const SCREEN_W: i32 = 1280;
    const SCREEN_H: i32 = 960;
    let (mut rl, thread) = raylib::init().size(SCREEN_W, SCREEN_H).title("gui showcase").build();

    let mut p_src = Vector2::new(SCREEN_W as f32, SCREEN_H as f32) / 2.;
    let mut p_dst = p_src + Vector2::new(0., -100.);
    let radius = 20.;

    let walls = vec![
        (Vector2::new(100., 100.), Vector2::new(300., 300.))
    ];

    let triangles: Vec<_> = walls.iter().map(|(s, e)| {
        (Vector3::new(s.x, 0., s.y), Vector3::new(e.x, 0., e.y), Vector3::new(e.x, 1., e.y))
    }).collect();

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let time = rl.get_time();
        let fps = rl.get_fps();

        // player src movement
        {
            let w = rl.is_key_down(KeyboardKey::KEY_W);
            let a = rl.is_key_down(KeyboardKey::KEY_A);
            let s = rl.is_key_down(KeyboardKey::KEY_S);
            let d = rl.is_key_down(KeyboardKey::KEY_D);
            let ws = w as i8 as f32 - s as i8 as f32;
            let ad = a as i8 as f32 - d as i8 as f32;

            p_src -= Vector2::new(ad, ws) * fd * 100.;
        }

        // player dst movement
        {
            let u = rl.is_key_down(KeyboardKey::KEY_UP);
            let d = rl.is_key_down(KeyboardKey::KEY_DOWN);
            let l = rl.is_key_down(KeyboardKey::KEY_LEFT);
            let r = rl.is_key_down(KeyboardKey::KEY_RIGHT);
            let ud = u as i8 as f32 - d as i8 as f32;
            let lr = l as i8 as f32 - r as i8 as f32;

            p_dst -= Vector2::new(lr, ud) * fd * 100.;
        }

        // calculate collisions and walls, etc
        // let res = hotdog(p_src, p_dst, &triangles);

        let p_out = p_dst;

        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::WHITESMOKE);

            let rad = radius as i32;

            // src circle
            let psx = p_src.x as i32;
            let psy = p_src.y as i32;
            d.draw_circle_v(p_src, radius * 3./4., Color::BLUE);
            d.draw_circle_lines_v(p_src, radius, Color::BLUE);
            d.draw_text("src", psx + rad, psy, rad, Color::BLACK);

            // // out circle
            // let pox = p_out.x as i32;
            // let poy = p_out.y as i32;
            // d.draw_circle_v(p_out, radius * 3./4., Color::GREEN);
            // d.draw_circle_lines_v(p_out, radius, Color::GREEN);
            // d.draw_text("out", pox + rad, poy, rad, Color::BLACK);

            // if p_out != p_dst {
                // dst circle
                let pdx = p_dst.x as i32;
                let pdy = p_dst.y as i32;
                d.draw_circle_v(p_dst, radius * 3./4., Color::RED);
                d.draw_circle_lines_v(p_dst, radius, Color::RED);
                d.draw_text("dst", pdx + rad, pdy, rad, Color::BLACK);

                d.draw_line_v(p_src, p_dst, Color::RED);
            // }

            // walls and normals
            for (a, b, c) in &triangles {
                let a2 = Vector2::new(a.x, a.z);
                let b2 = Vector2::new(b.x, b.z);
                d.draw_line_ex(a2, b2, radius / 4., Color::BLACK);
                
                // draw "normal"
                let n = get_face_normal((*a).to_mcapv3(), (*b).to_mcapv3(), (*c).to_mcapv3());
                let center = (a2 + b2) / 2.;
                let end = center + Vector2::new(n.x, n.z) * radius;
                d.draw_line_ex(center,
                    end, radius / 8., Color::ORANGE);
            }

            d.draw_text(&format!("FPS Demo"), 20, 20, 20, Color::BLACK);
            d.draw_text(
                &format!(
                    "p_src: {:.1} {:.1}",
                    p_src.x, p_src.y
                ),
                20,
                40,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!(
                    "p_dst: {:.1} {:.1}",
                    p_dst.x, p_dst.y
                ),
                20,
                60,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!(
                    "p_out: {:.1} {:.1}",
                    p_out.x, p_out.y
                ),
                20,
                80,
                20,
                Color::BLACK,
            );
        }
    }
}
