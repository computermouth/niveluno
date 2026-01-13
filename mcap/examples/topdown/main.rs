use ::core::f32;

use glam::Vec2;
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
    let mut p_rad = 0.;
    let radius = 20.;

    let center_x100 = center_v2 + x100;

    let p1 = Point {
        x: center_x100.x as f64,
        y: center_x100.y as f64,
    };
    let p2 = Point {
        x: (center_x100.x + 100.) as f64,
        y: (center_x100.y - 50.) as f64,
    };
    let p3 = Point {
        x: (center_x100.x + 100.) as f64,
        y: (center_x100.y + 50.) as f64,
    };
    let p4 = Point {
        x: (center_x100.x) as f64,
        y: (center_x100.y + 150.) as f64,
    };
    let p5 = Point {
        x: (center_x100.x - 150.) as f64,
        y: (center_x100.y + 150.) as f64,
    };
    let p6 = Point {
        x: (center_x100.x - 225.) as f64,
        y: (center_x100.y + 125.) as f64,
    };
    let p7 = Point {
        x: (center_x100.x - 300.) as f64,
        y: (center_x100.y + 75.) as f64,
    };
    let p8 = Point {
        x: (center_x100.x - 300.) as f64,
        y: (center_x100.y) as f64,
    };
    let p9 = Point {
        x: (center_x100.x - 290.) as f64,
        y: (center_x100.y - 75.) as f64,
    };
    let walls = vec![
        LineSegment::new(p1, p2),
        LineSegment::new(p2, p3),
        LineSegment::new(p3, p1),
        LineSegment::new(p4, p1), // right of 90 degree
        LineSegment::new(p4, p1), // right of 90 degree
        LineSegment::new(p1, p4),
        LineSegment::new(p5, p4), // bottom of 90 degree
        LineSegment::new(p6, p5),
        LineSegment::new(p7, p6),
        LineSegment::new(p8, p7),
        LineSegment::new(p9, p8),
    ];

    let surfaces: Vec<_> = walls.iter()
        .map(|t| {
            [
                Vec3::new(t.p1.x as f32, -1.,t.p1.y as f32),
                Vec3::new(t.p2.x as f32, -1.,t.p2.y as f32),
                Vec3::new((t.p1.x + t.p2.x)  as f32 / 2., 1.,(t.p1.y + t.p2.y) as f32 / 2.),
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

    // let surfaces = vec![
    //     Surface::new(
    //         [Vec3::new(87.100006, 0., 113.4) * 2., Vec3::new(87.100006, 0., 113.4) * 2., Vec3::new(87.100006, 0., 103.4)*2.],
    //         Vec3::new(1.0, 0.0, 0.0)),
    //     Surface::new(
    //         [Vec3::new(97.100006, 0., 113.4)*2., Vec3::new(87.100006, 0., 113.4)*2., Vec3::new(87.100006, 0., 113.4)*2.],
    //         Vec3::new(-0.0, 0.0, -1.0)
    //     )
    // ];

    let wsurfs: Vec<_> = surfaces.iter().filter(|s| if let Surface::Wall(_) = s {true} else {false}).collect();
    eprintln!("{:?}", wsurfs);

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let time = rl.get_time();
        let fps = rl.get_fps();

        // line src movement
        let w = rl.is_key_down(KeyboardKey::KEY_W);
        let a = rl.is_key_down(KeyboardKey::KEY_A);
        let s = rl.is_key_down(KeyboardKey::KEY_S);
        let d = rl.is_key_down(KeyboardKey::KEY_D);
        let ws = w as i8 as f32 - s as i8 as f32;
        let ad = a as i8 as f32 - d as i8 as f32;
        let wsad = Vector2::new(ad, ws).normalized();

        // change movement angle
        {
            let l = rl.is_key_down(KeyboardKey::KEY_LEFT);
            let r = rl.is_key_down(KeyboardKey::KEY_RIGHT);
            let lr = l as i8 as f32 - r as i8 as f32;

            p_rad -= lr * 2. * fd;
        }

        let move_speed = 100.;
        let p_dir = Vector2::new(0., 20. * 2.).rotated(p_rad);

        let move_dir = wsad.rotated(p_rad);
        // eprintln!("move_dir: {:?}", move_dir);

        // p_src += move_dir * move_speed * fd;

        let p_dst = p_src + move_dir * move_speed * fd;
        // eprintln!("p_src: {:?} - p_dst: {:?}", p_src, p_dst);

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

            // // dst circle
            // let pdx = p_dst.x as i32;
            // let pdy = p_dst.y as i32;
            // d.draw_circle_v(p_dst, radius * 3. / 4., Color::RED);
            // d.draw_circle_lines_v(p_dst, radius, Color::RED);
            // d.draw_text("dst", pdx + rad, pdy, rad, Color::BLACK);

            // dir path
            let dir_dst = p_src + p_dir;
            let pdx = dir_dst.x as i32;
            let pdy = dir_dst.y as i32;
            d.draw_line(psx, psy, pdx, pdy, Color::BLACK);

            // draw walls
            for wall in &surfaces {
                if let Surface::Wall(t) = wall {
                    let start = Vector2::new(t.verts()[0].x, t.verts()[0].z);
                    let end = Vector2::new(t.verts()[1].x, t.verts()[1].z);
                    d.draw_line_ex(start, end, 3., Color::BLACK);
                    let start = Vector2::new(t.verts()[0].x, t.verts()[0].z);
                    let end = Vector2::new(t.verts()[2].x, t.verts()[2].z);
                    d.draw_line_ex(start, end, 3., Color::BLACK);
                    let start = Vector2::new(t.verts()[2].x, t.verts()[2].z);
                    let end = Vector2::new(t.verts()[1].x, t.verts()[1].z);
                    d.draw_line_ex(start, end, 3., Color::BLACK);

                    let norm = t.normal();
                    let a = t.verts()[0];
                    let b = t.verts()[1];
                    let c = t.verts()[2];
                    let start = Vector2::new((a.x + b.x + c.x) / 3., (a.z + b.z + c.z) / 3.);
                    let end = start + Vector2::new(norm.x, norm.z) * 20.;
                    d.draw_line_ex(start, end, 3., Color::ORANGE);
                }
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
            let colors: Vec<Color> = (0..max_iter).map(|i| cs.lerp(ce, i as f32 / max_iter as f32)).collect();

            let mut final_stop = p_dst;

            let starting_dir = (p_dst - p_src).normalized();

            let mut lsrc = src;
            let mut ldst = dst;
            for i in 0..max_iter {
                let hotdog = HotDog::new(lsrc, ldst, radius, Vec3::new(starting_dir.x, 0., starting_dir.y));
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
                    d.draw_line_ex(hit, hit + Vector2::new(hdc.next_move.x, hdc.next_move.y), 3., colors[i]);
                } else {
                    // eprintln!("skip: {i}")
                    break;
                }
            }

            p_src = final_stop;

            // final circle
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
        }
    }
}
