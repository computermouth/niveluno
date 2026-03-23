use core::{f32, panic};

use mcap::Surface;
use raymath::{
    matrix_rotate_y, matrix_translate, vector2_add, vector2_distance, vector2_dot_product,
    vector2_length, vector2_normalize, vector2_scale, vector2_subtract, vector3_add,
    vector3_distance, vector3_dot_product, vector3_length, vector3_multiply, vector3_negate,
    vector3_normalize, vector3_scale, vector3_subtract, vector3_transform, BoundingBox,
    RayCollision, Vector2,
};
use sdl2::rect::Point;

use crate::g_game::TopState;
use crate::g_instance::{get_decor_instances, Instance};
use crate::map::{self, Entity};
use crate::math::{
    closest_point_to_triangle, get_ray_collision_mesh, mesh_tranform, vec3_face_normal, Vector3,
};
use crate::text::{self, OverlaySurface};
use crate::{asset, g_game};
use crate::{g_instance, input};
use crate::{render, time};

pub struct Player {
    base: Entity,
    pitch: f32,
    yaw: f32,
    position: Vector3,
    hud: Box<text::OverlaySurface>,
    speed: f32,
    acceleration: Vector3,
    velocity: Vector3,
    on_ground: bool,
    last_floor: Vector3,
    bid: Option<u32>,
    friction: f32,
    height: f32,
    radius: f32,
    chest_height: f32,
    snap_up: f32,
    snap_down: f32,
    opt_ass: Option<OptAssets>,
}

pub struct OptAssets {
    encounter_bar: Box<text::OverlaySurface>,
    encounter_bar_frame: Box<text::OverlaySurface>,
}

// 4x regular gravity, 4x regular terminal velocity
const GRAVITY: f32 = -36.0;
const TERMINAL_VEL: f32 = -216.0;

impl Player {
    pub fn new(entt: &Entity) -> Self {
        // timed surface on spawn
        let mut spawn = text::create_text_overlay_surface(text::TextInput {
            text: "SPAWN".to_string(),
            mode: text::Mode::Solid {
                color: text::FontColor {
                    r: 32,
                    g: 196,
                    b: 64,
                    a: 255,
                },
            },
            font: g_game::get_text_font_lg().unwrap(),
        })
        .unwrap();

        spawn.dst_rect.set_x(100);
        spawn.dst_rect.set_y(100);

        let ts = text::TimedSurface::new(spawn, 1000);

        text::push_timed_surface(ts).unwrap();

        let snap_up = 1.;
        let radius = 1.;
        let chest_height = snap_up * 0.7 + radius;
        let height = chest_height + radius;

        Self {
            base: entt.clone(),
            pitch: 0.,
            yaw: 0.,
            position: entt.location.into(),
            speed: 96.,
            acceleration: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            velocity: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            last_floor: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            bid: None,
            on_ground: true,
            friction: 0.3,
            hud: match g_game::get_state().unwrap() {
                TopState::Menu => text::create_text_overlay_surface(text::TextInput {
                    text: "MAIN MENU".to_string(),
                    mode: text::Mode::Solid {
                        color: text::FontColor {
                            r: 255,
                            g: 167,
                            b: 167,
                            a: 255,
                        },
                    },
                    font: g_game::get_text_font_lg().unwrap(),
                })
                .unwrap(),
                TopState::Play => text::create_text_overlay_surface(text::TextInput {
                    text: "󰊠󰘉".to_string(),
                    mode: text::Mode::Solid {
                        color: text::FontColor {
                            r: 167,
                            g: 167,
                            b: 255,
                            a: 255,
                        },
                    },
                    font: g_game::get_symb_font().unwrap(),
                })
                .unwrap(),
            },
            height: height,
            radius: radius, 
            chest_height: chest_height,
            snap_up: snap_up,
            snap_down: 0.5,
            opt_ass: match g_game::get_state().unwrap() {
                TopState::Menu => None,
                TopState::Play => {
                    let mut encounter_bar = text::create_png_overlay_surface(
                        asset::get_file("img/encounter_bar.png").unwrap().unwrap(),
                    )
                    .unwrap();

                    let mut encounter_bar_frame = text::create_png_overlay_surface(
                        asset::get_file("img/encounter_bar_frame.png")
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    encounter_bar_frame
                        .dst_rect
                        .set_x(render::INTERNAL_W / 2 - encounter_bar_frame.w as i32 / 2);
                    encounter_bar_frame
                        .dst_rect
                        .set_y(render::INTERNAL_H - 2 * encounter_bar_frame.h as i32);

                    encounter_bar
                        .dst_rect
                        .set_x(encounter_bar_frame.dst_rect.x() + 3);
                    encounter_bar
                        .dst_rect
                        .set_y(encounter_bar_frame.dst_rect.y() + 6);

                    Some(OptAssets {
                        encounter_bar,
                        encounter_bar_frame,
                    })
                }
            },
        }
    }

    pub fn update_hud(&mut self) {
        text::push_surface(&self.hud).unwrap();

        match g_game::get_state().unwrap() {
            TopState::Play => {
                if self.opt_ass.is_some() {
                    let eb = &mut self.opt_ass.as_mut().unwrap().encounter_bar;
                    let w = eb.w as f64 * (time::get_run_time().unwrap() % 20.) / 20.;
                    eb.src_rect.set_width(w as u32);
                    eb.dst_rect.set_width(w as u32);
                    text::push_surface(&self.opt_ass.as_ref().unwrap().encounter_bar).unwrap();
                    text::push_surface(&self.opt_ass.as_ref().unwrap().encounter_bar_frame)
                        .unwrap();
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let _ = self.base;

        let keys = input::get_keys().unwrap();

        if g_game::get_state().unwrap() == TopState::Menu && keys[input::Key::Jump as usize] == true
        {
            let nmap = asset::get_file("map/nmap.mp").unwrap().unwrap();
            let payload = mparse::unmarshal(&nmap).unwrap();
            let level = map::load(payload).unwrap();
            g_game::set_state(TopState::Play).unwrap();
            g_game::stage_level(level.clone()).unwrap();
            return;
        } else if g_game::get_state().unwrap() == TopState::Menu {
            render::set_camera_pos(self.position).unwrap();
            return;
        }

        let (mouse_x, mouse_y) = input::get_mouse().unwrap();
        self.pitch = (self.pitch + mouse_y * 0.00015).clamp(-1.5, 1.5);
        self.yaw = (self.yaw + mouse_x * 0.00015) % (2. * std::f32::consts::PI);

        render::set_camera_pitch(self.pitch).unwrap();
        render::set_camera_yaw(self.yaw).unwrap();

        self.update_physics();

        let mut bid = None;
        let bis = g_instance::get_barrier_instances().unwrap();
        for ba in bis {
            let g_instance::Instance::EBarrier(barrier) = ba else {
                continue;
            };

            if barrier.position_is_inside(vector3_add(
                self.position,
                Vector3::new(0., self.height / 2., 0.),
            )) {
                bid = Some(barrier.get_id())
            }
        }
        self.bid = bid;

        render::set_camera_pos(vector3_add(
            self.position,
            Vector3::new(0., self.chest_height, 0.),
        ))
        .unwrap();

        self.update_hud();
    }

    pub fn draw_model(&mut self) {
        // draw cylinder
        if cfg!(debug_assertions)
            && g_game::get_state().unwrap() != g_game::TopState::Menu
        {
            render::push_debug_cylinder_wires(
                self.position, 
                vector3_add(self.position, Vector3::new(0., self.height, 0.)), 
                self.radius, 
                self.radius, 
                11, 
                [1., 1., 0.]
            ).unwrap();
        }

        self.draw_hud();
    }

    pub fn draw_hud(&mut self) {
        // draw debug hud
        // if cfg!(debug_assertions) && g_game::get_state().unwrap() != g_game::TopState::Menu {
        if g_game::get_state().unwrap() != g_game::TopState::Menu {
            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!(
                    "velocity: {{{:>5.1},{:>5.1},{:>5.1}}}",
                    self.velocity.x, self.velocity.y, self.velocity.z
                ),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 255,
                        g: 167,
                        b: 167,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_sm().unwrap(),
            })
            .unwrap();
            v_text.dst_rect.set_y(32);
            text::push_surface(&v_text).unwrap();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("on_ground: {:?}", self.on_ground),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 167,
                        g: 255,
                        b: 167,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_sm().unwrap(),
            })
            .unwrap();
            v_text.dst_rect.set_y(16 * 3);
            text::push_surface(&v_text).unwrap();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("position: {{{:>5.1},{:>5.1},{:>5.1}}}", self.position.x, self.position.y, self.position.z),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 167,
                        g: 167,
                        b: 167,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_sm().unwrap(),
            })
            .unwrap();
            v_text.dst_rect.set_y(16 * 4);
            text::push_surface(&v_text).unwrap();

            // get grid pos
            let grid_pos = {
                let fgpos = vector3_scale(self.position, 1. / mcap::GRID_SIZE);
                (fgpos.x.floor() as u32, fgpos.y.floor() as u32, fgpos.z.floor() as u32 )
            };
            let collision_surfaces = g_game::get_surface_grid().unwrap().surfaces_in_cell(grid_pos).unwrap_or_default();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("grid[{:>2},{:>2},{:>2}]: {} ",
                    (self.position.x / mcap::GRID_SIZE) as u32,
                    (self.position.y / mcap::GRID_SIZE) as u32,
                    (self.position.z / mcap::GRID_SIZE) as u32,
                    collision_surfaces.len(),
                ),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 255,
                        g: 167,
                        b: 255,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_sm().unwrap(),
            })
            .unwrap();
            v_text.dst_rect.set_y(16 * 5);
            text::push_surface(&v_text).unwrap();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("barrier id: {:?}", self.bid),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_sm().unwrap(),
            })
            .unwrap();
            v_text.dst_rect.set_y(16 * 6);
            text::push_surface(&v_text).unwrap();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("fps: {}", time::get_fps().unwrap() ),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 255,
                        g: 255,
                        b: 167,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_sm().unwrap(),
            })
            .unwrap();
            v_text.dst_rect.set_y(16 * 7);
            text::push_surface(&v_text).unwrap();
        }

        match g_game::get_state().unwrap() {
            TopState::Play => {
                if self.opt_ass.is_some() {
                    text::push_surface(&self.opt_ass.as_ref().unwrap().encounter_bar).unwrap();
                    text::push_surface(&self.opt_ass.as_ref().unwrap().encounter_bar_frame)
                        .unwrap();
                }
            }
            _ => {}
        }
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }

    pub fn update_physics(&mut self) {

        let y_mat = matrix_rotate_y(self.yaw);

        let keys = input::get_keys().unwrap();

        let key_r = keys[input::Key::Right as usize] as i8;
        let key_l = keys[input::Key::Left as usize] as i8;
        let key_u = keys[input::Key::Up as usize] as i8;
        let key_d = keys[input::Key::Down as usize] as i8;

        let sprint = keys[input::Key::Sprint as usize];
        let jump = keys[input::Key::Jump as usize];

        let fd = time::get_delta_time().unwrap() as f32;

        self.acceleration = vector3_transform(
            Vector3 {
                x: (key_r - key_l) as f32,
                y: 0.,
                z: (key_u - key_d) as f32,
            },
            y_mat,
        );

        let move_dir = vector3_normalize(self.acceleration);

        let speed = 100.;
        let sprint_factor = match sprint {
            true => 1.5,
            false => 1.0,
        };
        let move_speed = match self.on_ground {
            true => 1. * sprint_factor * speed,
            false => 0.8 * sprint_factor * speed,
        };
        let friction = match self.on_ground {
            true => 10.,
            false => 9.,
        };
        let friction_factor = 1.0 - (friction * fd).min(1.0);
        self.velocity = vector3_multiply(self.velocity, Vector3::new(friction_factor, 1., friction_factor));
        self.velocity = vector3_add(self.velocity, vector3_scale(vector3_scale(move_dir, move_speed), fd));

        if jump && self.on_ground {
            self.velocity.y = 15.;
            self.on_ground = false;
        }

        let max_move_dist = self.radius / 10.;
        let desired_move = vector3_scale(self.velocity, fd);
        let desired_move_len = vector3_length(desired_move);

        let (move_count, move_dist) = if desired_move_len < max_move_dist {
            (1, desired_move)
        } else {
            let move_count = (vector3_length(desired_move) / max_move_dist) as u32 + 1;
            let move_dist = vector3_scale(desired_move, 1. / move_count as f32);
            (move_count, move_dist)
        };

        for _ in 0..move_count {

            let fd = fd / (move_count as f32);

            let pos = vector3_add(self.position, move_dist);
            let mut pos = mcap::Vec3::new(pos.x, pos.y, pos.z);

            let grid_pos = {
                let fgpos = pos / mcap::GRID_SIZE;
                (fgpos.x.floor() as u32, fgpos.y.floor() as u32, fgpos.z.floor() as u32 )
            };

            let collision_surfaces = g_game::get_surface_grid().unwrap().surfaces_in_cell(grid_pos).unwrap_or_default();

            // wall push
            (pos, _) = mcap::push_out_walls_2(pos, self.chest_height, self.radius, collision_surfaces);

            // snap down only when on ground
            let snap_down = match self.on_ground {
                true => self.snap_down,
                false => 0.
            };
            // radius is the range from player's center they start falling at
            // here, when center is half-radius off a ledge, starts falling
            match mcap::find_floor_height_hotdog_v4(pos, self.snap_up, snap_down, collision_surfaces, self.radius / 2.) {
                Some((Surface::Floor(floor), y)) => {
                    pos.y = y;
                    self.velocity.y = self.velocity.y.max(0.0);
                    self.on_ground = true;

                    let v1 = floor.verts[0] + floor.normal * 0.1;
                    let v2 = floor.verts[1] + floor.normal * 0.1;
                    let v3 = floor.verts[2] + floor.normal * 0.1;
                    let v1 = Vector3::new(v1.x, v1.y, v1.z);
                    let v2 = Vector3::new(v2.x, v2.y, v2.z);
                    let v3 = Vector3::new(v3.x, v3.y, v3.z);
                    render::push_debug_triangle(v1, v2, v3, 1., 0., 0.).unwrap();
                }
                Some((Surface::Slide(slide), y)) => {
                    pos.y = y;
                    let n = slide.normal();
                    let g = mcap::Vec3::new(0.0, GRAVITY, 0.0);
                    let g_slide = g - n * g.dot(n);

                    let mut velocity = mcap::Vec3::new(self.velocity.x, self.velocity.y, self.velocity.z);

                    // remove velocity into the slope
                    let vel_into_slope = n * velocity.dot(n);
                    velocity -= vel_into_slope;

                    // just feels better with 2x grav
                    velocity += g_slide * fd;

                    self.velocity = Vector3::new(velocity.x, velocity.y, velocity.z);

                    self.on_ground = false;

                    let v1 = slide.verts[0] + slide.normal * 0.1;
                    let v2 = slide.verts[1] + slide.normal * 0.1;
                    let v3 = slide.verts[2] + slide.normal * 0.1;
                    let v1 = Vector3::new(v1.x, v1.y, v1.z);
                    let v2 = Vector3::new(v2.x, v2.y, v2.z);
                    let v3 = Vector3::new(v3.x, v3.y, v3.z);
                    render::push_debug_triangle(v1, v2, v3, 0., 0., 1.).unwrap();
                }
                _ => {
                    // falling
                    self.velocity.y = (self.velocity.y + GRAVITY * fd).max(TERMINAL_VEL);
                    self.on_ground = false;
                }
            }

            // cieling clamp
            if let Some((Surface::Cieling(ciel), y)) = mcap::find_ciel_height_hotdog_v3(pos, self.chest_height, self.height - self.chest_height, collision_surfaces, self.radius / 2. ) {
                pos.y = y - self.height;
                self.velocity.y = self.velocity.y.min(0.0);

                let v1 = ciel.verts[0] + ciel.normal * 0.1;
                let v2 = ciel.verts[1] + ciel.normal * 0.1;
                let v3 = ciel.verts[2] + ciel.normal * 0.1;
                let v1 = Vector3::new(v1.x, v1.y, v1.z);
                let v2 = Vector3::new(v2.x, v2.y, v2.z);
                let v3 = Vector3::new(v3.x, v3.y, v3.z);
                render::push_debug_triangle(v1, v2, v3, 1., 1., 0.).unwrap();
            }

            self.position = Vector3::new(pos.x, pos.y, pos.z);

        }


    }
}
