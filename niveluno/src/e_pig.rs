use crate::e_entity::EntityInstance;
use crate::map::Entity;
use crate::time;

use crate::g_game;
use crate::render;

pub struct Pig {
    base: Entity,
    yaw: f32,
    pitch: f32,
    scale_mat: raymath::Matrix,
    quat: raymath::Quaternion,
    animations: Vec<Vec<usize>>,
    anim_id: PigAnimations,
    anim_time: f32,
    anim_length: Vec<f32>,
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum PigAnimations {
    Tpose,
    Swipe,
    Charge,
    Land,
    Die,
    Bump,
    __End,
}

const PIG_FRAME_DEFAULT: &'static str = "default";

const PIG_FRAME_SWIPE_000: &'static str = "swipe.000";
const PIG_FRAME_SWIPE_001: &'static str = "swipe.001";
const PIG_FRAME_SWIPE_002: &'static str = "swipe.002";
const PIG_FRAME_SWIPE_003: &'static str = "swipe.003";
const PIG_FRAME_SWIPE_004: &'static str = "swipe.004";
const PIG_FRAME_SWIPE_005: &'static str = "swipe.005";
const PIG_FRAME_SWIPE_006: &'static str = "swipe.006";
const PIG_FRAME_SWIPE_007: &'static str = "swipe.007";
const PIG_FRAME_SWIPE_008: &'static str = "swipe.008";
const PIG_FRAME_SWIPE_009: &'static str = "swipe.009";
const PIG_FRAME_SWIPE_010: &'static str = "swipe.010";
const PIG_FRAME_SWIPE_011: &'static str = "swipe.011";
const PIG_FRAME_SWIPE_012: &'static str = "swipe.012";
const PIG_FRAME_SWIPE_013: &'static str = "swipe.013";
const PIG_FRAME_SWIPE_014: &'static str = "swipe.014";
const PIG_FRAME_SWIPE_015: &'static str = "swipe.015";
const PIG_FRAME_SWIPE_016: &'static str = "swipe.016";
const PIG_FRAME_SWIPE_017: &'static str = "swipe.017";
const PIG_FRAME_SWIPE_018: &'static str = "swipe.018";
const PIG_FRAME_SWIPE_019: &'static str = "swipe.019";

const PIG_FRAME_CHARGE_000: &'static str = "charge.000";
const PIG_FRAME_CHARGE_001: &'static str = "charge.001";
const PIG_FRAME_CHARGE_002: &'static str = "charge.002";
const PIG_FRAME_CHARGE_003: &'static str = "charge.003";
const PIG_FRAME_CHARGE_004: &'static str = "charge.004";

const PIG_FRAME_LAND_000: &'static str = "land.000";
const PIG_FRAME_LAND_001: &'static str = "land.001";
const PIG_FRAME_LAND_002: &'static str = "land.002";
const PIG_FRAME_LAND_003: &'static str = "land.003";
const PIG_FRAME_LAND_004: &'static str = "land.004";

const PIG_FRAME_DIE_000: &'static str = "die.000";
const PIG_FRAME_DIE_001: &'static str = "die.001";
const PIG_FRAME_DIE_002: &'static str = "die.002";
const PIG_FRAME_DIE_003: &'static str = "die.003";
const PIG_FRAME_DIE_004: &'static str = "die.004";
const PIG_FRAME_DIE_005: &'static str = "die.005";
const PIG_FRAME_DIE_006: &'static str = "die.006";
const PIG_FRAME_DIE_007: &'static str = "die.007";
const PIG_FRAME_DIE_008: &'static str = "die.008";
const PIG_FRAME_DIE_009: &'static str = "die.009";
const PIG_FRAME_DIE_010: &'static str = "die.010";
const PIG_FRAME_DIE_011: &'static str = "die.011";
const PIG_FRAME_DIE_012: &'static str = "die.012";
const PIG_FRAME_DIE_013: &'static str = "die.013";

const PIG_FRAME_BUMP_000: &'static str = "bump.000";
const PIG_FRAME_BUMP_001: &'static str = "bump.001";
const PIG_FRAME_BUMP_002: &'static str = "bump.002";
const PIG_FRAME_BUMP_003: &'static str = "bump.003";
const PIG_FRAME_BUMP_004: &'static str = "bump.004";
const PIG_FRAME_BUMP_005: &'static str = "bump.005";
const PIG_FRAME_BUMP_006: &'static str = "bump.006";
const PIG_FRAME_BUMP_007: &'static str = "bump.007";
const PIG_FRAME_BUMP_008: &'static str = "bump.008";

const PIG_ANIMATIONS: &[&[&str]] = &[
    &[PIG_FRAME_DEFAULT],
    &[
        PIG_FRAME_SWIPE_000,
        PIG_FRAME_SWIPE_001,
        PIG_FRAME_SWIPE_002,
        PIG_FRAME_SWIPE_003,
        PIG_FRAME_SWIPE_004,
        PIG_FRAME_SWIPE_005,
        PIG_FRAME_SWIPE_006,
        PIG_FRAME_SWIPE_007,
        PIG_FRAME_SWIPE_008,
        PIG_FRAME_SWIPE_009,
        PIG_FRAME_SWIPE_010,
        PIG_FRAME_SWIPE_011,
        PIG_FRAME_SWIPE_012,
        PIG_FRAME_SWIPE_013,
        PIG_FRAME_SWIPE_014,
        PIG_FRAME_SWIPE_015,
        PIG_FRAME_SWIPE_016,
        PIG_FRAME_SWIPE_017,
        PIG_FRAME_SWIPE_018,
        PIG_FRAME_SWIPE_019,
    ],
    &[
        PIG_FRAME_CHARGE_000,
        PIG_FRAME_CHARGE_001,
        PIG_FRAME_CHARGE_002,
        PIG_FRAME_CHARGE_003,
        PIG_FRAME_CHARGE_004,
    ],
    &[
        PIG_FRAME_LAND_000,
        PIG_FRAME_LAND_001,
        PIG_FRAME_LAND_002,
        PIG_FRAME_LAND_003,
        PIG_FRAME_LAND_004,
    ],
    &[
        PIG_FRAME_DIE_000,
        PIG_FRAME_DIE_001,
        PIG_FRAME_DIE_002,
        PIG_FRAME_DIE_003,
        PIG_FRAME_DIE_004,
        PIG_FRAME_DIE_005,
        PIG_FRAME_DIE_006,
        PIG_FRAME_DIE_007,
        PIG_FRAME_DIE_008,
        PIG_FRAME_DIE_009,
        PIG_FRAME_DIE_010,
        PIG_FRAME_DIE_011,
        PIG_FRAME_DIE_012,
        PIG_FRAME_DIE_013,
    ],
    &[
        PIG_FRAME_BUMP_000,
        PIG_FRAME_BUMP_001,
        PIG_FRAME_BUMP_002,
        PIG_FRAME_BUMP_003,
        PIG_FRAME_BUMP_004,
        PIG_FRAME_BUMP_005,
        PIG_FRAME_BUMP_006,
        PIG_FRAME_BUMP_007,
        PIG_FRAME_BUMP_008,
    ],
];

impl EntityInstance for Pig {
    fn update(&mut self) {
        // let dt = time::get_delta_time().unwrap() as f32;

        // self.yaw += 1. * dt;
        // self.pitch -= 3. * dt;

        // let quat_y = raymath::quaternion_from_axis_angle(Vector3::new(1., 0., 0.), self.yaw);
        // let quat_p = raymath::quaternion_from_axis_angle(Vector3::new(0., 1., 0.), self.pitch);

        self.quat = self.base.rotation.into();
        // self.quat = raymath::quaternion_multiply(self.quat, quat_p);
        // self.quat = raymath::quaternion_multiply(self.quat, quat_y);
    }

    fn draw_model(&mut self) {
        let ref_ent = g_game::get_ref_entity(self.base.index).unwrap();

        // scale, rotation, translation
        let mat_r = raymath::quaternion_to_matrix(self.quat);
        let mat_t = raymath::matrix_translate(
            self.base.location[0],
            self.base.location[1],
            self.base.location[2],
        );

        let mut mat = raymath::matrix_identity();
        mat = raymath::matrix_multiply(mat, self.scale_mat);
        mat = raymath::matrix_multiply(mat, mat_r);
        mat = raymath::matrix_multiply(mat, mat_t);

        // animation, frame, mix
        self.anim_time += time::get_delta_time().unwrap() as f32;

        let mut f = self.anim_time / self.anim_length[self.anim_id as usize];
        let mix = f - f.floor();

        let anim = &self.animations[self.anim_id as usize];

        let mut frame_curr = anim[(f as usize) % anim.len()];
        let mut frame_next = anim[((f as usize) + 1) % anim.len()];
        // if frame_next < frame_curr {
        //     let tmp = frame_curr;
        //     frame_curr = frame_next;
        //     frame_next = tmp;
        //     mix = 1. - mix;
        // }
        if f > 1.0 {
            let mut next_id = (self.anim_id as usize) + 1;
            if next_id >= PigAnimations::__End as usize {
                next_id = 0;
            }
            self.anim_id = unsafe { std::mem::transmute(next_id) };
            f = 0.;

            frame_curr = anim[(f as usize) % anim.len()];
            frame_next = anim[((f as usize) + 1) % anim.len()];

            self.anim_time = 0.;
        }

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[frame_curr] as i32,
            f2: ref_ent.frame_handles[frame_next] as i32,
            mix,
            num_verts: ref_ent.num_verts,
            unlit: false,
        };
        render::draw(dc).unwrap();
    }
}

impl Pig {
    pub fn new(entt: &Entity) -> Self {
        let ref_ent = g_game::get_ref_entity(entt.index).unwrap();
        let animations = g_game::get_animation_ids(PIG_ANIMATIONS, &ref_ent);

        eprintln!("re.names: {:?}", ref_ent.frame_names);
        eprintln!("re.animations: {:?}", animations);

        let anim_length = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

        assert_eq!(animations.len(), anim_length.len());

        Self {
            base: entt.clone(),
            yaw: 0.,
            pitch: 0.,
            scale_mat: raymath::matrix_scale(entt.scale[0], entt.scale[1], entt.scale[2]),
            quat: raymath::quaternion_identity(),
            animations,
            anim_id: PigAnimations::Tpose,
            anim_time: 0.,
            anim_length,
        }
    }
}
