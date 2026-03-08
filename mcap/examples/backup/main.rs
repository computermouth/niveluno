use ::core::f32;

use mcap::scrap as mcap;

use glam::Vec2;
use mcap::{
    C2Ray, C2Raycast, HotDog, Surface, Vec3, get_face_normal, 
};
use raylib::prelude::*;

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
        [
            Vec3::new(fw * 1.0 / 8., 0., fh * 2.0 / 8.),
            Vec3::new(fw * 7. / 8., 0., fh * 2.0 / 8.),
        ],
        [
            Vec3::new(fw * 3.0 / 8., 0., fh * 2.5 / 8.),
            Vec3::new(fw * 7. / 8., 0., fh * 2.5 / 8.),
        ],
        [
            Vec3::new(fw * 4.0 / 8., 0., fh * 3.0 / 8.),
            Vec3::new(fw * 7. / 8., 0., fh * 3.0 / 8.),
        ],
        [
            Vec3::new(fw * 5.0 / 8., 0., fh * 3.5 / 8.),
            Vec3::new(fw * 7. / 8., 0., fh * 3.5 / 8.),
        ],
        [
            Vec3::new(fw * 5.5 / 8., 0., fh * 4.0 / 8.),
            Vec3::new(fw * 7. / 8., 0., fh * 4.0 / 8.),
        ],
        [
            Vec3::new(fw * 6.0 / 8., 0., fh * 4.5 / 8.),
            Vec3::new(fw * 7. / 8., 0., fh * 4.5 / 8.),
        ],
    ];

    let surfaces: Vec<_> = walls
        .iter()
        .map(|[p1, p2]| {
            [
                Vec3::new(p1.x as f32, 0., p1.z as f32),
                Vec3::new(p2.x as f32, 1., p2.z as f32),
                Vec3::new(p2.x as f32, 0., p2.z as f32),
            ]
        })
        .map(|t| Surface::new([t[0], t[1], t[2]], get_face_normal(t[0], t[1], t[2])))
        .collect();

    let wsurfs: Vec<_> = surfaces
        .iter()
        .filter_map(|s| match s {
            Surface::Wall(w) => Some(w),
            _ => None,
        })
        .collect();

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();

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

            let lpos = Vec3::new(p_src.x, 0., p_src.y);
            let ldst = Vec3::new(p_dst.x, 0., p_dst.y);
            let step = ldst - lpos;
            let move_len = step.length();
            let move_dir = step.normalize();
            let hotdog = HotDog::new(lpos, ldst, radius, move_dir);

            let ray = C2Ray {
                p: Vec2::new(p_src.x, p_src.y),
                d: Vec2::new(move_dir.x, move_dir.z),
                t: move_len,
            };

            let collision = C2Raycast {
                n: Vec2::ONE.normalize(),
                t: move_len,
            };

            let dest_xz = hotdog.step_back(ray, &collision, &wsurfs, Some(&mut d));
            d.draw_circle(
                dest_xz.x as i32,
                dest_xz.y as i32,
                radius,
                Color::GREEN.alpha(0.5),
            );

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
