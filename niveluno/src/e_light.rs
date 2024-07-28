use crate::e_entity::EntityInstance;
use crate::level::Entity;

pub struct Light {}

impl EntityInstance for Light {
    fn update(&mut self) {}
}

impl Light {
    pub fn new(entt: &Entity) -> Self {
        Self {}
    }
}
