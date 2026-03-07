use mcap::real as mcap;

use mcap::{
    Surface, Triangle, Vec3, push_out_walls_2, find_ciel_height_hotdog_v3, find_floor_height_hotdog_v4
};
use modelz;
use raylib::prelude::*;

mod triangles;

pub fn get_face_normal(v1_pos: Vec3, v2_pos: Vec3, v3_pos: Vec3) -> Vec3 {
    let edge1 = v2_pos - v1_pos;
    let edge2 = v3_pos - v1_pos;

    edge1.cross(edge2).normalize()
}

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
    pos: Vector3, // foot position
    velocity: Vector3,
    cam_pitch: f32,
    cam_yaw: f32,
    height: f32,
    chest_height: f32,
    radius: f32,
    on_ground: bool,
}

// 4x regular gravity, 4x regular terminal velocity
const GRAVITY: f32 = -36.0;
const TERMINAL_VEL: f32 = -216.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("fps6 - OoT style")
        .build();

    let origin = at_origin(Vector3::zero());

    let model = rl.load_model(&thread, "res/nmap.glb").unwrap();
    let collision_triangles =
        triangles::get_triangles(modelz::Model3D::load("res/nmap.glb").unwrap());
    let surfaces: Vec<_> = collision_triangles
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

    let wall_tris: Vec<&Triangle> = surfaces
        .iter()
        .filter_map(|s| match s {
            Surface::Wall(t) => Some(t),
            _ => None,
        })
        .collect();

    let floors: Vec<&Surface> = surfaces
        .iter()
        .filter(|s| matches!(s, Surface::Floor(_) | Surface::Slide(_)))
        .collect();

    let cielings: Vec<&Surface> = surfaces
        .iter()
        .filter(|s| matches!(s, Surface::Cieling(_)))
        .collect();

    let mut player = Player {
        pos: origin - Vector3::new(0., 5., 0.),
        velocity: Vector3::new(0., 0., 0.),
        cam_pitch: 0.,
        cam_yaw: 0.,
        height: 3.,
        chest_height: 2.,
        radius: 1.,
        on_ground: false,
    };

    rl.disable_cursor();

    let mut total = 0.;
    let mut fc: f32 = 0.;

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
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

        if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT))
            && rl.is_key_pressed(KeyboardKey::KEY_ENTER)
        {
            rl.toggle_fullscreen();
        }

        let w = rl.is_key_down(KeyboardKey::KEY_W);
        let a = rl.is_key_down(KeyboardKey::KEY_A);
        let s = rl.is_key_down(KeyboardKey::KEY_S);
        let d = rl.is_key_down(KeyboardKey::KEY_D);
        let ws = w as i8 as f32 - s as i8 as f32;
        let ad = a as i8 as f32 - d as i8 as f32;
        let space = rl.is_key_down(KeyboardKey::KEY_SPACE);

        let forward_dir = Vector3::new(-camera_dir.x, 0.0, -camera_dir.z).normalized();
        let right_dir = -forward_dir.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();

        let move_dir = (forward_dir * ws + right_dir * ad).normalized();
        let move_speed = match player.on_ground {
            true => 100.0,
            false => 80.0,
        };
        let friction = match player.on_ground {
            true => 10.,
            false => 9.,
        };
        let friction_factor = 1.0 - (friction * fd).min(1.0);
        player.velocity = player.velocity * Vector3::new(friction_factor, 1., friction_factor);
        player.velocity = player.velocity + move_dir.scale_by(move_speed) * fd;

        if space && player.on_ground {
            player.velocity.y = 15.;
        }

        let mut pos = (player.pos + player.velocity * fd).to_mcapv3();

        // wall push
        (pos, _) = push_out_walls_2(pos, player.chest_height, player.radius, &wall_tris);

        // floor find, snap, gravity
        let snap = 1.;
        let mut draw_floor = None;
        let mut draw_ciel = None;

        // radius is the range from player's center they start falling at
        // here, when center is half-radius off a ledge, starts falling
        match find_floor_height_hotdog_v4(pos, snap, snap / 4., &floors, player.radius / 2.) {
            Some((Surface::Floor(floor), y)) => {
                // don't floor snap if we're not moving down
                // mitigates not reaching escape velocity of snap with jump
                if player.velocity.y <= 0. {
                    pos.y = y;
                    player.velocity.y = player.velocity.y.max(0.0);
                    player.on_ground = true;
                    draw_floor = Some(floor);
                } else {
                    // falling
                    player.velocity.y = (player.velocity.y + GRAVITY * fd).max(TERMINAL_VEL);
                    player.on_ground = false;
                }
            }
            Some((Surface::Slide(slide), y)) => {
                pos.y = y;
                let n = slide.normal().to_rayv3();
                let g = Vector3::new(0.0, GRAVITY, 0.0);
                let g_slide = g - n * g.dot(n);

                // remove velocity into the slope
                let vel_into_slope = n * player.velocity.dot(n);
                player.velocity = player.velocity - vel_into_slope;

                // just feels better with 2x grav
                player.velocity = player.velocity + g_slide * fd * 2.;

                player.on_ground = false;
                draw_floor = Some(slide);
            }
            _ => {
                // falling
                player.velocity.y = (player.velocity.y + GRAVITY * fd).max(TERMINAL_VEL);
                player.on_ground = false;
            }
        }

        // cieling clamp
        if let Some((Surface::Cieling(ciel), y)) = find_ciel_height_hotdog_v3(pos, player.chest_height, player.radius, &cielings, player.radius / 2. ) {
            pos.y = y - player.height;
            player.velocity.y = player.velocity.y.min(0.0);
            draw_ciel = Some(ciel);
        }

        player.pos = pos.to_rayv3();

        let player_top = player.pos + Vector3::new(0., player.height, 0.);
        let player_chest = player.pos + Vector3::new(0., player.chest_height, 0.);

        let camera = Camera3D::perspective(
            player_top + camera_dir * 5.,
            player_top,
            Vector3::new(0.0, 1.0, 0.0),
            90.0,
        );

        let mut d = rl.begin_drawing(&thread);
        {
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

                if let Some(ft) = draw_floor {
                    draw_surf(&mut d3d, &ft, Color::ORANGE.alpha(0.5));
                }
                if let Some(ft) = draw_ciel {
                    draw_surf(&mut d3d, &ft, Color::ORANGE.alpha(0.5));
                }

                // player cylinder
                d3d.draw_cylinder_wires(
                    player.pos,
                    player.radius / 2.,
                    player.radius / 2.,
                    player.height,
                    16,
                    Color::YELLOW,
                );

                // wall-test sphere at chest height
                d3d.draw_sphere_wires(player_chest, player.radius, 7, 7, Color::SKYBLUE);

                // step cylinder
                d3d.draw_cylinder_wires(
                    player.pos - Vector3::new(0., snap / 4., 0.),
                    player.radius,
                    player.radius,
                    snap / 4. + snap,
                    15,
                    Color::RED,
                );

                // cieling cylinder
                d3d.draw_cylinder_wires(
                    player.pos + Vector3::new(0., player.chest_height, 0.),
                    player.radius,
                    player.radius,
                    player.height - player.chest_height,
                    14,
                    Color::YELLOWGREEN,
                );
            });

            d.draw_text("fps6 - oot", 20, 20, 20, Color::WHITE);
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
                &format!("on_ground: {:?}", player.on_ground),
                20,
                100,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "velocity: {:0.2} {:0.2} {:0.2}",
                    player.velocity.x, player.velocity.y, player.velocity.z
                ),
                20,
                120,
                20,
                Color::WHITE,
            );
        }
    }
}
