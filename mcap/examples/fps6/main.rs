use glam::Vec2;
use mcap::{Surface, Triangle, Vec3, closest_point_triangle, find_floor_height_m64, get_face_normal};
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

pub fn push_out_walls_2(
    pos: Vec3,
    check_height: f32,
    radius: f32,
    walls: &Vec<&Triangle>,
) -> (Vec3, bool) {
    let sph_y = pos.y + check_height;
    let mut out_x = pos.x;
    let mut out_z = pos.z;
    let mut hit = false;

    for tri in walls {
        let n = tri.normal;
        let xz_len = (n.x * n.x + n.z * n.z).sqrt();

        let cur = Vec3::new(out_x, sph_y, out_z);
        let nearest = closest_point_triangle(cur, &tri.verts);
        
        let dist = nearest.distance(cur);

        if dist.abs() >= radius {
            continue;
        }

        // get appropriate deflection dir
        let diff = cur - nearest;
        let diff_xz = Vec2::new(diff.x, diff.z);
        let diff_xz_len = diff_xz.length();

        let (push_dir_x, push_dir_z) = if diff_xz_len > f32::EPSILON {
            (diff_xz.x / diff_xz_len, diff_xz.y / diff_xz_len)
        } else {
            // diff is ~ 0
            // center is directly above/below the nearest point
            // fall back to face normal
            //
            // we should be able to ensure this doesn't happen
            // by padding downward raycasts a little bit
            (n.x / xz_len, n.z / xz_len)
        };

        let push = radius - dist;
        out_x += push_dir_x * push;
        out_z += push_dir_z * push;
        hit = true;
    }

    (Vec3::new(out_x, pos.y, out_z), hit)
}

struct Player {
    pos: Vector3, // foot position
    vel_y: f32,
    cam_pitch: f32,
    cam_yaw: f32,
    height: f32,
    chest_height: f32,
    radius: f32,
    on_ground: bool,
}

const GRAVITY: f32 = -9.8;
const TERMINAL_VEL: f32 = -20.0;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("fps6 - OoT style").build();

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

    let mut player = Player {
        pos: origin - Vector3::new(0., 5., 0.),
        vel_y: 0.,
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

        if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT))
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

        let forward_dir = Vector3::new(-camera_dir.x, 0.0, -camera_dir.z).normalized();
        let right_dir = -forward_dir.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();

        let move_speed = 10.0;
        let move_dir = (forward_dir * ws + right_dir * ad).normalized();

        let mut pos = player.pos.to_mcapv3();
        pos.x += move_dir.x * move_speed * fd;
        pos.z += move_dir.z * move_speed * fd;
        pos.y += player.vel_y * fd;

        // wall push
        (pos, _) = push_out_walls_2(pos, player.chest_height, player.radius, &wall_tris);

        // floor find, snap, gravity
        let snap = 1.;
        let mut draw_floor = None;
        if let Some((floor, y)) = find_floor_height_m64(pos, snap, &floors) {
            pos.y = y;
            player.vel_y = player.vel_y.max(0.0);
            player.on_ground = true;
            draw_floor = Some(floor);
        } else {
            player.vel_y = (player.vel_y + GRAVITY * fd).max(TERMINAL_VEL);
            player.on_ground = false;
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

                // player cylinder
                d3d.draw_cylinder_wires(
                    player.pos,
                    player.radius,
                    player.radius,
                    player.height,
                    16,
                    Color::YELLOW,
                );

                // wall-test sphere at chest height
                d3d.draw_sphere_wires(player_chest, player.radius, 7, 7, Color::SKYBLUE);
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
                &format!("vel_y: {:.2}", player.vel_y),
                20,
                120,
                20,
                Color::WHITE,
            );
        }
    }
}
