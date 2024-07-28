use crate::e_entity::EntityInstance;
use crate::level::Entity;

pub struct Gcyl {}

impl EntityInstance for Gcyl {
    fn update(&mut self) {}
}

impl Gcyl {
    pub fn new(entt: &Entity) -> Self {
        Self {}
    }
}
