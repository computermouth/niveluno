use ::core::f32;

use glam::Vec2;
use line_clipping::cohen_sutherland::clip_line;
use line_clipping::{LineSegment, Point, Window};
use mcap::{
    Surface, Triangle, Vec3, circle_wall_for_hotdog, closest_point_on_segment, get_face_normal,
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

struct HotDog {
    src: Vec2,
    dst: Vec2,
    radius: f32,
    y_dir: Vec2,
    x_dir: Vec2,
    window: Window,
}

impl HotDog {
    pub fn new(src: Vec3, dst: Vec3, radius: f32) -> Self {
        let src = Vec2::new(src.x, src.z);
        let dst = Vec2::new(dst.x, dst.z);
        let diff = dst - src;
        let y_dir = diff.normalize();
        let length = diff.length();
        Self {
            src,
            dst,
            radius,
            y_dir,
            x_dir: Vec2::new(-y_dir.y, y_dir.x),
            window: Window::new(
                -radius as f64,
                radius as f64,
                0.,
                length as f64,
            )
        }
    }

    pub fn clip_line_segment(&self, p1: Vec2, p2: Vec2) -> Option<(Vec2, Vec2)> {
        let rp1 = self.origin_point_to_rect_space(p1);
        let rp2 = self.origin_point_to_rect_space(p2);
        match clip_line(
            LineSegment {
                p1: Point { x: rp1.x as f64, y: rp1.y as f64 },
                p2: Point { x: rp2.x as f64, y: rp2.y as f64 }
            }, 
            self.window
        ) {
            None => None,
            Some(l) => {
                let op1 = self.rect_point_to_origin_space(Vec2::new(l.p1.x as f32, l.p1.y as f32));
                let op2 = self.rect_point_to_origin_space(Vec2::new(l.p2.x as f32, l.p2.y as f32));
                Some((op1, op2))
            }
        }
    }

    pub fn closest_point_on_segment_origin(&self, a: Vec2, b: Vec2) -> Vec2 {
        let ab = b - a;
        let ap = self.src - a;

        let proj = ap.dot(ab);
        let ab_len_sq = ab.length().powi(2);
        let d = proj / ab_len_sq;

        match d {
            f32::NEG_INFINITY..=0f32 => a,
            1f32..=f32::INFINITY => b,
            d => a + ab * d,
        }
    }

    pub fn closest_point_on_segment_rect(&self, a: Vec2, b: Vec2) -> Vec2 {
        let a = self.origin_point_to_rect_space(a);
        let b = self.origin_point_to_rect_space(b);

        let ab = b - a;
        let ap = Vec2::ZERO - a;

        let proj = ap.dot(ab);
        let ab_len_sq = ab.length().powi(2);
        let d = proj / ab_len_sq;

        match d {
            f32::NEG_INFINITY..=0f32 => a,
            1f32..=f32::INFINITY => b,
            d => a + ab * d,
        }
    }

    pub fn origin_point_to_rect_space(&self, p: Vec2) -> Vec2 {
        // p relative to p_src
        let p_trans = p - self.src;

        // project p_tran onto the rectangle
        Vec2::new(
            p_trans.dot(self.x_dir),
            p_trans.dot(self.y_dir)
        )
    }

    pub fn rect_point_to_origin_space(&self, p: Vec2) -> Vec2 {
        // project back on src with
        self.src + self.x_dir * p.x + self.y_dir * p.y
    }

    pub fn get_window(&self) -> Window {
        self.window
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

    let right_3q = center_v2 + Vector2::new(quarter_x, 0.);
    let p1 = Point {
        x: right_3q.x as f64,
        y: right_3q.y as f64,
    };
    let p2 = Point {
        x: (right_3q.x + 75.) as f64,
        y: (right_3q.y - 50.) as f64,
    };
    let p3 = Point {
        x: (right_3q.x + 75.) as f64,
        y: (right_3q.y + 50.) as f64,
    };
    let walls = vec![
        LineSegment::new(p1, p2),
        LineSegment::new(p2, p3),
        LineSegment::new(p3, p1),
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
            let clips = [(p1, p2), (p2, p3), (p3, p1)].map(|(p1, p2)|{
                hotdog.clip_line_segment(Vec2::new(p1.x as f32, p1.y as f32), Vec2::new(p2.x as f32, p2.y as f32))
            });
            for clip in clips {
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
            {
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
                d.draw_circle(center_x as i32, top_left.y as i32, 4., Color::BLUE);
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
                        let cp1 = hotdog.closest_point_on_segment_rect(Vec2::new(p1.x as f32, p1.y as f32), Vec2::new(p2.x as f32, p2.y as f32));
                        match nearest {
                            None => nearest = Some(cp1),
                            Some(n) => {
                                if n.distance(Vec2::ZERO) > cp1.distance(Vec2::ZERO) {
                                    nearest = Some(cp1)
                                }
                            }
                        }
                        d.draw_circle((top_left.x + radius + cp1.x) as i32, (top_left.y + cp1.y) as i32, 3., Color::GREEN);
                    }
                }

                if let Some(n) = nearest {
                    d.draw_line((top_left.x + radius) as i32, top_left.y as i32, (top_left.x + radius + n.x) as i32, (top_left.y + n.y) as i32, Color::BLACK);
                }

                d.draw_circle((top_left.x + radius + cp1.x) as i32, (top_left.y + cp1.y) as i32, 3., Color::GREEN);
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
        }
    }
}
