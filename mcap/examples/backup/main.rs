use ::core::f32;

use line_clipping::cohen_sutherland::clip_line;
use line_clipping::{LineSegment, Point, Window};
use mcap::{
    Surface, Triangle, Vec3, circle_wall_for_hotdog, closest_point_on_segment_v3, get_face_normal,
    get_step_push, rect_wall_for_hotdog,
};
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

fn main() {
    const SCREEN_W: i32 = 640;
    const SCREEN_H: i32 = 480;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("gui showcase")
        .build();

    let center_v2 = Vector2::new(SCREEN_W as f32, SCREEN_H as f32) / 2.;
    let mut p_src = center_v2 + Vector2::new(0., center_v2.y / 2.);
    let mut p_dst = center_v2 - Vector2::new(0., center_v2.y / 2.);
    let radius = 20.;

    let fw = SCREEN_W as f32;
    let fh = SCREEN_H as f32;

    let walls = vec![
        [Vec3::new(fw / 8., 0., fh / 4.), Vec3::new(fw * 7. / 8., 0., fh / 4.)]
    ];

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let time = rl.get_time();
        let fps = rl.get_fps();

        // line src movement
        {
            let w = rl.is_key_down(KeyboardKey::KEY_W);
            let a = rl.is_key_down(KeyboardKey::KEY_A);
            let s = rl.is_key_down(KeyboardKey::KEY_S);
            let d = rl.is_key_down(KeyboardKey::KEY_D);
            let ws = w as i8 as f32 - s as i8 as f32;
            let ad = a as i8 as f32 - d as i8 as f32;

            p_src -= Vector2::new(ad, ws) * fd * 100.;
        }

        // line dst movement
        {
            let u = rl.is_key_down(KeyboardKey::KEY_UP);
            let d = rl.is_key_down(KeyboardKey::KEY_DOWN);
            let l = rl.is_key_down(KeyboardKey::KEY_LEFT);
            let r = rl.is_key_down(KeyboardKey::KEY_RIGHT);
            let ud = u as i8 as f32 - d as i8 as f32;
            let lr = l as i8 as f32 - r as i8 as f32;

            p_dst -= Vector2::new(lr, ud) * fd * 100.;
        }

        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::WHITESMOKE);

            let rad = radius as i32;

            // src circle
            let psx = p_src.x as i32;
            let psy = p_src.y as i32;
            d.draw_circle_v(p_src, radius * 3. / 4., Color::BLUE);
            d.draw_circle_lines_v(p_src, radius, Color::BLUE);
            d.draw_text("src", psx + rad, psy, rad, Color::BLACK);

            // dst circle
            let pdx = p_dst.x as i32;
            let pdy = p_dst.y as i32;
            d.draw_circle_v(p_dst, radius * 3. / 4., Color::RED);
            d.draw_circle_lines_v(p_dst, radius, Color::RED);
            d.draw_text("dst", pdx + rad, pdy, rad, Color::BLACK);

            // full path
            d.draw_line(psx, psy, pdx, pdy, Color::BLACK);

            for [w1, w2] in &walls {
                let s = Vector2::new(w1.x, w1.z);
                let e = Vector2::new(w2.x, w2.z);
                d.draw_line_ex(s, e, 5., Color::BLACK);
            }



            d.draw_text(&format!("FPS Demo"), 20, 20, 20, Color::BLACK);
            d.draw_text(
                &format!("p_src: {:.1} {:.1}", p_src.x, p_src.y),
                20,
                40,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!("p_dst: {:.1} {:.1}", p_dst.x, p_dst.y),
                20,
                60,
                20,
                Color::BLACK,
            );
        }
    }
}
