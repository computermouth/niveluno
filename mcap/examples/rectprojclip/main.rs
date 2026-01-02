use ::core::f32;

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

            // rect
            let window = Window::new(
                0.,
                r_dim.x as f64,
                0.,
                r_dim.y as f64,
            );
            let rect = Rectangle::new(window.x_min as f32, window.y_min as f32, r_dim.x, r_dim.y);

            // get rectangle space vectors
            let rect_y = (p_dst - p_src).normalized();
            let rect_x = Vector2::new(-rect_y.y, rect_y.x);

            // p1 relative to p_src
            let p1_translated = Vector2::new(p1.x as f32, p1.y as f32) - p_src;

            // project p1_translated onto the rectangle
            // use this for the clipping
            let p1_rspace = Vector2::new(
                p1_translated.dot(rect_x),
                p1_translated.dot(rect_y)
            );

            // clip p1-p2 here2
            let clipped_p1 = p1_rspace - Vector2::one() * 10.;

            // then project back clipped line
            let p1_back = p_src + rect_x * clipped_p1.x + rect_y * clipped_p1.y;
            d.draw_circle(p1_back.x as i32, p1_back.y as i32, 3., Color::PURPLE);

            // Visualize the transformed point (offset for visibility in rectangle-space origin)
            let vp1 = Vector2::new(
                radius + p1_rspace.x,
                p1_rspace.y
            );
            { // top-lefts
                let vrect = Rectangle::new(top_left.x + rect.x, top_left.y + rect.y, rect.width, rect.height);
                d.draw_rectangle_lines_ex(vrect, 3., Color::BLACK);
                d.draw_circle((top_left.x + vp1.x) as i32, (top_left.y + vp1.y) as i32, 3., Color::ORANGE);
                d.draw_circle((top_left.x + window.x_min as f32 + radius) as i32, (top_left.y + window.y_min as f32) as i32, 4., Color::BLUE);
                d.draw_circle((top_left.x + window.x_min as f32 + radius) as i32, (top_left.y + window.y_max as f32) as i32, 4., Color::RED);
            }

            // accepted path
            // let line = LineSegment::new(Point::new(-10.0, -10.0), Point::new(20.0, 20.0));
            // if let Some(cl) = clip_line(line, window) {
            //     let start = Vector2::new(cl.p1.x as f32, cl.p1.y as f32);
            //     let end = Vector2::new(cl.p2.x as f32, cl.p2.y as f32);
            //     d.draw_line_ex(start, end, 3., Color::GREEN);
            // }

            // draw walls
            for wall in &walls {
                let start = Vector2::new(wall.p1.x as f32, wall.p1.y as f32);
                let end = Vector2::new(wall.p2.x as f32, wall.p2.y as f32);
                d.draw_line_ex(start, end, 3., Color::BLACK);
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
