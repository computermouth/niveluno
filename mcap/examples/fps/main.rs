use mcap::{HotDog, Surface, Triangle, Vec3};
use raylib::prelude::*;

use mcap::scrap as mcap;

#[path = "../../common/mod.rs"]
mod common;
use common::{ToVec3, ToVector3};

pub fn at_origin(v: Vector3) -> Vector3 {
    v + Vector3::one() * 100.
}

struct Player {
    pos: Vector3,
    cam_pitch: f32,
    cam_yaw: f32,
    height: f32,
    chest_height: f32,
    radius: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("gui showcase").build();

    rl.set_target_fps(30);

    let origin = at_origin(Vector3::zero());

    let tri = Triangle {
        verts: [
            Vec3::new(80.0, 95.0, 120.0),
            Vec3::new(80.0, 95.0, 80.0),
            Vec3::new(80.0, 105.0, 80.0),
        ],
        normal: Vec3::new(1.0, 0.0, 0.0),
        origin_offset: -80.0,
    };
    let surfaces = vec![Surface::new(tri.verts(), tri.normal)];

    let surfaces: Vec<_> = surfaces.iter().map(|s| s).collect();

    let mut player = Player {
        // bottom of cylinder
        pos: origin - Vector3::new(0., 2.25, 0.),
        cam_pitch: 0.,
        cam_yaw: 0.,
        height: 3.,
        chest_height: 2.,
        radius: 1.,
    };

    rl.disable_cursor();

    let mut total = 0.;
    let mut fc: f32 = 0.;

    let mut last_hotdog = None;
    let mut fail_open = false;

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let fps = rl.get_fps();

        total += fps as f32;
        fc += 1.0;

        let mouse_in = rl.get_mouse_delta();
        player.cam_pitch = (player.cam_pitch + mouse_in.y * 0.0015).clamp(-0.9, 0.9);
        player.cam_yaw += mouse_in.x * 0.0015;

        let camera_dir = Vector3::new(
            player.cam_yaw.cos() * player.cam_pitch.cos(),
            player.cam_pitch.sin(),
            player.cam_yaw.sin() * player.cam_pitch.cos(),
        )
        .normalized();

        if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT))
            && rl.is_key_pressed(KeyboardKey::KEY_ENTER)
        {
            rl.toggle_fullscreen();
        }

        // player horizontal movement
        let w = rl.is_key_down(KeyboardKey::KEY_W);
        let a = rl.is_key_down(KeyboardKey::KEY_A);
        let s = rl.is_key_down(KeyboardKey::KEY_S);
        let d = rl.is_key_down(KeyboardKey::KEY_D);
        let ws = w as i8 as f32 - s as i8 as f32;
        let ad = a as i8 as f32 - d as i8 as f32;

        let forward_dir = Vector3::new(-camera_dir.x, 0.0, -camera_dir.z).normalized();
        let right_dir = -forward_dir.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();

        let move_speed = 10.0;
        let move_dir = (forward_dir * ws + right_dir * ad).normalized();

        let src = player.pos + Vector3::new(0., player.chest_height, 0.);
        let dst = src + (move_dir * move_speed * fd);

        let max_iter = 5;
        // let src = Vector3::new(81.000046, 99.75, 88.801704);
        // let dst = Vector3::new(80.99945, src.y, 89.13504);
        // let move_dir = (dst - src).normalized();

        let mut lpos = src.to_mcapv3();
        let mut ldst = dst.to_mcapv3();
        let mut lout = ldst.with_y(ldst.y - player.chest_height);

        for i in 0..max_iter {
            let hotdog = HotDog::new(lpos, ldst, player.radius, move_dir.to_mcapv3());

            // on no-collision or no-move
            let mut exit_early = true;

            if let Some(hdc) = hotdog.check_walls_c2(&surfaces) {
                exit_early = false;

                eprintln!("hotdog: {:?} hdc: {:?}", hotdog, hdc);

                // update current position
                lpos = Vec3::new(hdc.dest_xz.x, lpos.y, hdc.dest_xz.y);

                // set up next destination
                ldst = lpos + Vec3::new(hdc.next_move.x, 0., hdc.next_move.y);

                // bailing on no-move collision
                if hdc.next_move_len == 0. {
                    ldst = lpos;
                    exit_early = true;
                }

                // if final collision, ditch remaining dst
                if i == max_iter - 1 {
                    fail_open = true;
                    ldst = lpos;
                }

                // if next move is basically 0, exit after floor check
                if hdc.next_move_len < f32::EPSILON {
                    ldst = lpos;
                    exit_early = true;
                }

                if fail_open {
                    eprintln!("hotdog: {:?}", hotdog);
                    eprintln!("last_hotdog: {:?}", last_hotdog);
                }

                last_hotdog = Some(hotdog);
            }

            lout = ldst.with_y(ldst.y - player.chest_height);

            if exit_early {
                break;
            }
        }

        player.pos = lout.to_rayv3();

        // calculate cam pos
        let player_top = player.pos + Vector3::new(0., player.height, 0.);
        let player_chest = player.pos + Vector3::new(0., player.chest_height, 0.);
        let player_step_top =
            player.pos + Vector3::new(0., player.height - player.chest_height, 0.);
        let player_step_bot =
            player.pos - Vector3::new(0., player.height - player.chest_height, 0.);
        let camera = Camera3D::perspective(
            player_top + camera_dir * 5.,
            player_top,
            Vector3::new(0.0, 1.0, 0.0),
            90.0,
        );

        let mut d = rl.begin_drawing(&thread);
        {
            // draws

            d.clear_background(Color::new(16, 16, 32, 255));
            d.draw_mode3D(camera, |mut d3d, _| {
                d3d.draw_triangle3D(
                    tri.verts[0].to_rayv3(),
                    tri.verts[1].to_rayv3(),
                    tri.verts[2].to_rayv3(),
                    Color::WHITE,
                );

                fn draw_surf(
                    d3d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>,
                    tri: &Triangle,
                    color: Color,
                ) {
                    let v = tri.verts();
                    let t1 = v[0].to_rayv3();
                    let t2 = v[1].to_rayv3();
                    let t3 = v[2].to_rayv3();
                    d3d.draw_triangle3D(t1, t2, t3, color);
                    d3d.draw_line_3D(t1, t2, Color::WHITE);
                    d3d.draw_line_3D(t1, t3, Color::WHITE);
                    d3d.draw_line_3D(t3, t2, Color::WHITE);
                }

                for surf in &surfaces {
                    match surf {
                        Surface::Wall(tri) => draw_surf(&mut d3d, tri, Color::GREEN.alpha(0.5)),
                        Surface::Floor(tri) => draw_surf(&mut d3d, tri, Color::RED.alpha(0.5)),
                        Surface::Slide(tri) => draw_surf(&mut d3d, tri, Color::BLUE.alpha(0.5)),
                        Surface::Cieling(tri) => draw_surf(&mut d3d, tri, Color::YELLOW.alpha(0.5)),
                    }
                }

                // player cylinder
                d3d.draw_cylinder_wires(
                    player.pos,
                    player.radius,
                    player.radius,
                    player.height,
                    16,
                    Color::YELLOW,
                );

                // collision circle
                d3d.draw_circle_3D(
                    player_chest,
                    player.radius,
                    Vector3::new(1., 0., 0.),
                    90.,
                    Color::SKYBLUE,
                );
                // top step circle
                d3d.draw_circle_3D(
                    player_step_top,
                    player.radius,
                    Vector3::new(1., 0., 0.),
                    90.,
                    Color::RED,
                );
                // bottom step circle
                d3d.draw_circle_3D(
                    player_step_bot,
                    player.radius,
                    Vector3::new(1., 0., 0.),
                    90.,
                    Color::RED,
                );
            });

            d.draw_text(&format!("FPS Demo"), 20, 20, 20, Color::WHITE);
            d.draw_text(
                &format!(
                    "p: {:.1} {:.1} {:.1}",
                    player.pos.x, player.pos.y, player.pos.z
                ),
                20,
                40,
                20,
                Color::WHITE,
            );
            d.draw_text(&format!("fps: {}", fps), 20, 60, 20, Color::WHITE);
            d.draw_text(&format!("avg: {:.0}", total / fc), 20, 80, 20, Color::WHITE);
        }
    }
}
