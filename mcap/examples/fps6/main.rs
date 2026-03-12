use std::collections::VecDeque;
use mcap::real as mcap;

use mcap::{
    Surface, Triangle, Vec3, push_out_walls_2, find_ciel_height_hotdog_v3, find_floor_height_hotdog_v4
};
use modelz;
use raylib::prelude::*;

mod triangles;
mod grid;

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

#[derive(Clone, Copy)]
struct Player {
    pos: Vector3, // foot position
    velocity: Vector3,
    cam_pitch: f32,
    cam_yaw: f32,
    height: f32,
    chest_height: f32,
    radius: f32,
    on_ground: bool,
    snap_up: f32,
    snap_down: f32,
}

// 4x regular gravity, 4x regular terminal velocity
const GRAVITY: f32 = -36.0;
const TERMINAL_VEL: f32 = -216.0;

#[derive(Debug)]
enum CollisionMode {
    All,
    Single,
    // Cube,
}

#[derive(Debug)]
enum FpsMode {
    Uncapped,
    Cap15,
    Cap30,
    Cap60,
    Cap240,
}

struct Levels {
    names: Vec<String>,
    models: Vec<Model>,
    grids: Vec<grid::SurfaceGrid>,
    current: usize,
}

impl Levels {
    fn new_with(load: (String, Model, Vec<Surface>)) -> Self {
        Levels { names: vec![load.0], models: vec![load.1], grids: vec![grid::SurfaceGrid::new(load.2)], current: 0 }
    }

    fn push(mut self, load: (String, Model, Vec<Surface>)) -> Self {
        self.names.push(load.0);
        self.models.push(load.1);
        self.grids.push(grid::SurfaceGrid::new(load.2));

        self
    }

    fn model(&self) -> &Model {
        &self.models[self.current]
    }

    fn next(&mut self) {
        self.current = (self.current + 1) % self.names.len();
        eprintln!("tris: {}", self.grid().all_surfaces().unwrap_or_default().len());
    }

    fn current(&self) -> usize {
        self.current
    }

    fn count(&self) -> usize {
        self.names.len() - 1
    }

    fn name(&self) -> &String {
        &self.names[self.current]
    }

    fn grid(&self) -> &grid::SurfaceGrid {
        &self.grids[self.current]
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("fps6 - oot + m64 + quake")
        .build();

    let origin = at_origin(Vector3::zero());

    let mut load = |f: &str| -> (String, Model, Vec<Surface>) {
        (
        f.to_string(),
        rl.load_model(&thread, f).unwrap(),
        triangles::get_triangles(modelz::Model3D::load(f).unwrap())
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
            .collect()
        )
    };

    let bob = load("res/bob.glb");
    let nmap = load("res/nmap.glb");
    let auto2 = load("res/auto2.glb");
    let e1m1 = load("res/e1m1.glb");
    
    let mut levels = Levels::new_with(bob)
        .push(nmap)
        .push(auto2)
        .push(e1m1);

    let mut model = levels.model();

    let n_player = Player {
        pos: origin,
        velocity: Vector3::new(0., 0., 0.),
        cam_pitch: 0.,
        cam_yaw: 0.,
        height: 2.,
        chest_height: 2. *  (2. / 3.),
        radius: 2. / 3.,
        on_ground: false,
        snap_up: 1.,
        snap_down: 0.5,
    };

    let mut player = n_player;

    rl.disable_cursor();

    let mut fps_samples: VecDeque<u32> = VecDeque::new();
    let mut fps_accum_time: f32 = 0.0;
    let mut fps_accum_frames: u32 = 0;

    let mut draw_surfs = true;
    let mut collision_mode = CollisionMode::Single;
    let mut show_grid = false;
    let mut fps_mode = FpsMode::Uncapped;

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let fps = rl.get_fps();
        fps_accum_time += fd;
        fps_accum_frames += 1;
        if fps_accum_time >= 1.0 {
            if fps_samples.len() >= 60 {
                fps_samples.pop_front();
            }
            fps_samples.push_back(fps_accum_frames);
            fps_accum_time = 0.0;
            fps_accum_frames = 0;
        }
        let avg_fps = if fps_samples.is_empty() {
            0.0
        } else {
            fps_samples.iter().sum::<u32>() as f32 / fps_samples.len() as f32
        };

        let n = rl.is_key_released(KeyboardKey::KEY_N);
        if n {
            levels.next();
            model = levels.model();
            player = n_player;
        }
        let r = rl.is_key_released(KeyboardKey::KEY_R);
        if r {
            player = n_player;
        }
        let t = rl.is_key_released(KeyboardKey::KEY_T);
        if t {
            draw_surfs = !draw_surfs;
        }
        let c = rl.is_key_released(KeyboardKey::KEY_C);
        if c {
            collision_mode = match collision_mode {
                CollisionMode::All => CollisionMode::Single,
                CollisionMode::Single => CollisionMode::All,
                // CollisionMode::Cube => CollisionMode::All,
            }
        }
        let f = rl.is_key_released(KeyboardKey::KEY_F);
        if f {
            fps_mode = match fps_mode {
                FpsMode::Uncapped => { rl.set_target_fps(15); FpsMode::Cap15},
                FpsMode::Cap15 => { rl.set_target_fps(30); FpsMode::Cap30},
                FpsMode::Cap30 => { rl.set_target_fps(60); FpsMode::Cap60},
                FpsMode::Cap60 => { rl.set_target_fps(240); FpsMode::Cap240},
                FpsMode::Cap240 => { rl.set_target_fps(99999); FpsMode::Uncapped},
            }
        }
        let g = rl.is_key_released(KeyboardKey::KEY_G);
        if g {
            show_grid = !show_grid;
        }

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
        let sprint = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT);

        let forward_dir = Vector3::new(-camera_dir.x, 0.0, -camera_dir.z).normalized();
        let right_dir = -forward_dir.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        let move_dir = (forward_dir * ws + right_dir * ad).normalized();

        let speed = 100.;
        let sprint_factor = match sprint {
            true => 1.5,
            false => 1.0,
        };
        let move_speed = match player.on_ground {
            true => 1. * sprint_factor * speed,
            false => 0.8 * sprint_factor * speed,
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

        let max_move_dist = player.radius / 2.;
        let desired_move = player.velocity * fd;
        let desired_move_len = desired_move.length();

        let (move_count, move_dist) = if desired_move_len < max_move_dist {
            (1, desired_move)
        } else {
            let move_count = (desired_move.length() / max_move_dist) as u32 + 1;
            let move_dist = desired_move / move_count as f32;
            (move_count, move_dist)
        };

        let mut grid_pos = (0, 0, 0);
        let mut collision_surfaces: &[&Surface] = &vec![];
        let mut draw_floor = None;
        let mut draw_ciel = None;
        let mut snap_down = 0.;

        for _ in 0..move_count {

            let fd = fd / (move_count as f32);

            let mut pos = (player.pos + move_dist).to_mcapv3();

            grid_pos = {
                let fgpos = pos / grid::GRID_SIZE;
                (fgpos.x.floor() as u32, fgpos.y.floor() as u32, fgpos.z.floor() as u32 )
            };

            collision_surfaces = match collision_mode {
                CollisionMode::All => levels.grid().all_surfaces().unwrap_or_default(),
                CollisionMode::Single => levels.grid().surfaces_in_cell(grid_pos).unwrap_or_default(),
                // CollisionMode::Cube => &levels.grid().surfaces_in_cell_and_adjacent(grid_pos),
                _ => unreachable!(),
            };

            // wall push
            (pos, _) = push_out_walls_2(pos, player.chest_height, player.radius, collision_surfaces);

            // snap down only when on ground
            snap_down = match player.on_ground {
                true => player.snap_down,
                false => 0.
            };
            // radius is the range from player's center they start falling at
            // here, when center is half-radius off a ledge, starts falling
            match find_floor_height_hotdog_v4(pos, player.snap_up, snap_down, collision_surfaces, player.radius / 2.) {
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
                    player.velocity = player.velocity + g_slide * fd;

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
            if let Some((Surface::Cieling(ciel), y)) = find_ciel_height_hotdog_v3(pos, player.chest_height, player.radius, collision_surfaces, player.radius / 2. ) {
                pos.y = y - player.height;
                player.velocity.y = player.velocity.y.min(0.0);
                draw_ciel = Some(ciel);
            }

            player.pos = pos.to_rayv3();

        }

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
                // d3d.draw_model(&goober, origin + Vector3::one() * 5., 1., Color::WHITE);

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

                for surf in collision_surfaces {
                    match surf {
                        Surface::Wall(tri)    => if draw_surfs { draw_surf(&mut d3d, &tri, Color::GREEN.alpha(0.25))  },
                        Surface::Floor(tri)   => if draw_surfs { draw_surf(&mut d3d, &tri, Color::RED.alpha(0.25))    },
                        Surface::Slide(tri)   => if draw_surfs { draw_surf(&mut d3d, &tri, Color::BLUE.alpha(0.25))   },
                        Surface::Cieling(tri) => if draw_surfs { draw_surf(&mut d3d, &tri, Color::YELLOW.alpha(0.25)) },
                    }
                }

                if let Some(ft) = draw_floor {
                    if draw_surfs { draw_surf(&mut d3d, &ft, Color::ORANGE.alpha(0.5)) };
                }
                if let Some(ft) = draw_ciel {
                    if draw_surfs { draw_surf(&mut d3d, &ft, Color::ORANGE.alpha(0.5)) };
                }
                if show_grid {
                    let igrid = (grid_pos.0 as i32, grid_pos.1 as i32, grid_pos.2 as i32);

                    for dx in -2i32..=2 {
                        let x = (igrid.0 + dx) as f32;
                        for dy in -2i32..=2 {
                            let y = (igrid.1 + dy) as f32;
                            for dz in -2i32..=2 {
                                let z = (igrid.2 + dz) as f32;

                                let start_pos = Vector3::new(x * 10., y * 10., z * 10.);

                                let ep1 = start_pos + Vector3::new(0., 0., 10.);
                                let ep2 = start_pos + Vector3::new(0., 10., 0.);
                                let ep3 = start_pos + Vector3::new(10., 0., 0.);
                                d3d.draw_line_3D(start_pos, ep1, Color::YELLOW);
                                d3d.draw_line_3D(start_pos, ep2, Color::YELLOW);
                                d3d.draw_line_3D(start_pos, ep3, Color::YELLOW);
                            }
                        }
                    }
                }

                // player cylinder
                d3d.draw_cylinder_wires(
                    player.pos,
                    player.radius / 2.,
                    player.radius / 2.,
                    player.height,
                    16,
                    Color::GRAY,
                );

                // wall-test sphere at chest height
                d3d.draw_sphere_wires(player_chest, player.radius, 7, 7, Color::SKYBLUE);

                // step cylinder
                d3d.draw_cylinder_wires(
                    player.pos - Vector3::new(0., snap_down, 0.),
                    player.radius / 2.,
                    player.radius / 2.,
                    player.snap_up + snap_down,
                    15,
                    Color::RED,
                );

                // cieling cylinder
                d3d.draw_cylinder_wires(
                    player.pos + Vector3::new(0., player.chest_height, 0.),
                    player.radius / 2.,
                    player.radius / 2.,
                    player.radius,
                    14,
                    Color::YELLOWGREEN,
                );
            });

            d.draw_text("fps6 - oot + m64 + quake", 20, 20, 20, Color::WHITE);
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
            d.draw_text(&format!("(F)ps: {:?} {:.0}", fps_mode, fps), 20, 60, 20, Color::WHITE);
            d.draw_text(&format!("avg: {:.0}", avg_fps), 20, 80, 20, Color::WHITE);
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
            d.draw_text(
                &format!(
                    "(N)ext -- level: {}/{} -- {}",
                    levels.current() + 1, levels.count() + 1, levels.name()
                ),
                20,
                140,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "(R)eset"
                ),
                20,
                160,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "(T)ri debug: {:?}", draw_surfs
                ),
                20,
                180,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "(C)ollision mode: {:?}", collision_mode
                ),
                20,
                200,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "(G)rid cell: {:?}", grid_pos
                ),
                20,
                220,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "move count: {}", move_count
                ),
                20,
                240,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!(
                    "move dist: {:0.4} {:0.4} {:0.4}", move_dist.x, move_dist.y, move_dist.z
                ),
                20,
                260,
                20,
                Color::WHITE,
            );
        }
    }
}
