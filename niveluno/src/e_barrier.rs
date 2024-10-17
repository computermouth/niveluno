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
    bounds: Bounds,
    mats: [Matrix; 8],
}

struct Bounds {
    b1: Vector3,
    b2: Vector3,
    b3: Vector3,
    b4: Vector3,
    t1: Vector3,
    t2: Vector3,
    t3: Vector3,
    t4: Vector3,
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

        #[rustfmt::skip]
        let mut bounds = [
            // bottoms
            vector3_transform(Vector3::new(-entt.scale[0] / 2., 0., -entt.scale[2] / 2.), mat_r),
            vector3_transform(Vector3::new(-entt.scale[0] / 2., 0.,  entt.scale[2] / 2.), mat_r),
            vector3_transform(Vector3::new( entt.scale[0] / 2., 0., -entt.scale[2] / 2.), mat_r),
            vector3_transform(Vector3::new( entt.scale[0] / 2., 0.,  entt.scale[2] / 2.), mat_r),
            // tops
            vector3_add(vector3_transform( Vector3::new(-entt.scale[0] / 2., 0., -entt.scale[2] / 2.), mat_r), Vector3::new(0., 2., 0.)),
            vector3_add(vector3_transform( Vector3::new(-entt.scale[0] / 2., 0.,  entt.scale[2] / 2.), mat_r), Vector3::new(0., 2., 0.)),
            vector3_add(vector3_transform( Vector3::new( entt.scale[0] / 2., 0., -entt.scale[2] / 2.), mat_r), Vector3::new(0., 2., 0.)),
            vector3_add(vector3_transform( Vector3::new( entt.scale[0] / 2., 0.,  entt.scale[2] / 2.), mat_r), Vector3::new(0., 2., 0.)),
        ];

        let mut mats = [matrix_identity(); 8];
        let mat_t = raymath::matrix_translate(entt.location[0], entt.location[1], entt.location[2]);
        for (i, point) in bounds.iter_mut().enumerate() {
            let translate_mat = raymath::matrix_translate(point.x, point.y, point.z);
            let mat = raymath::matrix_multiply(mat_t, translate_mat);
            mats[i] = mat;
            // reset the points with the matrix
            // both of these work, not sure which is less work
            // todo -- perf -- test this
            // *point = raymath::vector3_transform(Vector3::new(0., 0., 0.), mat);
            *point = raymath::vector3_transform(*point, mat_t);
        }

        Self {
            base: entt.clone(),
            id,
            bounds: Bounds {
                b1: bounds[0],
                b2: bounds[1],
                b3: bounds[2],
                b4: bounds[3],
                t1: bounds[4],
                t2: bounds[5],
                t3: bounds[6],
                t4: bounds[7],
            },
            mats,
        }
    }

    pub fn update(&mut self) {
        // todo, push the barrier level surface to a queue with a distance,
        // to perform an ordered draw with the final camera params
        let pos_2d = math::world_point_to_screen_coord(
            self.base.location.into(),
            render::get_camera_pos().unwrap(),
            render::get_camera_yaw().unwrap(),
            render::get_camera_pitch().unwrap(),
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
        let t1 = [self.bounds.t1, self.bounds.t2, self.bounds.t3];
        let t2 = [self.bounds.t2, self.bounds.t3, self.bounds.t4];

        let rt = math::Ray {
            position: point,
            direction: Vector3::new(0., 1., 0.),
        };

        if !math::get_ray_collision_triangle(rt, t1[0], t1[1], t1[2]).hit
            && !math::get_ray_collision_triangle(rt, t2[0], t2[1], t2[2]).hit
        {
            return false;
        }

        let b1 = [self.bounds.b1, self.bounds.b2, self.bounds.b3];
        let b2 = [self.bounds.b2, self.bounds.b3, self.bounds.b4];

        let rb = math::Ray {
            position: point,
            direction: Vector3::new(0., -1., 0.),
        };

        if !math::get_ray_collision_triangle(rb, b1[0], b1[1], b1[2]).hit
            && !math::get_ray_collision_triangle(rb, b2[0], b2[1], b2[2]).hit
        {
            return false;
        }

        true
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
