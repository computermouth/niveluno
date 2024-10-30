use core::{f32, panic};

use raymath::{
    matrix_rotate_y, matrix_translate, vector2_add, vector2_distance, vector2_dot_product,
    vector2_length, vector2_normalize, vector2_scale, vector2_subtract, vector3_add,
    vector3_distance, vector3_dot_product, vector3_length, vector3_multiply, vector3_negate,
    vector3_normalize, vector3_scale, vector3_subtract, vector3_transform, BoundingBox,
    RayCollision, Vector2,
};

use crate::g_game::TopState;
use crate::g_instance::{get_decor_instances, Instance};
use crate::map::{self, Entity};
use crate::math::{
    closest_point_to_triangle, get_ray_collision_mesh, mesh_tranform, vec3_face_normal, Vector3,
};
use crate::text;
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
    width: f32,
    // opt_ass: Option<OptAssets>,
}

pub struct OptAssets {
    encounter_bar: u32,
}

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

        spawn.x = 100;
        spawn.y = 100;

        let ts = text::TimedSurface::new(spawn, 1000);

        text::push_timed_surface(ts).unwrap();

        let height = 2.001; // the .001 is for climbing 1m ledges

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
            height,
            width: 3.,
        }
    }

    pub fn update_hud(&self) {
        text::push_surface(&self.hud).unwrap();
    }

    pub fn update(&mut self) {
        let _ = self.base;

        let keys = input::get_keys().unwrap();

        if keys[input::Key::Jump as usize] == true && g_game::get_state().unwrap() == TopState::Menu
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

        let y_mat = matrix_rotate_y(self.yaw);

        let key_r = keys[input::Key::Right as usize] as i8;
        let key_l = keys[input::Key::Left as usize] as i8;
        let key_u = keys[input::Key::Up as usize] as i8;
        let key_d = keys[input::Key::Down as usize] as i8;

        self.acceleration = vector3_transform(
            Vector3 {
                x: (key_r - key_l) as f32,
                y: 0.,
                z: (key_u - key_d) as f32,
            },
            y_mat,
        );

        let key_jump = keys[input::Key::Jump as usize] as i8;
        if key_jump == 1 && self.on_ground {
            self.velocity.y = 15.;
            self.on_ground = false;
        }

        let speed_factor = match self.on_ground {
            true => 1.0,
            false => 0.9,
        };
        self.acceleration = vector3_scale(self.acceleration, self.speed * speed_factor);

        self.friction = match self.on_ground {
            true => 10.,
            false => 2.5,
        };

        update_physics(
            &mut self.acceleration,
            &mut self.velocity,
            &mut self.position,
            &mut self.on_ground,
            &mut self.last_floor,
            self.width,
        );

        let mut bid = None;
        let bis = g_instance::get_barrier_instances().unwrap();
        for ba in bis {
            let barrier = match ba {
                g_instance::Instance::EBarrier(b) => b,
                _ => panic!(), // this sucks ass
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
            Vector3::new(0., self.height, 0.),
        ))
        .unwrap();

        self.update_hud();
    }

    pub fn draw_model(&mut self) {
        // draw cylinder
        let re = g_instance::ref_ent_from_str("icosphere");
        if cfg!(debug_assertions)
            && g_game::get_state().unwrap() != g_game::TopState::Menu
            && re.is_some()
        {
            let re = re.unwrap();

            for i in 0..10 {
                let mat_y = matrix_rotate_y((2. * f32::consts::PI) * i as f32 / 10.);
                let horizontal = vector3_add(
                    vector3_add(self.position, Vector3::new(0., self.width / 2., 0.)),
                    vector3_transform(Vector3::new(self.width / 2., 0., 0.), mat_y),
                );

                let h_mat = matrix_translate(horizontal.x, horizontal.y, horizontal.z);

                let dc = render::DrawCall {
                    matrix: h_mat,
                    texture: re.texture_handle as u32,
                    f1: re.frame_handles[0] as i32,
                    f2: re.frame_handles[0] as i32,
                    mix: 0.0,
                    num_verts: re.num_verts,
                    glow: Some(Vector3::new(0., 0.7, 0.)),
                };
                render::draw(dc).unwrap();
            }

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!(
                    "velocity: {{{:>5.1},{:>5.1},{:>5.1}  }}",
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
            v_text.y = 32;
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
            v_text.y = 16 * 3;
            text::push_surface(&v_text).unwrap();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("position: {:?}", self.position),
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
            v_text.y = 16 * 4;
            text::push_surface(&v_text).unwrap();

            let mut v_text = text::create_text_overlay_surface(text::TextInput {
                text: format!("last_floor: {:?}", self.last_floor),
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
            v_text.y = 16 * 5;
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
            v_text.y = 16 * 6;
            text::push_surface(&v_text).unwrap();
        }
    }
    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}

const GRAVITY: f32 = 1.0;
const FRICTION: f32 = 10.0;
// ~45.01 degrees -- if you change this, you'll
// have to change a few of the height / 2. below
pub const MAX_SLOPE: f32 = 0.707;
// todo, add an nmcc check to output the minimum largest value here
// before we start falling through triangles. it'd be half the largest
// distance between two points of a decor triangle I think
pub const MAX_COLLISION_DIST: f32 = 8.;

pub fn update_physics(
    acceleration: &mut Vector3,
    velocity: &mut Vector3,
    position: &mut Vector3,
    on_ground: &mut bool,
    last_floor: &mut Vector3,
    width: f32,
) {
    // Apply Gravity
    acceleration.y = -36. * GRAVITY;

    let delta_time = time::get_delta_time().unwrap() as f32;

    // Integrate acceleration & friction into velocity
    let df = 1.0f32.min(FRICTION * delta_time);
    let af = vector3_scale(*acceleration, delta_time);
    let vf = vector3_multiply(
        *velocity,
        Vector3 {
            x: df,
            y: 0.,
            z: df,
        },
    );
    *velocity = vector3_add(*velocity, vector3_subtract(af, vf));
    // todo -- perf
    // we can probably return early here, if v3len(velocity) == 0.0 (or within epsilon)
    *on_ground = false;

    let steps = 8;

    let move_dist = vector3_scale(*velocity, delta_time);
    let mut step = vector3_scale(move_dist, 1. / steps as f32);

    for _ in 0..steps {
        let pos = vector3_add(*position, step);
        *position = collide_and_slide_3d(pos, &mut step, last_floor, velocity, on_ground, width, 0);
    }
}

const VCLOSE: f32 = 0.00001;

fn collide_and_slide_3d(
    pos: Vector3,
    step: &mut Vector3,
    last_floor: &mut Vector3,
    velocity: &mut Vector3,
    on_ground: &mut bool,
    width: f32,
    iter: usize,
) -> Vector3 {
    if iter >= 5 {
        return pos;
    }

    let player_radius = width / 2.;
    let player_center = vector3_add(pos, Vector3::new(0., width / 2., 0.));

    // WALLS
    // todo -- perf dissolve map faces to reduce triangle count
    // todo -- height here should be divided by max slope out of 90 degrees. 45/90 == 1/2.
    if let Some((nearest, norm)) = process_intersector_3d(player_center, width) {
        let collision_normal = vector3_normalize(vector3_subtract(player_center, nearest));
        let closest_distance = vector3_distance(player_center, nearest);

        // Move the circle to just touch the wall
        let penetration_depth = player_radius - closest_distance + VCLOSE * 2.;
        let new_pos = vector3_add(pos, vector3_scale(collision_normal, penetration_depth));

        // Determine if this is a floor or a wall
        if norm.y > MAX_SLOPE {
            // FLOOR LOGIC
            *on_ground = true;
            *last_floor = nearest;
            velocity.y = 0.;
            step.y = 0.;
        }

        // Calculate new velocity to slide along the wall
        let mut new_velocity = vector3_subtract(
            *velocity,
            vector3_scale(
                collision_normal,
                vector3_dot_product(*velocity, collision_normal),
            ),
        );

        if vector3_length(new_velocity) < VCLOSE {
            new_velocity = Vector3::zero();
        }

        // Adjust `local_step` to account for the distance already traveled
        let remaining_distance = vector3_length(*step) - closest_distance;
        let mut new_step = vector3_scale(vector3_normalize(new_velocity), remaining_distance);

        if vector3_length(new_step) < VCLOSE {
            new_step = Vector3::zero();
        }

        // Recursive call with updated values
        return collide_and_slide_3d(
            new_pos,
            &mut new_step,
            last_floor,
            velocity,
            on_ground,
            width,
            iter + 1,
        );
    }
    pos
}

fn process_intersector_3d(center_pos: Vector3, width: f32) -> Option<(Vector3, Vector3)> {
    let mut wall_collisions = vec![];
    let decs = get_decor_instances().unwrap();

    for dec in decs {
        let mesh = dec.get_mesh();
        let mat = dec.get_matrix();
        let mesh = mesh_tranform(mesh, mat);

        for (tri, normal) in mesh.into_iter().map(|tri| {
            (
                tri,
                vector3_negate(vec3_face_normal(tri[0], tri[1], tri[2])),
            )
        }) {
            if (vector3_distance(center_pos, tri[0]) > MAX_COLLISION_DIST)
                && (vector3_distance(center_pos, tri[1]) > MAX_COLLISION_DIST)
                && (vector3_distance(center_pos, tri[2]) > MAX_COLLISION_DIST)
            {
                continue;
            }

            let closest = closest_point_to_triangle(tri, center_pos);

            if cfg!(debug_assertions) {
                let re = g_instance::ref_ent_from_str("icosphere").unwrap();
                let mat = matrix_translate(closest.x, closest.y, closest.z);

                let dc = render::DrawCall {
                    matrix: mat,
                    texture: re.texture_handle as u32,
                    f1: re.frame_handles[0] as i32,
                    f2: re.frame_handles[0] as i32,
                    mix: 0.0,
                    num_verts: re.num_verts,
                    glow: Some(Vector3::new(0.7, 0.7, 0.7)),
                };
                render::draw(dc).unwrap();
            }
            // collides inside sphere
            if vector3_distance(center_pos, closest) <= width / 2. + VCLOSE {
                wall_collisions.push((closest, vector3_negate(normal)));
            }
        }
    }

    if wall_collisions.is_empty() {
        return None;
    }

    // find closest intersection point to the center of the player's hitbox
    let mut intersector = None;
    let mut closest_distance = f32::INFINITY;
    for (coll, norm) in wall_collisions {
        let dist = vector3_distance(center_pos, coll);
        if dist < closest_distance {
            intersector = Some((coll, norm));
            closest_distance = dist;
        }
    }

    intersector
}
