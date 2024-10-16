use raymath::{
    matrix_identity, quaternion_to_matrix, vector3_add, vector3_transform, Matrix, Vector3,
};

use crate::{g_instance, math};

use crate::map::Entity;

use crate::text::{BannerInput, FontColor};
use crate::{g_game, render, text};

pub struct Barrier {
    base: Entity,
    id: Option<u32>,
    bounds: [Vector3; 8],
    normals: [Vector3; 6],
    mats: [Matrix; 8],
}

const BANNER_COLORS_V3: [[f32; 3]; 9] = [
    [0.0, 0.0, 1.0],
    [0.0, 0.5, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.5, 0.0],
    [1.0, 0.0, 0.0],
    [0.5, 0.0, 0.0],
    [1.0, 1.0, 1.0],
];

const BANNER_COLORS_RGB: [[u8; 3]; 9] = [
    [0x00, 0x00, 0xFF],
    [0x00, 0x80, 0xFF],
    [0x00, 0xFF, 0xFF],
    [0x00, 0xFF, 0x00],
    [0xFF, 0xFF, 0x00],
    [0xFF, 0x80, 0x00],
    [0xFF, 0x00, 0x00],
    [0x80, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
];

impl Barrier {
    pub fn new(entt: &Entity) -> Self {
        let mut id = None;

        for (i, v) in entt.params.iter().enumerate() {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "id" {
                let value = g_game::get_param(entt.params[i + 1] as usize).unwrap();
                id = Some(value.parse().unwrap());
                break;
            }
        }

        let mat_r = quaternion_to_matrix(entt.rotation.into());

        let fbl = vector3_transform(
            Vector3::new(-entt.scale[0] / 2., 0., -entt.scale[2] / 2.),
            mat_r,
        );
        let bbl = vector3_transform(
            Vector3::new(-entt.scale[0] / 2., 0., entt.scale[2] / 2.),
            mat_r,
        );
        let fbr = vector3_transform(
            Vector3::new(entt.scale[0] / 2., 0., -entt.scale[2] / 2.),
            mat_r,
        );
        let bbr = vector3_transform(
            Vector3::new(entt.scale[0] / 2., 0., entt.scale[2] / 2.),
            mat_r,
        );

        let ftl = vector3_add(
            vector3_transform(
                Vector3::new(-entt.scale[0] / 2., 0., -entt.scale[2] / 2.),
                mat_r,
            ),
            Vector3 {
                x: 0.,
                y: 2.,
                z: 0.,
            },
        );
        let btl = vector3_add(
            vector3_transform(
                Vector3::new(-entt.scale[0] / 2., 0., entt.scale[2] / 2.),
                mat_r,
            ),
            Vector3 {
                x: 0.,
                y: 2.,
                z: 0.,
            },
        );
        let ftr = vector3_add(
            vector3_transform(
                Vector3::new(entt.scale[0] / 2., 0., -entt.scale[2] / 2.),
                mat_r,
            ),
            Vector3 {
                x: 0.,
                y: 2.,
                z: 0.,
            },
        );
        let btr = vector3_add(
            vector3_transform(
                Vector3::new(entt.scale[0] / 2., 0., entt.scale[2] / 2.),
                mat_r,
            ),
            Vector3 {
                x: 0.,
                y: 2.,
                z: 0.,
            },
        );

        let mut bounds = [ftl, ftr, fbr, fbl, btl, btr, bbr, bbl];

        let mut mats = [matrix_identity(); 8];
        let mat_t = raymath::matrix_translate(entt.location[0], entt.location[1], entt.location[2]);
        for (i, point) in bounds.iter().enumerate() {
            let translate_mat = raymath::matrix_translate(point.x, point.y, point.z);
            let mat = raymath::matrix_multiply(mat_t, translate_mat);
            mats[i] = mat;
        }

        // apply mats to the points for bounds checking by player
        for b in &mut bounds {
            let translate_mat = raymath::matrix_translate(b.x, b.y, b.z);
            let mat = raymath::matrix_multiply(mat_t, translate_mat);
            *b = raymath::vector3_transform(*b, mat);
        }

        let normals = [
            math::vec3_face_normal(fbl, fbr, ftl), // Front
            math::vec3_face_normal(bbl, bbr, btl), // Back
            math::vec3_face_normal(fbl, bbl, ftl), // Left
            math::vec3_face_normal(fbr, bbr, ftr), // Right
            math::vec3_face_normal(ftl, ftr, btl), // Top
            math::vec3_face_normal(fbl, fbr, bbl), // Bottom
        ];

        Self {
            base: entt.clone(),
            id,
            bounds,
            normals,
            mats,
        }
    }

    pub fn update(&mut self) {
        // todo, push the barrier level surface to a queue with a distance,
        // to perform an ordered draw with the final camera params
        let pos_2d = math::world_point_to_screen_coord(
            self.base.location.into(),
            render::get_camera_pos().unwrap(),
            -render::get_camera_yaw().unwrap(),
            -render::get_camera_pitch().unwrap(),
            render::INTERNAL_W as f32,
            render::INTERNAL_H as f32,
        );

        match pos_2d {
            None => {}
            Some(pos) => {
                if !g_instance::pos_is_visible(
                    render::get_camera_pos().unwrap(),
                    self.base.location.into(),
                ) {
                    return;
                }

                let color = BANNER_COLORS_RGB[self.id.unwrap() as usize];

                let mut v_text = text::create_barrier_level_surface(BannerInput {
                    color: FontColor {
                        r: color[0],
                        g: color[1],
                        b: color[2],
                        a: 255,
                    },
                    level: self.id.unwrap() * 10,
                })
                .unwrap();

                let pos_x = pos.x as i32;
                let pos_y = pos.y as i32;
                let vtx_w = v_text.w as i32;
                let vtx_h = v_text.h as i32;
                let int_w = render::INTERNAL_W as i32;
                let int_h = render::INTERNAL_H as i32;

                if pos_x < (0 - vtx_w)
                    || pos_y < (0 - vtx_h)
                    || pos_x > (int_w + vtx_w)
                    || pos_y > (int_h + vtx_h)
                {
                    return;
                }

                v_text.x = (pos_x - vtx_w / 2) as u32;
                v_text.y = (pos_y - vtx_h / 2) as u32;
                text::push_surface(&v_text).unwrap();
            }
        }
    }

    pub fn draw_model(&mut self) {
        let ref_ent = g_game::get_ref_entity(self.base.ref_id).unwrap();

        let mat_s =
            raymath::matrix_scale(self.base.scale[0], self.base.scale[1], self.base.scale[2]);
        let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());
        let mat_t = raymath::matrix_translate(
            self.base.location[0],
            self.base.location[1],
            self.base.location[2],
        );

        let mut mat = raymath::matrix_identity();
        mat = raymath::matrix_multiply(mat, mat_s);
        mat = raymath::matrix_multiply(mat, mat_r);
        mat = raymath::matrix_multiply(mat, mat_t);

        let color = BANNER_COLORS_V3[self.id.unwrap() as usize];

        let cam_pos = render::get_camera_pos().unwrap();
        let cam_light_dist = math::vector3_distance(self.base.location.into(), cam_pos);

        let fade = math::scale(cam_light_dist, 32., 34., 1., 0.).clamp(0., 1.);
        let color = math::vector3_scale(color.into(), fade);

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[0] as i32,
            mix: 0.0,
            num_verts: ref_ent.num_verts,
            glow: Some(color),
        };
        render::draw(dc).unwrap();

        if cfg!(debug_assertions) {
            let re = g_instance::ref_ent_from_str("icosphere").unwrap();

            for mat in self.mats {
                let dc = render::DrawCall {
                    matrix: mat,
                    texture: re.texture_handle as u32,
                    f1: re.frame_handles[0] as i32,
                    f2: re.frame_handles[0] as i32,
                    mix: 0.0,
                    num_verts: re.num_verts,
                    glow: Some(color),
                };
                render::draw(dc).unwrap();
            }
        }
    }

    pub fn position_is_inside(&self, point: Vector3) -> bool {
        // Derive the min and max bounds from the points
        let min_x = self.bounds[0]
            .x
            .min(self.bounds[3].x)
            .min(self.bounds[4].x)
            .min(self.bounds[7].x);
        let max_x = self.bounds[1]
            .x
            .max(self.bounds[2].x)
            .max(self.bounds[5].x)
            .max(self.bounds[6].x);

        let min_y = self.bounds[2]
            .y
            .min(self.bounds[3].y)
            .min(self.bounds[6].y)
            .min(self.bounds[7].y);
        let max_y = self.bounds[0]
            .y
            .max(self.bounds[1].y)
            .max(self.bounds[4].y)
            .max(self.bounds[5].y);

        let min_z = self.bounds[0]
            .z
            .min(self.bounds[1].z)
            .min(self.bounds[2].z)
            .min(self.bounds[3].z);
        let max_z = self.bounds[4]
            .z
            .max(self.bounds[5].z)
            .max(self.bounds[6].z)
            .max(self.bounds[7].z);

        // Check if the point is within bounds
        point.x >= min_x
            && point.x <= max_x
            && point.y >= min_y
            && point.y <= max_y
            && point.z >= min_z
            && point.z <= max_z
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap()
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
