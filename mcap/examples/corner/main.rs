use ::core::f32;
use std::panic;

use mcap::scrap as mcap;

use glam::Vec2;
use line_clipping::{LineSegment, Point, Window};
use mcap::{
    HotDog, Surface, Triangle, Vec3, circle_wall_for_hotdog, closest_point_on_segment_v2,
    closest_point_on_segment_v3, get_face_normal, get_step_push, rect_wall_for_hotdog,
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

    let y100 = Vector2::new(0., 100.);
    let x100 = Vector2::new(100., 0.);

    let mut p_src = center_v2 - y100;
    let mut p_dst = center_v2 + y100;
    let radius = 20.;

    let center_x100 = center_v2 - x100;

    let p1 = Point {
        x: center_x100.x as f64,
        y: center_x100.y as f64,
    };
    let p2 = Point {
        x: (center_x100.x + 100.) as f64,
        y: (center_x100.y) as f64,
    };
    let p3 = Point {
        x: (center_x100.x) as f64,
        y: (center_x100.y + 100.) as f64,
    };
    let walls = vec![LineSegment::new(p2, p1), LineSegment::new(p1, p3)];

    let surfaces: Vec<_> = walls
        .iter()
        .map(|t| {
            [
                Vec3::new(t.p1.x as f32, 0., t.p1.y as f32),
                Vec3::new(t.p2.x as f32, 1., t.p2.y as f32),
                Vec3::new(t.p2.x as f32, 0., t.p2.y as f32),
            ]
        })
        .map(|t| Surface::new([t[0], t[1], t[2]], get_face_normal(t[0], t[1], t[2])))
        .collect();

    let wsurfs: Vec<_> = surfaces
        .iter()
        .filter(|s| {
            if let Surface::Wall(_) = s {
                true
            } else {
                false
            }
        })
        .collect();

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

            // draw walls
            for wall in &walls {
                let start = Vector2::new(wall.p1.x as f32, wall.p1.y as f32);
                let end = Vector2::new(wall.p2.x as f32, wall.p2.y as f32);
                d.draw_line_ex(start, end, 3., Color::BLACK);
            }

            // origin space normals
            let triangles: Vec<_> = walls
                .iter()
                .map(|ls| {
                    [
                        Vec3::new(ls.p1.x as f32, 0., ls.p1.y as f32),
                        Vec3::new(ls.p2.x as f32, 1., ls.p2.y as f32),
                        Vec3::new(ls.p2.x as f32, 0., ls.p2.y as f32),
                    ]
                })
                .collect();
            for [a, b, c] in triangles {
                let norm = get_face_normal(a, b, c);
                let start = Vector2::new((a.x + b.x) / 2., (a.z + b.z) / 2.);
                let end = start + Vector2::new(norm.x, norm.z) * 20.;
                d.draw_line_ex(start, end, 3., Color::ORANGE);
            }

            // hotdog
            let src = Vec3::new(p_src.x, 0., p_src.y);
            let dst = Vec3::new(p_dst.x, 0., p_dst.y);

            d.draw_text(&format!("FPS Demo"), 20, 600, 20, Color::BLACK);
            d.draw_text(
                &format!("p_src: {:.1} {:.1}", p_src.x, p_src.y),
                20,
                20,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!("p_dst: {:.1} {:.1}", p_dst.x, p_dst.y),
                20,
                40,
                20,
                Color::BLACK,
            );

            let max_iter = 4;
            let cs = Color::YELLOW.lerp(Color::GREEN, 1.0 / (max_iter as f32 + 1.0));
            let ce = Color::YELLOW.lerp(Color::GREEN, max_iter as f32 / max_iter as f32 + 1.0);
            let colors: Vec<Color> = (0..max_iter)
                .map(|i| cs.lerp(ce, i as f32 / max_iter as f32))
                .collect();

            let final_stop = {
                let mut final_stop = p_dst;
                let starting_dir = (p_dst - p_src).normalized();
                let mut lsrc = src;
                let mut ldst = dst;
                for i in 0..max_iter {
                    let hotdog = HotDog::new(
                        lsrc,
                        ldst,
                        radius,
                        Vec3::new(starting_dir.x, 0., starting_dir.y),
                    );
                    if let Some(hdc) = hotdog.check_walls_c2(&wsurfs) {
                        // if hdc.nt == 0. {
                        //     panic!("hdc.nt0");
                        // }

                        lsrc = Vec3::new(hdc.dest_xz.x, 0., hdc.dest_xz.y);
                        ldst = lsrc + Vec3::new(hdc.next_move.x, 0., hdc.next_move.y);

                        // stop circle
                        let hit = Vector2::new(hdc.dest_xz.x, hdc.dest_xz.y);
                        let hitix = hit.x as i32;
                        let hitiy = hit.y as i32;
                        d.draw_circle_v(hit, radius * 3. / 4., colors[i]);
                        d.draw_circle_lines_v(hit, radius, colors[i]);
                        d.draw_text(&format!("s{i}"), hitix + rad, hitiy, rad, Color::BLACK);

                        if i == max_iter - 1 {
                            final_stop = hit;
                        } else {
                            final_stop = hit + Vector2::new(hdc.next_move.x, hdc.next_move.y);
                        }
                        // stop redirect
                        d.draw_line_ex(
                            hit,
                            hit + Vector2::new(hdc.next_move.x, hdc.next_move.y),
                            3.,
                            colors[i],
                        );
                    } else {
                        break;
                    }
                }

                // final circle1
                if final_stop.distance_to(p_dst) > 0.01 {
                    d.draw_circle_v(final_stop, radius * 3. / 4., Color::GREEN);
                    d.draw_circle_lines_v(final_stop, radius, Color::GREEN);
                }

                d.draw_text(
                    &format!("stop: {:.1} {:.1}", final_stop.x, final_stop.y),
                    20,
                    60,
                    20,
                    Color::BLACK,
                );

                final_stop
            };

            let src = final_stop;
            let dst = final_stop + Vector2::new(100., 100.);
            let mut final_stop = dst;

            // let mut final_stop = p_dst;
            let starting_dir = (dst - src).normalized();
            let mut lsrc = Vec3::new(src.x, 0., src.y);
            let mut ldst = Vec3::new(dst.x, 0., dst.y);
            for i in 0..max_iter {
                let hotdog = HotDog::new(
                    lsrc,
                    ldst,
                    radius,
                    Vec3::new(starting_dir.x, 0., starting_dir.y),
                );
                if let Some(hdc) = hotdog.check_walls_c2(&wsurfs) {
                    lsrc = Vec3::new(hdc.dest_xz.x, 0., hdc.dest_xz.y);
                    ldst = lsrc + Vec3::new(hdc.next_move.x, 0., hdc.next_move.y);

                    // stop circle
                    let hit = Vector2::new(hdc.dest_xz.x, hdc.dest_xz.y);
                    let hitix = hit.x as i32;
                    let hitiy = hit.y as i32;
                    d.draw_circle_v(hit, radius * 3. / 4., colors[i]);
                    d.draw_circle_lines_v(hit, radius, colors[i]);
                    d.draw_text(&format!("s{i}"), hitix + rad, hitiy, rad, Color::BLACK);

                    if i == max_iter - 1 {
                        final_stop = hit;
                    } else {
                        final_stop = hit + Vector2::new(hdc.next_move.x, hdc.next_move.y);
                    }
                    // stop redirect
                    d.draw_line_ex(
                        hit,
                        hit + Vector2::new(hdc.next_move.x, hdc.next_move.y),
                        3.,
                        colors[i],
                    );
                } else {
                    break;
                }
            }

            // final circle1
            // if final_stop.distance_to(p_dst) > 0.01 {
            d.draw_circle_v(final_stop, radius * 3. / 4., Color::PURPLE);
            d.draw_circle_lines_v(final_stop, radius, Color::PURPLE);
            // }
        }
    }
}
