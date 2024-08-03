use crate::e_entity::EntityInstance;
use crate::map::Entity;

use crate::render;

pub struct Light {
    base: Entity,
}

impl EntityInstance for Light {
    fn update(&mut self) {
        render::push_light(self.base.location.into(), 1, 1, 1, 1).unwrap();
    }
    fn draw_model(&mut self) {}
}

impl Light {
    pub fn new(entt: &Entity) -> Self {
        Self { base: entt.clone() }
    }
}
