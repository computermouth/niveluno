use ::core::f32;

use glam::Vec2;
use line_clipping::cohen_sutherland::clip_line;
use line_clipping::{LineSegment, Point, Window};
use mcap::{
    Surface, Triangle, Vec3, circle_wall_for_hotdog, closest_point_on_segment_v2, closest_point_on_segment_v3, get_face_normal, get_step_push, rect_wall_for_hotdog, HotDog
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
    const SCREEN_W: i32 = 1280;
    const SCREEN_H: i32 = 960;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("gui showcase")
        .build();

    let center_v2 = Vector2::new(SCREEN_W as f32, SCREEN_H as f32) / 2.;
    let quarter_y = center_v2.y / 2.;
    let quarter_x = center_v2.x / 2.;
    let top_left = center_v2 - Vector2::new(quarter_x, quarter_y);

    let mut p_src = center_v2;
    let mut p_dst = p_src + Vector2::new(0., -100.);
    let radius = 20.;

    let right_3q = center_v2 + Vector2::new(quarter_x, 0.) - 100.;
    let p1 = Point {
        x: right_3q.x as f64,
        y: right_3q.y as f64,
    };
    let p2 = Point {
        x: (right_3q.x + 200.) as f64,
        y: (right_3q.y - 150.) as f64,
    };
    let p3 = Point {
        x: (right_3q.x + 200.) as f64,
        y: (right_3q.y + 150.) as f64,
    };
    let walls = vec![
        LineSegment::new(p1, p2),
        LineSegment::new(p2, p3),
        LineSegment::new(p3, p1),
    ];

    let surfaces: Vec<_> = vec![[p1, p2], [p2, p3], [p3, p1]].iter()
        .map(|t| {
            [
                Vec3::new(t[0].x as f32, 0.,t[0].y as f32),
                Vec3::new(t[1].x as f32, 1.,t[1].y as f32),
                Vec3::new(t[1].x as f32, 0.,t[1].y as f32),
            ]
        }).map(|t| {
            Surface::new(
                [
                    t[0],
                    t[1],
                    t[2],
                ],
                get_face_normal(t[0], t[1], t[2]),
            )
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

        // get rectangle dimensions from radius and src/dst distance
        let r_dim = Vector2::new(radius * 2., (p_dst - p_src).length());

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

            // hotdog
            let src = Vec3::new(p_src.x, 0., p_src.y);
            let dst = Vec3::new(p_dst.x, 0., p_dst.y);
            let hotdog = HotDog::new(src, dst, radius);

            let window = hotdog.get_window();

            // draw points in origin space
            d.draw_circle(p1.x as i32, p1.y as i32, 3., Color::ORANGE);
            d.draw_circle(p2.x as i32, p2.y as i32, 3., Color::FUCHSIA);
            d.draw_circle(p3.x as i32, p3.y as i32, 3., Color::PURPLE);

            // draw clipped line in origin space
            let clips: Vec<_> = walls.iter().map(|wall|{
                hotdog.clip_line_segment(Vec2::new(wall.p1.x as f32, wall.p1.y as f32), Vec2::new(wall.p2.x as f32, wall.p2.y as f32))
            }).collect();
            // eprintln!("clips: {:?}", clips);
            for clip in &clips {
                if let Some((p1, p2)) = clip {
                    d.draw_line_ex(Vector2::new(p1.x, p1.y), Vector2::new(p2.x, p2.y), 8., Color::RED);
                }
            }

            // closest point origin
            let cp1 = hotdog.closest_point_on_segment_origin(Vec2::new(p1.x as f32, p1.y as f32), Vec2::new(p2.x as f32, p2.y as f32));
            d.draw_circle(cp1.x as i32, cp1.y as i32, 3., Color::GREEN);
            let cp2 = hotdog.closest_point_on_segment_origin(Vec2::new(p2.x as f32, p2.y as f32), Vec2::new(p3.x as f32, p3.y as f32));
            d.draw_circle(cp2.x as i32, cp2.y as i32, 3., Color::GREEN);
            let cp3 = hotdog.closest_point_on_segment_origin(Vec2::new(p3.x as f32, p3.y as f32), Vec2::new(p1.x as f32, p1.y as f32));
            d.draw_circle(cp3.x as i32, cp3.y as i32, 3., Color::GREEN);

            // top-lefts
                // rect
                let rect_width = (window.x_max - window.x_min) as f32;
                let rect_height = (window.y_max - window.y_min) as f32;
                let vrect = Rectangle::new(
                    top_left.x,
                    top_left.y,
                    rect_width,
                    rect_height
                );
                d.draw_rectangle_lines_ex(vrect, 3., Color::BLACK);

                // top and bottom
                let center_x = top_left.x + rect_width / 2.0;
                d.draw_circle_lines(center_x as i32, top_left.y as i32, radius, Color::BLUE);
                d.draw_circle(center_x as i32, (top_left.y + rect_height) as i32, 4., Color::RED);

                // points
                let p1_r = hotdog.origin_point_to_rect_space(Vec2::new(p1.x as f32, p1.y as f32));
                let lp1 = Vector2::new(top_left.x + p1_r.x + radius, top_left.y + p1_r.y);
                d.draw_circle(lp1.x as i32, lp1.y as i32, 3., Color::ORANGE);

                let p2_r = hotdog.origin_point_to_rect_space(Vec2::new(p2.x as f32, p2.y as f32));
                let lp2 = Vector2::new(top_left.x + p2_r.x + radius, top_left.y + p2_r.y);
                d.draw_circle(lp2.x as i32, lp2.y as i32, 3., Color::FUCHSIA);

                let p3_r = hotdog.origin_point_to_rect_space(Vec2::new(p3.x as f32, p3.y as f32));
                let lp3 = Vector2::new(top_left.x + p3_r.x + radius, top_left.y + p3_r.y);
                d.draw_circle(lp3.x as i32, lp3.y as i32, 3., Color::PURPLE);

                // full lines
                d.draw_line_ex(lp1, lp2, 2., Color::GRAY);
                d.draw_line_ex(lp2, lp3, 2., Color::GRAY);
                d.draw_line_ex(lp3, lp1, 2., Color::GRAY);

                let mut nearest = None;

                // draw clipped lines in rectangle space
                for clip in clips {
                    if let Some((p1, p2)) = clip {
                        let p1_r = hotdog.origin_point_to_rect_space(p1);
                        let p2_r = hotdog.origin_point_to_rect_space(p2);
                        let vis_p1 = Vector2::new(top_left.x + p1_r.x + radius, top_left.y + p1_r.y);
                        let vis_p2 = Vector2::new(top_left.x + p2_r.x + radius, top_left.y + p2_r.y);
                        d.draw_line_ex(vis_p1, vis_p2, 4., Color::RED);

                        // closest point rect
                        let cp1 = hotdog.closest_point_on_segment_rect_circ(Vec2::new(p1.x as f32, p1.y as f32), Vec2::new(p2.x as f32, p2.y as f32));
                        match nearest {
                            None => nearest = Some(cp1),
                            Some(n) => {
                                if n.distance(Vec2::ZERO) > cp1.distance(Vec2::ZERO) {
                                    nearest = Some(cp1)
                                } else if n.distance(cp1) < f32::EPSILON {
                                    // if two points are the same, we're colliding with a corner
                                    // either choose one, or do something with both normals
                                }
                            }
                        }
                        d.draw_circle_lines((top_left.x + radius + cp1.x) as i32, (top_left.y + cp1.y) as i32, 3., Color::GREEN);

                        let true_closest = hotdog.closest_point_on_segment_rect(Vec2::new(p1.x as f32, p1.y as f32), Vec2::new(p2.x as f32, p2.y as f32));
                        d.draw_circle((top_left.x + radius + true_closest.x) as i32, (top_left.y + true_closest.y) as i32, 3., Color::GREEN);
                        
                    }
                }

                let res = hotdog.nearest_point_on_surfaces_for_rect(&surfaces);
                assert_eq!(res.is_none(), nearest.is_none());

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

            d.draw_text(&format!("FPS Demo"), 20, 600, 20, Color::BLACK);
            d.draw_text(
                &format!("p_src: {:.1} {:.1}", p_src.x, p_src.y),
                20,
                620,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!("p_dst: {:.1} {:.1}", p_dst.x, p_dst.y),
                20,
                640,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!("r_dim: {:.1} {:.1}", r_dim.x, r_dim.y),
                20,
                660,
                20,
                Color::BLACK,
            );

                if let Some(n) = nearest {
                    // top-left line to nearest
                    d.draw_line((top_left.x + radius) as i32, top_left.y as i32, (top_left.x + radius + n.x) as i32, (top_left.y + n.y) as i32, Color::BLACK);
                    d.draw_circle_lines((top_left.x + radius + n.x) as i32, (top_left.y + n.y) as i32, radius, Color::PURPLE);

                    // stop circle
                    let p_stop = hotdog.rect_point_to_origin_space(n);
                    let p_stop_v2 = Vector2::new(p_stop.x, p_stop.y);
                    let psx = p_stop.x as i32;
                    let psy = p_stop.y as i32;
                    d.draw_circle_v(p_stop_v2, radius * 3. / 4., Color::PURPLE);
                    d.draw_circle_lines_v(p_stop_v2, radius, Color::PURPLE);
                    d.draw_text("stop", psx + rad, psy, rad, Color::BLACK);

                    let hdc = res.unwrap();
                    assert_eq!(hdc.dest_xz, n);

                    // stop redirect
                    d.draw_line_ex(p_stop_v2, p_stop_v2 + Vector2::new(hdc.new_target.x, hdc.new_target.y), 3., Color::HOTPINK);

                    d.draw_text(
                        &format!("stop: {:.1} {:.1}", p_stop.x, p_stop.y),
                        20,
                        680,
                        20,
                        Color::BLACK,
                    );
                }
        }
    }
}
