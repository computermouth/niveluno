use crate::e_entity::EntityInstance;
use crate::level::Entity;

use crate::render;

pub struct Light {
    base: Entity,
}

impl EntityInstance for Light {
    fn update(&mut self) {
        render::push_light(self.base.location.into(), 1, 123, 45, 67).unwrap();
    }
    fn draw_model(&mut self) {}
}

impl Light {
    pub fn new(entt: &Entity) -> Self {
        Self { base: entt.clone() }
    }
}
