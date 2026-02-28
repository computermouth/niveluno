use std::{iter, ops::Add};

use glam::Vec2;
use mcap::{
    HotDog, HotDogv2, Surface, Triangle, Vec3, find_floor_height_hotdog, find_floor_height_hotdog_v2, find_floor_height_m64, get_face_normal, get_step_push, get_step_push_m64, get_step_push_most_opposing
};
use modelz;
use raylib::prelude::*;

mod triangles;

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

pub fn at_origin(v: Vector3) -> Vector3 {
    v + Vector3::one() * 100.
}

struct Player {
    pos: Vector3,
    vel: Vector3,
    cam_pitch: f32,
    cam_yaw: f32,
    height: f32,
    snap: f32,
    radius: f32,
    airborne: bool,
}

// this was my pride and joy amongst the attempts in this repo to make
// something robust and ray-based, but there's still issues with the
// 2d circular test approach. it seems that a wall push is going to be
// unavoidable in certain scenarios, or at least, pretty hairy to work around

// all in all, the 2d circular test approach would work great if it weren't for
// gravity and jumping. everything works until integrating vertical movement
// outside of floor snaps.

// anyway, abandoned for general purpose use, and honestly a wall-push
// is probably more performant in any case, and will work just as well for fps
// as it would for like pokemon-esque flat plane movement

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("gui showcase").build();

    let origin = at_origin(Vector3::zero());

    let model = rl.load_model(&thread, "res/nmap.glb").unwrap();
    let collison_triangles =
        triangles::get_triangles(modelz::Model3D::load("res/nmap.glb").unwrap());
    let surfaces: Vec<_> = collison_triangles
        .iter()
        .map(|t| {
            Surface::new(
                [
                    (t[0] + origin).to_mcapv3(),
                    (t[1] + origin).to_mcapv3(),
                    (t[2] + origin).to_mcapv3(),
                ],
                get_face_normal(t[0].to_mcapv3(), t[1].to_mcapv3(), t[2].to_mcapv3()),
            )
        })
        .collect();

    let walls: Vec<_> = surfaces.iter().filter(|s| match s {
        Surface::Wall(_) => true,
        _ => false,
    }).collect();
    let floors: Vec<_> = surfaces.iter().filter(|s| match s {
        Surface::Floor(_) | Surface::Slide(_) => true,
        _ => false,
    }).collect();

    let mut player = Player {
        // bottom of cylinder
        pos: origin - Vector3::new(0., 5., 0.),
        vel: Vector3::zero(),
        cam_pitch: 0.,
        cam_yaw: 0.,
        height: 3.,
        snap: 1.,
        radius: 1.,
        airborne: true,
    };

    rl.disable_cursor();

    let mut total = 0.;
    let mut fc: f32 = 0.;

    let mut findex = 0;

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let time = rl.get_time();
        let fps = rl.get_fps();

        total += fps as f32;
        fc += 1.0;

        let mouse_in = rl.get_mouse_delta();
        rl.set_mouse_position(Vector2::new(320., 240.));
        player.cam_pitch = (player.cam_pitch + mouse_in.y * 0.0015).clamp(-0.9, 0.9);
        player.cam_yaw += mouse_in.x * 0.0015;

        let camera_dir = Vector3::new(
            player.cam_yaw.cos() * player.cam_pitch.cos(),
            player.cam_pitch.sin(),
            player.cam_yaw.sin() * player.cam_pitch.cos(),
        )
        .normalized();

        if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT)) &&
            rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
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

        let src = player.pos;
        let dst = src + (move_dir * move_speed * fd);

        let mut floor_draw = None;
        
        let max_iter = 5;

        let mut lpos = src.to_mcapv3();
        let mut ldst = dst.to_mcapv3();
        let mut lout = ldst;
        for i in 0..max_iter {
            // the body of this should be a function to invoke
            let hotdog_top = HotDogv2::new(lpos, ldst, player.radius, player.height - 0.002, 0.002, move_dir.to_mcapv3());
            let hotdog_bot = HotDogv2::new(lpos, ldst, player.radius, player.snap + 0.002, 0.002, move_dir.to_mcapv3());
            
            // on no-collision or no-move
            let mut exit_early = true;

            // I don't think this is gonna work out
            let first_hdc = match (
                hotdog_top.check_walls_c2(&walls),
                None,
                // hotdog_bot.check_walls_c2(&walls)
            ) {
                (None, None) => None,
                (Some(hdc), None) => Some(hdc),
                (None, Some(hdc)) => Some(hdc),
                (Some(hdc1), Some(hdc2)) => {
                    if hdc1.next_move_len < hdc2.next_move_len {
                        Some(hdc1)
                    } else {
                        Some(hdc2)
                    }
                }
            };
            
            if let Some(hdc) = first_hdc{
                exit_early = false;

                // update current position
                lpos = Vec3::new(hdc.dest_xz.x, lpos.y, hdc.dest_xz.y);

                // set up next destination
                ldst = lpos + Vec3::new(hdc.next_move.x, 0., hdc.next_move.y);

                // if final collision, ditch remaining dst
                if i == max_iter - 1 {
                    ldst = lpos;
                }

                // if next move is basically 0, exit after floor check
                if hdc.next_move_len < f32::EPSILON {
                    ldst = lpos;
                    exit_early = true;
                }
            }

            lout = ldst;

            // if let Some((floor, y)) = find_floor_height_closest_v2(lout, snap, &floors, player.radius) {
            // if let Some((floor, y)) = find_floor_height_hotdog(lout, snap, &floors, player.radius) {
            if let Some((floor, y)) = find_floor_height_hotdog_v2(lout, player.snap, &floors, player.radius) {
            // if let Some((floor, y)) = find_floor_height_m64(lout, snap, &floors) {
                lout.y = y - player.radius * 0.001;
                // // todo, apply this to inter-frame velocity
                // // zero out y, project step onto floor normal
                // step.y = 0.;
                // step -= floor.normal * step.dot(floor.normal);
                player.airborne = false;
                floor_draw = Some(floor);
            } else {
                // walked off ledge, become airborne
                player.airborne = true;
            }

            if exit_early || i == max_iter - 1 {
                break;
            }
        }


        // gravity pass
        //
        // gravity pass is it's own thing because
        // if we just do the above with gravity,
        // check_walls_c2 will prevent our downward movement
        // when colliding with a wall
        let mut lpos = lout;
        let mut ldst = lpos.with_y(lpos.y - 9.8 * fd);
        let mut lout = lout;

        if player.airborne {

            let hotdog = HotDogv2::new(lpos, ldst, player.radius, player.height - 0.002, 0.002, move_dir.to_mcapv3());
            
            if let Some(hdc) = hotdog.check_walls_c2(&walls){
                // update current position
                lpos = Vec3::new(hdc.dest_xz.x, lpos.y, hdc.dest_xz.y);

                // set up next destination
                ldst = lpos + Vec3::new(hdc.next_move.x, 0., hdc.next_move.y);

                // if next move is basically 0, exit after floor check
                if hdc.next_move_len < f32::EPSILON {
                    ldst = lpos;
                }
            }

            lout = ldst;

            // if let Some((floor, y)) = find_floor_height_closest_v2(lout, snap, &floors, player.radius) {
            // if let Some((floor, y)) = find_floor_height_hotdog(lout, snap, &floors, player.radius) {
            if let Some((floor, y)) = find_floor_height_hotdog_v2(lout, player.snap, &floors, player.radius) {
            // if let Some((floor, y)) = find_floor_height_m64(lout, snap, &floors) {
                lout.y = y - player.radius * 0.001;
                // // todo, apply this to inter-frame velocity
                // // zero out y, project step onto floor normal
                // step.y = 0.;
                // step -= floor.normal * step.dot(floor.normal);
                player.airborne = false;
                floor_draw = Some(floor);
            } else {
                // walked off ledge, become airborne
                player.airborne = true;
            }
        }

        player.pos = lout.to_rayv3();

        // calculate cam pos
        let player_top = player.pos + Vector3::new(0., player.height, 0.);
        let player_bot_collision = player.pos + Vector3::new(0., player.snap + 0.1, 0.);
        let player_step_top =
            player.pos + Vector3::new(0., player.snap, 0.);
        let player_step_bot =
            player.pos - Vector3::new(0., player.snap, 0.);
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
                d3d.draw_model(&model, origin, 1.0, Color::WHITE);

                fn draw_surf(
                    d3d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>,
                    tri: &Triangle,
                    color: Color,
                ) {
                    let v = tri.verts();
                    let t1 = (v[0] + (tri.normal() * 0.05)).to_rayv3();
                    let t2 = (v[1] + (tri.normal() * 0.05)).to_rayv3();
                    let t3 = (v[2] + (tri.normal() * 0.05)).to_rayv3();
                    d3d.draw_triangle3D(t1, t2, t3, color);
                    d3d.draw_line_3D(t1, t2, Color::WHITE);
                    d3d.draw_line_3D(t1, t3, Color::WHITE);
                    d3d.draw_line_3D(t3, t2, Color::WHITE);

                    let center = (t1 + t2 + t3) / 3.;
                    d3d.draw_line_3D(center, center + tri.normal().to_rayv3(), Color::ORANGE);
                }

                for surf in &surfaces {
                    match surf {
                        Surface::Wall(tri) => draw_surf(&mut d3d, tri, Color::GREEN.alpha(0.5)),
                        Surface::Floor(tri) => draw_surf(&mut d3d, tri, Color::RED.alpha(0.5)),
                        Surface::Slide(tri) => draw_surf(&mut d3d, tri, Color::BLUE.alpha(0.5)),
                        Surface::Cieling(tri) => draw_surf(&mut d3d, tri, Color::YELLOW.alpha(0.5)),
                    }
                }

                if let Some(ft) = floor_draw {
                    draw_surf(&mut d3d, &ft, Color::ORANGE.alpha(0.5));
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
                    player_top,
                    player.radius,
                    Vector3::new(1., 0., 0.),
                    90.,
                    Color::SKYBLUE,
                );
                // collision circle
                d3d.draw_circle_3D(
                    player_bot_collision,
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
            d.draw_text(
                &format!("func: HotDogWalls"),
                20,
                100,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!("airborn: {:?}", player.airborne),
                20,
                120,
                20,
                Color::WHITE,
            );
        }
    }
}
