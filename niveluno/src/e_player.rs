use core::{f32, panic};
use std::collections::HashSet;

use raymath::{
    get_ray_collision_triangle, matrix_rotate_y, matrix_translate, vector3_add,
    vector3_cross_product, vector3_distance, vector3_dot_product, vector3_length, vector3_multiply,
    vector3_negate, vector3_normalize, vector3_project, vector3_scale, vector3_subtract,
    vector3_transform, BoundingBox, Ray, RayCollision,
};
use sdl2::libc::close;

use crate::g_game::TopState;
use crate::g_instance::get_decor_instances;
use crate::map::{self, Entity};
use crate::math::{
    closest_point_to_triangle, get_ray_collision_mesh, mesh_tranform, sat_aabb_tri,
    vec3_face_normal, Vector3,
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
    hud: Box<text::TextSurface>,
    speed: f32,
    acceleration: Vector3,
    velocity: Vector3,
    on_ground: bool,
    friction: f32,
    height: f32,
    width: f32,
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        // timed surface on spawn
        let mut spawn = text::create_surface(text::TextInput {
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
            on_ground: true,
            friction: 0.3,
            hud: match g_game::get_state().unwrap() {
                TopState::Menu => text::create_surface(text::TextInput {
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
                TopState::Play => text::create_surface(text::TextInput {
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
            height: 2.001, // the .001 is for climbing 1m ledges
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
            self.yaw,
            self.height,
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

        if cfg!(debug_assertions) {
            let mut v_text = text::create_surface(text::TextInput {
                text: format!("barrier id: {:?}", bid),
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
            v_text.y = 16 * 8;
            text::push_surface(&v_text).unwrap();
        }

        // TODO: Smooth step up on stairs
        // r_camera.y = e->p.y + 8 - clamp(game_time - e->_stepped_up_at, 0, 0.1) * -160;
        let camera_pos = vector3_add(self.position, Vector3::new(0., self.height, 0.));
        render::set_camera_pos(camera_pos).unwrap();

        self.update_hud();
    }
    pub fn draw_model(&mut self) {}
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
const MAX_SLOPE: f32 = 0.707;
const MAX_COLLISION_DIST: f32 = 32.;

pub fn update_physics(
    acceleration: &mut Vector3,
    velocity: &mut Vector3,
    position: &mut Vector3,
    on_ground: &mut bool,
    yaw: f32,
    height: f32,
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

    let steps = 16;

    let move_dist = vector3_scale(*velocity, delta_time);
    let mut step_size = vector3_scale(move_dist, 1. / steps as f32);

    let mut last_aabb = BoundingBox {
        min: Vector3::new(0., 0., 0.),
        max: Vector3::new(0., 0., 0.),
    };
    let mut last_floor = Vector3::new(0., 0., 0.);

    let mut decs = get_decor_instances().unwrap();

    for _ in 0..steps {
        // move one step
        let mut tmp_pos = vector3_add(*position, step_size);

        // FLOORS

        // cast ray down from top of head, should be fine so long as we're never moving
        // more than +2m/frame downward
        let ray = raymath::Ray {
            position: Vector3::new(tmp_pos.x, tmp_pos.y + height, tmp_pos.z),
            direction: Vector3::new(0., -1., 0.),
        };

        // find nearest floor collision
        // todo -- perf, only loop over floors (normal.y >= .707)
        // pre calculate the normal, store somewhere else
        // we're checking every triangle twice
        let mut floor_hit: Option<RayCollision> = None;
        for dec in &mut decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();

            let coll = get_ray_collision_mesh(ray, mesh, mat, Some((tmp_pos, MAX_COLLISION_DIST)));
            // collides, and is closest, and is less than 45 degree slope
            if coll.hit && coll.distance < height && -coll.normal.y >= MAX_SLOPE {
                // If nearest collision is not set or this one is closer, update nearest_hit
                if floor_hit.is_none() || floor_hit.unwrap().distance > coll.distance {
                    floor_hit = Some(coll);
                }
            }
        }

        // If we hit the floor, step back and stop vertical movement
        if floor_hit.is_some() {
            velocity.y = 0.; // Stop vertical velocity
            let floor_collision = floor_hit.unwrap();
            let floor_normal = floor_collision.normal;

            // Project step_size onto the floor's plane
            // let dot = vector3_dot_product(step_size, floor_normal);
            // let correction = vector3_scale(floor_normal, dot);

            // Remove the component of the step_size in the direction of the floor normal
            // step_size = vector3_subtract(step_size, correction);
            step_size.y = 0.;

            // Ensure the player stays on or above the floor to prevent sinking
            let floor_y = floor_collision.point.y;
            if tmp_pos.y < floor_y {
                tmp_pos.y = floor_y; // Adjust player position to stay on the ground
            }

            // Mark the player as on the ground
            *on_ground = true;
            last_floor = floor_normal;
        }

        // WALLS
        // todo -- perf dissolve map faces to reduce triangle count
        // todo -- height here should be divided by max slope out of 90 degrees. 45/90 == 1/2.
        let mut aabb = BoundingBox {
            min: Vector3::new(tmp_pos.x, tmp_pos.y, tmp_pos.z),
            max: Vector3::new(tmp_pos.x, tmp_pos.y + height, tmp_pos.z),
        };

        match last_floor {
            Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            } => {
                // todo -- this will permit 1/2 height climb-ups -- do camera smoothing
                aabb.min.y += height / 2.;
            }
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            } => {}
            _ => {
                // lets see
                aabb.min.y += height / 2. + ((1. + last_floor.y) / (1. - MAX_SLOPE)) / 2.;
            }
        };

        last_aabb = aabb;

        let p_y_mat = matrix_rotate_y(yaw);
        let p_dir = vector3_transform(Vector3::new(1., 0., 0.), p_y_mat);

        let mut wall_collisions = vec![];

        for dec in &mut decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();
            let mesh = mesh_tranform(mesh, mat);

            for (tri, normal) in mesh.into_iter().map(|tri| {
                (
                    tri,
                    vector3_negate(vec3_face_normal(tri[0], tri[1], tri[2])),
                )
            }) {
                if !(-normal.y < MAX_SLOPE) {
                    continue;
                }

                if (vector3_distance(tmp_pos, tri[0]) > MAX_COLLISION_DIST)
                    && (vector3_distance(tmp_pos, tri[1]) > MAX_COLLISION_DIST)
                    && (vector3_distance(tmp_pos, tri[2]) > MAX_COLLISION_DIST)
                {
                    continue;
                }

                let ppos = Vector3::new(tmp_pos.x, (aabb.max.y + aabb.min.y) / 2., tmp_pos.z);
                let closest = closest_point_to_triangle(tri, ppos);

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

                    let mat = matrix_translate(ppos.x + width / 2., ppos.y, ppos.z);
                    let dc = render::DrawCall {
                        matrix: mat,
                        texture: re.texture_handle as u32,
                        f1: re.frame_handles[0] as i32,
                        f2: re.frame_handles[0] as i32,
                        mix: 0.0,
                        num_verts: re.num_verts,
                        glow: Some(Vector3::new(0., 0., 0.7)),
                    };
                    render::draw(dc).unwrap();
                }
                // collides height-wise
                if aabb.min.y <= closest.y &&
                    closest.y <= aabb.max.y &&
                    // collides width-wise
                    vector3_distance(Vector3::new(tmp_pos.x, 0., tmp_pos.z), Vector3::new(closest.x, 0., closest.z)) <= width / 2.
                {
                    // store the triangle we're colliding with,
                    // and store the y height we're going to collide at
                    // wall_collisions.push((tri, closest.y));
                    wall_collisions.push((closest, vector3_negate(normal)));
                }
            }
        }

        let coll_count = wall_collisions.len();

        // todo -- perf
        // 1, step back perfectly, using the closest collision
        // 2, exit early out of the loop above on first collision, and don't collect any walls
        // return to prior step's position
        if coll_count == 0 {
            *position = tmp_pos;
            continue;
        }

        let hb_center = vector3_add(
            tmp_pos,
            Vector3::new(0., (aabb.min.y + aabb.max.y) / 2., 0.),
        );
        if coll_count > 0 {
            // eprintln!("coll_count: {coll_count}");
            // eprintln!("colls[{coll_count}]: {:?}", wall_collisions);
        }

        // find closest intersection point to the center of the player's hitbox
        let mut intersector = None;
        let mut closest_distance = f32::INFINITY;
        let mut closest_angle = f32::INFINITY;
        for (coll, norm) in wall_collisions {
            // eprintln!("coll: {:?} -- norm: {:?}", coll, norm);
            let dist = vector3_distance(hb_center, coll);
            let angl = vector3_dot_product(p_dir, norm).abs();
            // eprintln!("angl: {:?}", angl);
            // if closer, or equally close but better alignment
            if dist < closest_distance || (dist == closest_distance && angl > closest_angle) {
                let merged = match intersector {
                    None => norm,
                    Some(old) => vector3_normalize(vector3_add(norm, old)),
                };
                // let merged = vector3_normalize(vector3_add(norm, intersector.unwrap()));
                let angl = vector3_dot_product(p_dir, merged).abs();
                intersector = Some(norm);
                closest_distance = dist;
                closest_angle = angl;
            }
        }

        // eprintln!("intersector: {:?}", intersector);
        match intersector {
            None => {
                if coll_count != 0 {
                    panic!("wtf");
                }
            }
            Some(norm) => {
                // let dot = vector3_dot_product(step_size, norm);
                // eprintln!("dot: {:?}", dot);
                // if dot < 0.0 {
                //     let correction = vector3_scale(norm, dot);
                //     step_size = vector3_subtract(step_size, correction);
                //     tmp_pos = vector3_add(*position, step_size);
                // }

                // Use `step_size` to determine correction, but introduce a threshold for minor adjustments
                let dot = vector3_dot_product(step_size, norm);
                let correction_threshold = 0.0001; // Small threshold to prevent overcorrection on minor contact

                // Apply correction when movement is pushing against the wall (negative dot product)
                // Only adjust if `dot` is below the threshold
                if dot < -correction_threshold {
                    let correction = vector3_scale(norm, dot);
                    step_size = vector3_subtract(step_size, correction);
                }

                // Apply a consistent small offset to prevent sticking along the wall
                let min_movement_offset = 0.001;
                step_size = vector3_add(step_size, vector3_scale(norm, min_movement_offset));

                // Update temporary position based on the adjusted step size
                tmp_pos = vector3_add(*position, step_size);
            }
        }

        // let hb_center = vector3_add(tmp_pos, Vector3::new(0., (aabb.min.y + aabb.max.y) / 2., 0.));
        // let mut intersector = None;
        // for (coll, norm) in wall_collisions {
        //     // Calculate the closest point on the movement trajectory to the collision point
        //     let inter = closest_point_on_line_segment(tmp_pos, vector3_add(tmp_pos, step_size), coll);

        //     match intersector {
        //         None => intersector = Some((inter, norm)),
        //         Some((old, _)) => {
        //             // Prioritize the collision point closest to the player's center
        //             if vector3_distance(inter, hb_center) < vector3_distance(old, hb_center) {
        //                 intersector = Some((inter, norm))
        //             }
        //         }
        //     }
        // }

        // match intersector {
        //     None => {
        //         if coll_count > 0 {
        //             panic!("shouldn't be possible");
        //         }
        //     },
        //     Some((_, wall_normal)) => {
        //         let wall_normal = vector3_negate(wall_normal);
        //         // let dot = vector3_dot_product(step_size, norm);
        //         // if dot < 0. {
        //         //     let correction = vector3_scale(norm, dot);
        //         //     step_size = vector3_subtract(step_size, correction);
        //         //     tmp_pos = vector3_add(*position, step_size);
        //         // }
        //     // Project velocity onto each wall normal
        //     let dot = vector3_dot_product(step_size, wall_normal);
        //     if dot < 0. {
        //         // Cancel out movement along the wall normal to prevent pushing into walls
        //         let correction = vector3_scale(wall_normal, dot);
        //         step_size = vector3_subtract(step_size, correction);
        //     }

        //     // Enforce minimum movement in the direction away from the wall
        //     // step_size = vector3_add(step_size, vector3_scale(wall_normal, 0.0005));
        //     tmp_pos = vector3_add(
        //         *position,
        //         vector3_add(step_size, vector3_scale(wall_normal, 0.005)),
        //     );
        //     }
        // }

        // // find the triangle that the player's body is going to hit first
        // let mut one_wall_to_rule_them_all = None;
        // for (tri, height) in wall_collisions {
        //     let ray = Ray {
        //         position: Vector3 {x: tmp_pos.x, y: height, z: tmp_pos.z},
        //         direction: vector3_normalize(Vector3 { x: step_size.x, y: 0., z: step_size.z })
        //     };

        //     let coll = get_ray_collision_triangle(ray, tri[0], tri[1], tri[2]);
        //     if coll.hit {
        //         match one_wall_to_rule_them_all {
        //             None => one_wall_to_rule_them_all = Some(coll),
        //             Some(old) => {
        //                 if coll.distance < old.distance {
        //                     one_wall_to_rule_them_all = Some(coll)
        //                 }
        //             }
        //         }
        //     }
        // }

        // match one_wall_to_rule_them_all {
        //     None => {
        //         if coll_count > 0 {
        //             panic!("there were collisions, but couldn't find closest???");
        //         }
        //     },
        //     Some(coll) => {
        //         eprintln!("still ok");
        //         let dot = vector3_dot_product(step_size, vector3_negate(coll.normal));
        //         if dot < 0. {
        //             let correction = vector3_scale(vector3_negate(coll.normal), dot);
        //             step_size = vector3_subtract(step_size, correction);
        //         }
        //         tmp_pos = vector3_add(*position, step_size);
        //     }
        // }

        *position = tmp_pos;
    }

    // draw cylinder
    if cfg!(debug_assertions) {
        let re = g_instance::ref_ent_from_str("icosphere").unwrap();

        for i in 0..10 {
            let mat_r = matrix_rotate_y((2. * f32::consts::PI) * i as f32 / 10.);
            let bot = vector3_add(
                last_aabb.min,
                vector3_transform(Vector3::new(width / 2., 0., 0.), mat_r),
            );
            let top = vector3_add(
                last_aabb.max,
                vector3_transform(Vector3::new(width / 2., 0., 0.), mat_r),
            );

            let bot_mat = matrix_translate(bot.x, bot.y, bot.z);
            let top_mat = matrix_translate(top.x, top.y, top.z);

            let dc = render::DrawCall {
                matrix: bot_mat,
                texture: re.texture_handle as u32,
                f1: re.frame_handles[0] as i32,
                f2: re.frame_handles[0] as i32,
                mix: 0.0,
                num_verts: re.num_verts,
                glow: Some(Vector3::new(0., 0.7, 0.)),
            };
            render::draw(dc).unwrap();

            let dc = render::DrawCall {
                matrix: top_mat,
                texture: re.texture_handle as u32,
                f1: re.frame_handles[0] as i32,
                f2: re.frame_handles[0] as i32,
                mix: 0.0,
                num_verts: re.num_verts,
                glow: Some(Vector3::new(0., 0.7, 0.)),
            };
            render::draw(dc).unwrap();
        }
    }

    if cfg!(debug_assertions) {
        let mut v_text = text::create_surface(text::TextInput {
            text: format!(
                "velocity: {{{:>5.1},{:>5.1},{:>5.1}  }}",
                velocity.x, velocity.y, velocity.z
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

        let mut v_text = text::create_surface(text::TextInput {
            text: format!("on_ground: {:?}", *on_ground),
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

        let mut v_text = text::create_surface(text::TextInput {
            text: format!("position: {:?}", *position),
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

        let mut v_text = text::create_surface(text::TextInput {
            text: format!(
                "aabb.min: {{{:>5.1},{:>5.1},{:>5.1}  }}",
                last_aabb.min.x, last_aabb.min.y, last_aabb.min.z
            ),
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
        v_text.y = 16 * 5;
        text::push_surface(&v_text).unwrap();

        let mut v_text = text::create_surface(text::TextInput {
            text: format!(
                "aabb.max: {{{:>5.1},{:>5.1},{:>5.1}  }}",
                last_aabb.max.x, last_aabb.max.y, last_aabb.max.z
            ),
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
        v_text.y = 16 * 6;
        text::push_surface(&v_text).unwrap();

        let mut v_text = text::create_surface(text::TextInput {
            text: format!("last_floor: {:?}", last_floor),
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
        v_text.y = 16 * 7;
        text::push_surface(&v_text).unwrap();
    }
}
