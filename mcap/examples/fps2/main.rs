use mcap::{Surface, Triangle, Vec3, get_face_normal, get_step_push, get_step_push_most_opposing};
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
    chest_height: f32,
    radius: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("gui showcase").build();

    let origin = at_origin(Vector3::zero());

    let model = rl.load_model(&thread, "res/map2.glb").unwrap();
    let collison_triangles =
        triangles::get_triangles(modelz::Model3D::load("res/map2.glb").unwrap());
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

    let player_start = origin + Vector3::new(10.0, -1.5, -8.0);

    let mut player = Player {
        // bottom of cylinder
        pos: player_start,
        vel: Vector3::zero(),
        cam_pitch: 0.,
        cam_yaw: 0.,
        height: 3.,
        chest_height: 2.,
        radius: 1.,
    };

    rl.disable_cursor();

    let mut total = 0.;
    let mut fc: f32 = 0.;

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let time = rl.get_time();
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

        // player horizontal movement
        let w = rl.is_key_down(KeyboardKey::KEY_W);
        let a = rl.is_key_down(KeyboardKey::KEY_A);
        let s = rl.is_key_down(KeyboardKey::KEY_S);
        let d = rl.is_key_down(KeyboardKey::KEY_D);
        let ws = w as i8 as f32 - s as i8 as f32;
        let ad = a as i8 as f32 - d as i8 as f32;

        let forward_dir = Vector3::new(-camera_dir.x, 0.0, -camera_dir.z).normalized();
        let right_dir = -forward_dir.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();

        // frame velocity
        let move_speed = 10.0;
        let movement = (forward_dir * ws + right_dir * ad) * move_speed * fd;

        let iterations = 8;

        let tic = movement / iterations as f32;
        for _ in 0..iterations {
            let res = get_step_push_most_opposing(
                player.pos.to_mcapv3(),
                tic.to_mcapv3(),
                player.radius,
                player.chest_height,
                player.height - player.chest_height,
                &surfaces
            );

            player.pos = res.final_pos.to_rayv3();
        }

        // calculate cam pos
        let player_top = player.pos + Vector3::new(0., player.height, 0.);
        let player_chest = player.pos + Vector3::new(0., player.chest_height, 0.);
        let player_step_top = player.pos + Vector3::new(0., player.height - player.chest_height, 0.);
        let player_step_bot = player.pos - Vector3::new(0., player.height - player.chest_height, 0.);
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
                    let t1 = v[0].to_rayv3();
                    let t2 = v[1].to_rayv3();
                    let t3 = v[2].to_rayv3();
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
                d3d.draw_circle_3D(player_chest, player.radius, Vector3::new(1., 0., 0.), 90., Color::SKYBLUE);
                // top step circle
                d3d.draw_circle_3D(player_step_top, player.radius, Vector3::new(1., 0., 0.), 90., Color::RED);
                // bottom step circle
                d3d.draw_circle_3D(player_step_bot, player.radius, Vector3::new(1., 0., 0.), 90., Color::RED);
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
