use ::core::f32;
use mcap::scrap as mcap;

use glam::Vec2;
use line_clipping::cohen_sutherland::clip_line;
use line_clipping::{LineSegment, Point, Window};
use mcap::{
    Surface, Triangle, Vec3, circle_wall_for_hotdog, closest_point_on_segment_v2,
    closest_point_on_segment_v3, find_floor_height_hotdog, get_face_normal, get_step_push,
    rect_wall_for_hotdog,
};
use modelz;
use raylib::ffi::RL_SRC_COLOR;
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
    const SCREEN_W: i32 = 1280;
    const SCREEN_H: i32 = 960;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("gui showcase")
        .build();

    let center_v2 = Vector2::new(SCREEN_W as f32, SCREEN_H as f32) / 2.;
    let mut p_src = center_v2;
    let radius = 20.;

    let p1 = center_v2 + Vector2::new(100., 100.);
    let p2 = center_v2 + Vector2::new(-100., 100.);
    let p3 = center_v2 + Vector2::new(0., -100.);

    let p1v3 = Vec3::new(p1.x as f32, 0., p1.y as f32);
    let p2v3 = Vec3::new(p2.x as f32, 0., p2.y as f32);
    let p3v3 = Vec3::new(p3.x as f32, 0., p3.y as f32);

    let surfaces = vec![Surface::new(
        [p3v3, p2v3, p1v3],
        get_face_normal(p3v3, p2v3, p1v3),
    )];

    let floors: Vec<_> = surfaces
        .iter()
        .filter(|s| {
            if let Surface::Floor(_) = s {
                true
            } else {
                false
            }
        })
        .collect();

    assert_eq!(floors.len(), 1);

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

        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::WHITESMOKE);

            d.draw_line_ex(p1, p2, 3., Color::BLACK);
            d.draw_line_ex(p3, p2, 3., Color::BLACK);
            d.draw_line_ex(p1, p3, 3., Color::BLACK);

            d.draw_circle(p1.x as i32, p1.y as i32, radius, Color::BLACK);
            d.draw_circle(p2.x as i32, p2.y as i32, radius, Color::BLACK);
            d.draw_circle(p3.x as i32, p3.y as i32, radius, Color::BLACK);

            // // collision with tri
            // let mut src_color = match raylib::check_collision_point_triangle(p_src, p1, p2, p3) {
            //     true => Color::GREEN,
            //     false => Color::RED,
            // };

            let np1p2 = closest_point_on_segment_v2(
                Vec2::new(p_src.x, p_src.y),
                Vec2::new(p1.x, p1.y),
                Vec2::new(p2.x, p2.y),
            );
            d.draw_circle_lines(np1p2.x as i32, np1p2.y as i32, radius, Color::BLUE);
            // if np1p2.distance(Vec2::new(p_src.x, p_src.y)) <= radius {
            //     src_color = Color::GREEN
            // }

            let np1p3 = closest_point_on_segment_v2(
                Vec2::new(p_src.x, p_src.y),
                Vec2::new(p1.x, p1.y),
                Vec2::new(p3.x, p3.y),
            );
            d.draw_circle_lines(np1p3.x as i32, np1p3.y as i32, radius, Color::BLUE);
            // if np1p3.distance(Vec2::new(p_src.x, p_src.y)) <= radius {
            //     src_color = Color::GREEN
            // }

            let np3p2 = closest_point_on_segment_v2(
                Vec2::new(p_src.x, p_src.y),
                Vec2::new(p3.x, p3.y),
                Vec2::new(p2.x, p2.y),
            );
            d.draw_circle_lines(np3p2.x as i32, np3p2.y as i32, radius, Color::BLUE);
            // if np3p2.distance(Vec2::new(p_src.x, p_src.y)) <= radius {
            //     src_color = Color::GREEN
            // }

            let src_color = match find_floor_height_hotdog(
                Vec3::new(p_src.x, 0., p_src.y),
                10., // unused here
                &floors,
                radius,
            ) {
                Some(_) => Color::GREEN,
                None => Color::RED,
            };

            let rad = radius as i32;

            // src circle
            let psx = p_src.x as i32;
            let psy = p_src.y as i32;
            d.draw_circle_v(p_src, radius * 3. / 4., src_color);
            d.draw_circle_lines_v(p_src, radius, src_color);
            d.draw_text("src", psx + rad, psy, rad, Color::BLACK);

            d.draw_text(&format!("FPS Demo"), 20, 20, 20, Color::BLACK);
            d.draw_text(
                &format!("p_src: {:.1} {:.1}", p_src.x, p_src.y),
                20,
                40,
                20,
                Color::BLACK,
            );
        }
    }
}
